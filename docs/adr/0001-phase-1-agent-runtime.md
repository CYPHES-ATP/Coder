# ADR 0001: Phase 1 Agent Runtime

## Status

Accepted.

## Date

2026-05-25

## Context

CYPHES is building Stacked, a free and open-source coding environment that teaches multi-agent collaboration through explicit contracts, approvals, reviews, synthesis, and receipts.

Phase 1 needs a working coding-agent runtime so the project can focus on CYPHES UI/UX, permission surfaces, provenance, and publication readiness before building the full multi-agent Stacked v0.1 workflow.

The evaluated options were:

- Fork `Kuberwastaken/claurst`.
- Use `yasasbanukaofficial/claude-code`.
- Use `vellum-ai` projects as a base.

## Decision

Fork `Kuberwastaken/claurst` for Phase 1.

Do not use `yasasbanukaofficial/claude-code` or any repository that represents itself as leaked Anthropic Claude Code source.

Use `vellum-ai` projects only as architectural inspiration for memory, trust, credentials, sandboxing, and assistant lifecycle concepts.

## Rationale

Claurst is the closest technical base for Phase 1:

- Rust coding-agent runtime.
- Terminal UI.
- Multi-provider support.
- Tool permissions.
- Plugin-oriented architecture.
- ACP support for editor integration.
- GPL-3.0 open-source licensing.

The leaked Claude Code mirror is rejected because it creates unacceptable provenance, licensing, publication, and trust risk.

Vellum is not selected as the base because it is a broader personal assistant platform rather than a focused coding-agent IDE/runtime. Its security and memory architecture can still inform later CYPHES design.

## Consequences

- Stacked Phase 1 inherits GPL-3.0 obligations from Claurst.
- Upstream Claurst attribution must remain visible in `NOTICE.md` and license files.
- CYPHES UI/UX work should be tracked as a rebrand and product-layer migration, not as a clean rewrite.
- Public release requires a provenance and workflow audit.
- Future proprietary or permissive components should be separated carefully and reviewed before distribution.

## Follow-Up Work

- Audit and update upstream workflows before enabling public releases.
- Replace Claurst public copy, domains, screenshots, and package metadata.
- Add CYPHES permission and receipt surfaces.
- Prepare Stacked v0.1 multi-agent ADR before Phase 2 implementation.
