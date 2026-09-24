// Uses
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetails {
	pub key:    String,
	pub fields: JiraTicketDetailsFields,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsFields {
	pub summary:     String,
	#[serde(rename = "issuelinks")]
	pub issue_links: Vec<JiraTicketDetailsIssueLink>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLink {
	pub inward_issue:  Option<JiraTicketDetailsIssueLinkIssue>,
	pub outward_issue: Option<JiraTicketDetailsIssueLinkIssue>,
	pub r#type:        JiraTicketDetailsIssueLinkType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLinkIssue {
	pub key: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLinkType {
	pub inward:  String,
	pub outward: String,
}
