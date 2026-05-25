# Provenance

Stacked is a CYPHES project forked from Claurst.

## Source Base

- Upstream repository: https://github.com/Kuberwastaken/claurst
- Local project name: Stacked
- Initial phase: CYPHES UI/UX rebrand and governance setup
- License inherited from upstream: GPL-3.0

The Phase 1 source base was selected because Claurst is an open-source Rust implementation of a terminal coding agent with provider support, tools, permissions, plugins, and ACP integration.

## Excluded Sources

This repository must not use:

- Leaked Anthropic Claude Code source.
- Mirrors or backups that describe themselves as leaked Claude Code source.
- Proprietary source maps or recovered proprietary source.
- Generated rewrites that are derived from pasted proprietary source.

Educational or internal intent does not change this rule. The project is being prepared for publication, so clean provenance is a product requirement.

## Allowed Inputs

Contributors may use:

- The existing GPL-3.0 Claurst source in this fork.
- Public Claurst documentation and specs included in the upstream repository.
- Public API documentation from model providers.
- Original implementation work.
- Clean-room behavior observations.
- CYPHES design files and product direction supplied by CYPHES.

## Contributor Certification

By contributing, you certify that:

- You have the right to submit the contribution.
- Your contribution is compatible with this repository's license.
- Your contribution does not include leaked or proprietary source code.
- Any third-party code, assets, or prompts are clearly attributed and license-compatible.

## Audit Notes

Phase 1 setup keeps upstream implementation files intact while adding project governance and CYPHES direction. Future rebrand work should document material source, license, and attribution changes in this file or in linked ADRs.
