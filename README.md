# `jira-ticket-spider`

[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

A utility for crawling Jira tickets based on their links, then creating a visual map of how they relate to each other. Outputs in PlantUML format on `stdout`.

It uses [the unofficial `jira` CLI](https://github.com/ankitpokhrel/jira-cli) to fetch Jira information, since that tool
takes care of all authentication, which makes this tool simpler.

Feel free to [open an issue](https://github.com/zedseven/jira-ticket-spider/issues/new) or submit a PR if you have something to add.

## Project License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in *jira-ticket-spider* by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
