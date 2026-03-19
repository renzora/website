# Contributing to Renzora

Renzora Engine is open source and welcomes contributions from the community.

## Getting started

1. Fork the [engine repo](https://github.com/renzora/engine)
2. Clone your fork and create a branch
3. Make your changes
4. Submit a pull request

```bash
git clone https://github.com/YOUR_USERNAME/engine.git
cd engine
git checkout -b my-feature
# make changes...
git commit -m "Add my feature"
git push origin my-feature
```

## Code style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Follow existing patterns in the codebase

## What to contribute

- **Bug fixes** — check the [issue tracker](https://github.com/renzora/engine/issues)
- **Documentation** — improve or add docs (edit files in the `docs/` folder of the website repo)
- **Editor panels** — implement the `EditorPanel` trait
- **Scripting functions** — add new APIs for Lua/Rhai
- **Export targets** — improve platform export support
- **Post-processing effects** — use the `#[renzora_macros::post_process]` macro

## Community

- [Discord](https://discord.gg/9UHUGUyDJv) — chat with other contributors
- [Forum](https://renzora.com/forum) — longer-form discussion
- [YouTube](https://youtube.com/@renzoragame) — video updates
