# Stacked

Stacked is the CYPHES coding environment for learning and practicing multi-agent software collaboration.

Phase 1 starts with a fork of [Claurst](https://github.com/Kuberwastaken/claurst), a GPL-3.0 Rust coding-agent runtime, and rebrands the user experience around CYPHES. Phase 2 introduces the Stacked v0.1 workflow: a drafter, reviewer, and synthesizer working through explicit approvals, contracts, and receipts.

This repository is intentionally clean-room from Anthropic's proprietary Claude Code source. See [docs/PROVENANCE.md](docs/PROVENANCE.md) and [docs/adr/0001-phase-1-agent-runtime.md](docs/adr/0001-phase-1-agent-runtime.md).

## Product Direction

Stacked is built to make agent collaboration inspectable:

- A coding agent runtime that can operate through a CYPHES-branded terminal and IDE experience.
- A permission-first interface for tool use, edits, shell commands, and external context.
- Receipts for meaningful agent actions, including provider, model, prompt hash, artifact hash, tool events, approval events, and final patch hash.
- A staged path toward CYPHES Agent Transfer Protocol training workflows.

The visual language follows the CYPHES system: near-black surfaces, precise grid structure, cyan/green execution accents, restrained orange risk accents, glassy panels, dense technical typography, and receipt-oriented audit surfaces.

## Phase 1 Scope

Phase 1 is not the full Stacked multi-agent product. It is the foundation:

- Fork Claurst and preserve GPL-3.0 obligations.
- Rebrand the visible product surface to Stacked and CYPHES.
- Replace Claurst-specific product copy, domains, package metadata, and screenshots over time.
- Keep the agent runtime stable while the UI/UX changes are made.
- Add CYPHES approval, receipt, and provenance language before broader publication.
- Prepare GitHub issues, docs, release hygiene, and security policy for a public project.

## Phase 2 Direction

Stacked v0.1 will add a three-stage collaboration loop:

1. Drafter creates a plan and proposed patch.
2. Reviewer independently critiques the draft against the contract.
3. Synthesizer resolves review findings into final output.

The user approves each stage. The system records structured receipts for the work.

## Repository Status

This repository is in Phase 1 setup. The upstream runtime still contains Claurst naming and assets in code, docs, workflows, package metadata, and screenshots. Those are tracked rebrand tasks, not final product state.

## Development

The Rust workspace lives under [src-rust](src-rust).

```bash
cd src-rust
cargo check --workspace
cargo fmt --all
cargo test --workspace
```

For UI/TUI changes, verify both interactive and headless paths where possible:

```bash
cd src-rust
cargo run -- --print "explain this repository"
```

## Governance

- [Roadmap](docs/ROADMAP.md)
- [Provenance](docs/PROVENANCE.md)
- [Security](SECURITY.md)
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Notice](NOTICE.md)

## License

Stacked is a fork of Claurst and is distributed under GPL-3.0. See [LICENSE](LICENSE), [LICENSE.md](LICENSE.md), and [NOTICE.md](NOTICE.md).
