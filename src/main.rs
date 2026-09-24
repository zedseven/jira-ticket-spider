//! A utility for crawling Jira tickets based on their links, then creating a
//! visual map of how they relate to each other. Outputs in PlantUML format on
//! `stdout`.

// Linting Rules
#![warn(
	clippy::complexity,
	clippy::correctness,
	clippy::pedantic,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::clone_on_ref_ptr,
	clippy::dbg_macro,
	clippy::decimal_literal_representation,
	clippy::exit,
	clippy::filetype_is_file,
	clippy::if_then_some_else_none,
	clippy::non_ascii_literal,
	clippy::self_named_module_files,
	clippy::str_to_string,
	clippy::undocumented_unsafe_blocks,
	clippy::wildcard_enum_match_arm
)]
#![allow(
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::doc_markdown,
	clippy::module_name_repetitions,
	clippy::similar_names,
	clippy::too_many_lines,
	clippy::unnecessary_wraps,
	dead_code,
	unused_macros
)]

// Modules
mod cli;
mod parsing;
mod util;

// Uses
use std::{
	collections::{HashMap, HashSet},
	process::Command,
};

use anyhow::{Context, Result as AnyhowResult};
use serde_json::from_str as parse_from_json_str;

use crate::{
	cli::build_cli,
	parsing::{JiraTicketDetails, JiraTicketDetailsIssueLink},
	util::run_command,
};

#[derive(Debug)]
struct JiraTicket {
	summary: String,
}

impl From<&JiraTicketDetails> for JiraTicket {
	fn from(jira_ticket_details: &JiraTicketDetails) -> Self {
		Self {
			summary: jira_ticket_details.fields.summary.clone(),
		}
	}
}

/// Outwards from `a` to `b` (`a` is above `b` on the resulting graph, with the
/// arrows heading downwards)
#[derive(Debug, PartialEq, Eq, Hash)]
struct JiraTicketRelationship {
	a:      String,
	b:      String,
	r#type: String,
}

// Entry Point
fn main() -> AnyhowResult<()> {
	let cli_definition = build_cli();
	let matches = cli_definition.get_matches();

	let starting_jira_tickets = matches
		.get_many::<String>("starting-jira-ticket")
		.expect("Clap ensures at least one argument is provided")
		.collect::<Vec<_>>();
	let follow_link_types = matches
		.get_many::<String>("follow-link-types")
		.expect("Clap provides a default value")
		.collect::<Vec<_>>();

	let follow_link_types_ref = follow_link_types
		.iter()
		.map(|s| s.trim())
		.collect::<Vec<_>>();

	let mut jira_tickets = HashMap::new();
	let mut relationships = HashSet::new();

	for starting_jira_ticket in starting_jira_tickets {
		visit_jira_ticket(
			&mut jira_tickets,
			&mut relationships,
			follow_link_types_ref.as_slice(),
			starting_jira_ticket.as_str(),
		)?;
	}

	dbg!(&jira_tickets);
	dbg!(&relationships);

	Ok(())
}

fn visit_jira_ticket(
	jira_tickets: &mut HashMap<String, JiraTicket>,
	relationships: &mut HashSet<JiraTicketRelationship>,
	follow_link_types: &[&str],
	jira_ticket: &str,
) -> AnyhowResult<()> {
	// Visit the ticket if it's new
	let jira_ticket = jira_ticket.trim();

	if jira_tickets.contains_key(jira_ticket) {
		return Ok(());
	}

	eprintln!("Visiting {jira_ticket}...");

	let jira_ticket_details_json = run_jira_ticket_fetch(jira_ticket)?;
	let jira_ticket_details =
		parse_from_json_str::<JiraTicketDetails>(jira_ticket_details_json.as_str())
			.with_context(|| "deserialising from JSON failed")?;

	// Store the ticket in the visited set
	jira_tickets.insert(
		jira_ticket.to_owned(),
		JiraTicket::from(&jira_ticket_details),
	);

	// Store all the links
	let mut jira_tickets_to_visit =
		Vec::with_capacity(jira_ticket_details.fields.issue_links.len());
	for issue_link in &jira_ticket_details.fields.issue_links {
		if let Some(outward_issue) = &issue_link.outward_issue {
			relationships.insert(JiraTicketRelationship {
				a:      jira_ticket.to_owned(),
				b:      outward_issue.key.clone(),
				r#type: issue_link.r#type.outward.clone(),
			});

			if should_visit_link(follow_link_types, issue_link) {
				jira_tickets_to_visit.push(outward_issue.key.as_str());
			}
		} else if let Some(inward_issue) = &issue_link.inward_issue {
			relationships.insert(JiraTicketRelationship {
				a:      inward_issue.key.clone(),
				b:      jira_ticket.to_owned(),
				r#type: issue_link.r#type.outward.clone(),
			});

			if should_visit_link(follow_link_types, issue_link) {
				jira_tickets_to_visit.push(inward_issue.key.as_str());
			}
		}
	}

	// Crawl all discovered Jira tickets
	for jira_ticket_to_visit in jira_tickets_to_visit {
		visit_jira_ticket(
			jira_tickets,
			relationships,
			follow_link_types,
			jira_ticket_to_visit,
		)?;
	}

	Ok(())
}

fn should_visit_link(follow_link_types: &[&str], issue_link: &JiraTicketDetailsIssueLink) -> bool {
	follow_link_types.contains(&issue_link.r#type.outward.trim())
		|| follow_link_types.contains(&issue_link.r#type.inward.trim())
}

fn run_jira_ticket_fetch(jira_ticket: &str) -> AnyhowResult<String> {
	let mut jira_cli_command = Command::new("jira");
	jira_cli_command
		.args(["issue", "view", "--raw"])
		.arg(jira_ticket);

	// Run the command
	run_command(jira_cli_command)
		.with_context(|| format!("unable to get ticket details for {jira_ticket}"))
}
