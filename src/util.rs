//! The module that provides utility functions.

// Uses
use std::{cmp::Ordering, process::Command, str::from_utf8 as str_from_utf8};

use anyhow::{Context, Result as AnyhowResult, anyhow};

/// Runs a provided command and returns the stdout in UTF-8.
pub fn run_command(mut command: Command) -> AnyhowResult<String> {
	// Run the command
	let command_result = command
		.output()
		.with_context(|| "unable to run the command")?;
	if !command_result.status.success() {
		return Err(anyhow!(
			"command failed: {:?}",
			command_result.status.code()
		));
	}

	// Convert the command output into a usable string of UTF-8
	str_from_utf8(&command_result.stdout)
		.with_context(|| "unable to parse command output as UTF-8")
		.map(ToOwned::to_owned)
}

/// Builds a `Vec` from an iterator that returns references, then sorts it with
/// `compare`.
///
/// This is intended to be used for creating a cheap sorted reference list from
/// a slice, [`HashSet`], or [`HashMap`].
///
/// Internally, it uses `Vec::sort_by`, instead of `Vec::sort_by_key`, because
/// we can't use `Vec::sort_by_key` without cloning the values yet:
/// https://github.com/rust-lang/rust/issues/34162
pub fn sorted_vec_from_iterator<I, V, F>(iterator: I, compare: F) -> Vec<V>
where
	I: Iterator<Item = V>,
	F: FnMut(&V, &V) -> Ordering,
{
	let mut result_vec = iterator.collect::<Vec<_>>();

	result_vec.sort_by(compare);

	result_vec
}

/// Takes a Jira ticket and returns it in a format that can be used as a sorting
/// key to avoid an ASCII sort.
pub fn sortable_jira_ticket(jira_ticket: &str) -> (&str, u32) {
	// Split on the hyphen
	let (project, issue) = jira_ticket.split_once('-').expect(
		"all Jira tickets should have 1 hyphen separating the project from the issue number",
	);

	// Parse the issue number as an integer
	let issue_num = issue
		.parse::<u32>()
		.expect("all issue numbers should be numeric");

	// Return the pair, so they can be used as a sorting key
	(project, issue_num)
}
