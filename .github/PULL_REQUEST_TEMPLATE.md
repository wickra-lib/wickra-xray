<!-- Keep it short. One logical change per PR. -->

## What

<!-- What does this change and why? -->

<!--
For a large change -- one that alters what a frame carries, adds a panel or a
dataset input, makes a performance claim, or touches more than one language --
there is a longer template with the sections a reviewer will otherwise have to
ask for: append `?template=detailed.md` to this page's URL, or copy
.github/PULL_REQUEST_TEMPLATE/detailed.md.
-->

## Checklist

- [ ] `cargo fmt --all` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` are clean
- [ ] `cargo test --workspace --all-features` and `--no-default-features` pass (parallel == sequential)
- [ ] `cargo deny check` is clean
- [ ] Tests added/updated (prefer hand-computed expectations for core changes)
- [ ] The spec stays data (a serde `XraySpec`), never Rust closures
- [ ] Binding surface mirrored across languages; golden frames regenerated if the schema changed
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
