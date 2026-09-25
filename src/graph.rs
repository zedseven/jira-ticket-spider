// Uses
use std::collections::{HashMap, HashSet};

use crate::{JiraTicket, JiraTicketRelationship, should_visit_link_type};

pub fn print_plantuml(
	jira_tickets: &HashMap<String, JiraTicket>,
	relationships: &HashSet<JiraTicketRelationship>,
	url_prefix: &str,
	follow_link_types: &[&str],
) {
	println!("@startuml");
	println!();
	println!("skinparam maxMessageSize 200");
	println!("skinparam wrapWidth 150");
	println!("skinparam componentStyle rectangle");
	println!();
	println!("' Tickets");

	for (key, details) in jira_tickets {
		println!(
			"component \"[[{url_prefix}{key} {key}]]: {}\" as {}",
			details.summary,
			escape_key(key)
		);
	}

	println!();
	println!("' Relationships");

	for relationship in relationships {
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
