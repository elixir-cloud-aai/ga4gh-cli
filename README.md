# GA4GH SDK
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)

## Synopsis

A Generic SDK and CLI for GA4GH API services.

## Basic usage
TBA

## Installation
TBA

## Building

**Developer note:** If you want to update any of the [OpenAPI](https://www.openapis.org/) specifications supported by this tool or extend it to support any other OpenAPI-backed services (e.g., not currently supported GA4GH API-based services or your own services), please refer to our [documentation for auto-generating OpenAPI models](docs/autogenerate_OAI_models.md).

To build the project:
```
cargo build
```

## Testing

Before running the tests, you need to install Funnel, a task execution system that is compatible with the GA4GH TES API. Follow the instructions in the Funnel Developer's Guide to install Funnel: https://ohsu-comp-bio.github.io/funnel/docs/development/developers/. Note that the link may change over time, so refer to the official Funnel repository or website for the most up-to-date information.

Once you have installed Funnel, you can run the tests. This will automatically run Funnel as well:

```
bash ./run-tests.sh
```
or, you can run using:
```
cargo nextest run
```
For checking the unit coverage, you can run:
```
cargo llvm-cov nextest
```

To test the CI/CD workflow locally, install `act` and run the following command:
```
act -j build --container-architecture linux/amd64 -P ubuntu-latest=ubuntu:24.04 --reuse
# Note: The specified Ubuntu version (24.04) may change in the future. 
# Please check for the latest version when running this command.
```

## Versioning
The project adopts the [semver] scheme for versioning.
Currently the software is in a pre-release stage, so changes to the API,
including breaking changes, may occur at any time without further notice.

## Contributing

This project is a community effort and lives off your contributions, be it in
the form of bug reports, feature requests, discussions, ideas, fixes, or other
code changes. Please read [these guidelines][docs-contributing] if you want to
contribute. And please mind the [code of conduct][docs-coc] for all
interactions with the community.

## License

This project is covered by the [Apache License 2.0](badge-url-license) also
[shipped with this repository][license].

## Contact
The project is maintained by [ELIXIR Cloud & AAI][elixir-cloud-aai], a Driver
Project of the [Global Alliance for Genomics and Health (GA4GH)][ga4gh], under
the umbrella of the [ELIXIR] [Compute Platform][elixir-compute].

To get in touch with us, please use one of the following routes:

- For filing bug reports, feature requests or other code-related issues, please
  make use of the project's [issue tracker][issue-tracker].
- For private/personal issues, more involved communication, or if you would like
  to join our team as a regular contributor, you can either join our
  [chat board][badge-chat-url] or [email] the community leaders.


[![logo-elixir]][elixir] [![logo-elixir-cloud-aai]][elixir-cloud-aai]

[badge-chat-url]: https://join.slack.com/t/elixir-cloud/shared_invite/enQtNzA3NTQ5Mzg2NjQ3LTZjZGI1OGQ5ZTRiOTRkY2ExMGUxNmQyODAxMDdjM2EyZDQ1YWM0ZGFjOTJhNzg5NjE0YmJiZTZhZDVhOWE4MWM
[badge-license-url]: http://www.apache.org/licenses/LICENSE-2.0
[code-of-conduct]: https://elixir-cloud-aai.github.io/about/code-of-conduct/
[contributing]: https://elixir-cloud-aai.github.io/guides/guide-contributor/
[elixir]: https://elixir-europe.org/
[elixir-cloud-aai]: https://elixir-cloud.dcc.sib.swiss/
[elixir-compute]: https://elixir-europe.org/platforms/compute
[email]: mailto:cloud-service@elixir-europe.org
[ga4gh]: https://ga4gh.org/
[license]: LICENSE
[logo-elixir]: ./images/logo-elixir.svg
[logo-elixir-cloud-aai]: ./images/logo-elixir-cloud-aai.svg
[semver]: https://semver.org/
