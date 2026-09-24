//! The module that provides utility functions.

// Uses
use std::{process::Command, str::from_utf8 as str_from_utf8};

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
