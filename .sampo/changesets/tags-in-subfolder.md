---
cargo/sampo: patch
cargo/sampo-core: patch
cargo/sampo-github-action: patch
---

Fixed `sampo publish` creating no tags, and saying nothing, when the workspace is a subfolder of its git repository or `.git` is a file (a worktree or a submodule).
