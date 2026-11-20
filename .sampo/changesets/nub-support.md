---
cargo/sampo: minor
cargo/sampo-core: minor
cargo/sampo-github-action: minor
---

In JavaScript/TypeScript (npm) projects, added support for the nub package manager, detected from `nub.lock`, `packageManager`, or `devEngines.packageManager`. `sampo release` refreshes `nub.lock`, `sampo publish` publishes with `nub publish`, and `packages.npm_stage` stages nub packages through npm.
