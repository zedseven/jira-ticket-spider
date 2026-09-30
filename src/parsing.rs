// Uses
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetails<'a> {
	pub key:    &'a str,
	#[serde(borrow)]
	pub fields: JiraTicketDetailsFields<'a>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsFields<'a> {
	pub summary:     &'a str,
	#[serde(borrow, rename = "issuelinks")]
	pub issue_links: Vec<JiraTicketDetailsIssueLink<'a>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLink<'a> {
	#[serde(borrow)]
	pub inward_issue:  Option<JiraTicketDetailsIssueLinkIssue<'a>>,
	#[serde(borrow)]
	pub outward_issue: Option<JiraTicketDetailsIssueLinkIssue<'a>>,
	#[serde(borrow)]
	pub r#type:        JiraTicketDetailsIssueLinkType<'a>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLinkIssue<'a> {
	pub key: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicketDetailsIssueLinkType<'a> {
	pub inward:  &'a str,
	pub outward: &'a str,
}
