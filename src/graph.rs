// Uses
use std::collections::{HashMap, HashSet};

use chrono::Utc;

use crate::{
	JiraTicket,
	JiraTicketRelationship,
	should_visit_link_type,
	util::{sortable_jira_ticket, sorted_vec_from_iterator},
};

pub fn print_plantuml(
	jira_tickets: &HashMap<String, JiraTicket>,
	relationships: &HashSet<JiraTicketRelationship>,
	url_prefix: Option<&str>,
	follow_link_types: &[&str],
	starting_jira_tickets: &[&str],
) {
	let jira_tickets_sorted = sorted_vec_from_iterator(jira_tickets.iter(), |(x, _), (y, _)| {
		let x_clean = sortable_jira_ticket(x.as_str());
		let y_clean = sortable_jira_ticket(y.as_str());

		x_clean.cmp(&y_clean)
	});
	let relationships_sorted = sorted_vec_from_iterator(relationships.iter(), |x, y| {
		let x_clean = sortable_jira_ticket(x.a.as_str());
		let y_clean = sortable_jira_ticket(y.a.as_str());

		x_clean.cmp(&y_clean).then_with(|| {
			let x_clean = sortable_jira_ticket(x.b.as_str());
			let y_clean = sortable_jira_ticket(y.b.as_str());

			x_clean.cmp(&y_clean)
		})
	});

	println!("@startuml");
	println!();
	println!("skinparam maxMessageSize 200");
	println!("skinparam wrapWidth 150");
	println!("skinparam componentStyle rectangle");
	println!();

	println!("' Metadata");
	println!("' Generated: {}", Utc::now().format("%+"));

	println!("' Starting Jira Tickets:");
	for starting_jira_ticket in starting_jira_tickets {
		println!("' - {starting_jira_ticket}");
	}

	println!("' Follow Link Types:");
	for follow_link_type in follow_link_types {
		println!("' - \"{follow_link_type}\"");
	}

	println!();

	println!("' Tickets");

	for (key, details) in jira_tickets_sorted {
		if let Some(url_prefix) = url_prefix {
			println!(
				"component \"[[{url_prefix}{key} {key}]]: {}\" as {}",
				details.summary,
				escape_key(key)
			);
		} else {
			println!(
				"component \"{key}: {}\" as {}",
				details.summary,
				escape_key(key)
			);
		}
	}

	println!();
	println!("' Relationships");

	for relationship in relationships_sorted {
		let arrow_type = if should_visit_link_type(follow_link_types, relationship.r#type.as_str())
		{
			"-->"
		} else {
			"..>"
		};

		println!(
			"{} {arrow_type} {}: {}",
			escape_key(relationship.a.as_str()),
			escape_key(relationship.b.as_str()),
			relationship.r#type
		);
	}

	println!();
	println!("@enduml");
}

fn escape_key(key: &str) -> String {
	key.replace('-', "_")
}
