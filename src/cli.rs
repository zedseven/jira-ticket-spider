//! Provides the CLI for the program.

// Uses
use clap::{Arg, ArgAction, Command, builder::NonEmptyStringValueParser, value_parser};

// Constants
const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

/// Builds the command-line interface.
pub fn build_cli() -> Command {
	Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.help_template(HELP_TEMPLATE)
		.arg_required_else_help(true)
		.help_expected(true)
		.arg(
			Arg::new("starting-jira-ticket")
				.action(ArgAction::Append)
				.value_name("JIRA_TICKET")
				.required(true)
				.help(
					"The ticket to start crawling from.\nMultiple tickets can be provided, \
					 separated by spaces, or this argument can be provided multiple times.",
				)
				.value_parser(NonEmptyStringValueParser::new()),
		)
		.arg(
			Arg::new("url-prefix")
				.short('u')
				.long("url-prefix")
				.visible_alias("jira-url-prefix")
				.num_args(1)
				.action(ArgAction::Set)
				.value_name("PREFIX")
				.help(
					"The prefix to apply to Jira tickets in the output to turn each ticket into a \
					 full URL.",
				),
		)
		.arg(
			Arg::new("follow-link-types")
				.short('t')
				.long("follow-link-types")
				.visible_short_alias('v')
				.visible_alias("visit-link-types")
				.visible_alias("follow-types")
				.visible_alias("visit-types")
				.num_args(1)
				.default_values(["blocks", "has to be done before", "is depended on by"])
				.action(ArgAction::Append)
				.value_name("LINK_TYPE")
				.help(
					"The issue link types to follow when crawling. Inward and outward issue link \
					 names are checked against these values.",
				)
				.value_parser(NonEmptyStringValueParser::new()),
		)
		.arg(
			Arg::new("status-colours")
				.short('s')
				.long("status-colours")
				.visible_alias("status-colors")
				.num_args(0..=1)
				.default_value("false")
				.default_missing_value("true")
				.action(ArgAction::Set)
				.value_name("TRUE/FALSE")
				.value_parser(value_parser!(bool))
				.help("Colour-code tickets by their statuses."),
		)
		.arg(
			Arg::new("dark-mode")
				.short('d')
				.long("dark-mode")
				.visible_alias("dark")
				.num_args(0..=1)
				.default_value("false")
				.default_missing_value("true")
				.action(ArgAction::Set)
				.value_name("TRUE/FALSE")
				.value_parser(value_parser!(bool))
				.requires("status-colours")
				.help("Use dark mode colours for status colour-coding."),
		)
}
