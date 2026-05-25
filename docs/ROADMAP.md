# Roadmap

This roadmap tracks Stacked as a CYPHES-branded fork of Claurst.

## Phase 1.0: Foundation

- Establish `cyphes/stacked` repository.
- Add license, notice, provenance, security, contribution, conduct, issue, and PR templates.
- Record the runtime decision in an ADR.
- Preserve upstream Claurst attribution and GPL-3.0 obligations.
- Audit existing workflows before enabling public releases.

## Phase 1.1: CYPHES UI Shell

- Replace top-level README, product pages, screenshots, and public copy.
- Define Stacked command language, project tone, and visual tokens.
- Replace visible Claurst naming in TUI surfaces.
- Replace or remove upstream mascot-led assets with CYPHES execution/receipt surfaces.
- Update GitHub Pages output or disable Pages until CYPHES assets are ready.

## Phase 1.2: Runtime Rebrand

- Rename package metadata where legally and technically appropriate.
- Update installer scripts and release artifact names.
- Add compatibility aliases for old commands if needed during transition.
- Update provider setup docs for Stacked.
- Audit telemetry, share, gist, auth, and external-service flows before publication.

## Phase 1.3: Permission And Receipt UX

- Add CYPHES-styled permission prompts for tools, file edits, shell commands, and network access.
- Add local receipt log schema for agent actions.
- Show approval history in the UI.
- Preserve secrets by design: hash or redact sensitive content in receipts.

## Phase 1.4: Internal Alpha

- Ship `v0.1.0-alpha.1` internally.
- Test on macOS, Linux, and Windows.
- Validate install, provider auth, project indexing, edits, tool permissions, and ACP mode.
- Produce screenshots and a short internal demo.
- Complete legal/provenance review before public release.

## Phase 2.0: Stacked v0.1

- Add three explicit agent roles: drafter, reviewer, synthesizer.
- Support separate providers and models per role.
- Add stage approval gates.
- Add structured review findings.
- Add contract and receipt records for each stage.
- Export a complete CYPHES Agent Transfer Protocol training receipt for a coding task.

## Later

- IDE shell around the runtime.
- Visual receipt inspector.
- Model/provider comparison harness.
- Multi-agent benchmark tasks.
- Public plugin/skill surface.
- Optional hosted coordination layer.
