# FleetPulse

FleetPulse is a hands-on learning project that grows from a small Rust URL
checker into a cloud-native uptime-monitoring service running on Kubernetes.

Status: Milestone 1 — single-check Rust CLI.

## Current scope

The first version checks one HTTP or HTTPS URL and reports the outcome. Later
milestones will add concurrent checks, an API, PostgreSQL, containers, local
Kubernetes, distributed workers, and a deployment to one cloud provider.

Initial non-goals:

- Web UI
- Authentication or multiple users
- Email, SMS, or chat alerts
- Billing
- Multi-region or multi-cloud deployment
- Kafka, a service mesh, or custom Kubernetes operators

## Current data flow

```text
CLI argument -> FleetPulse checker -> HTTP target
                       |
                       +-> result printed to stdout or stderr
```

## Development workflow

Prerequisites:

- A stable Rust toolchain with Cargo, rustfmt, and Clippy

Build the project:

```bash
cargo build
```

Check formatting:

```bash
cargo fmt --check
```

Run Clippy and treat warnings as errors:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run the tests:

```bash
cargo test
```

Run the current CLI:

```bash
cargo run -- https://example.com
```

## Milestone 1 acceptance criteria

- [ ] Accept exactly one URL and a configurable timeout.
- [ ] Produce a result containing the URL, timestamp, HTTP status when
      available, latency, and a typed outcome.
- [ ] Define and document what FleetPulse considers healthy.
- [ ] Treat malformed URLs, timeouts, and DNS or connection failures as
      expected errors rather than panics.
- [ ] Keep HTTP error responses distinct from transport failures.
- [ ] Keep output outside the HTTP-checking function.
- [ ] Cover success and representative failure paths with deterministic tests
      that do not depend on public websites.
- [ ] Pass formatting, Clippy, and test checks.
- [ ] Be explainable in terms of the ownership of each major value.

The detailed learning roadmap and mentoring agreement are maintained in local
continuity files that are intentionally excluded from Git.
