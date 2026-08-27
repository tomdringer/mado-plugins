# Contributing a plugin

Submitting your plugin adds it to the curated list that Mado users can browse. The bar is low — it needs to work, be safe, and have a public repo.

## Steps

1. **Build your plugin.** It must satisfy the plugin contract described in [README.md](README.md): render to stdout, handle `SIGWINCH`, read `COLUMNS`/`LINES`.

2. **Publish it.** Push the source to a public GitHub (or other) repo. Include a README explaining what it does and how to install it.

3. **Fork this repo** and add an entry to `plugins.toml`:

```toml
[[plugins]]
name        = "mado-yourplugin"
description = "One sentence describing what it does"
author      = "your-github-username"
repo        = "https://github.com/you/mado-yourplugin"
kind        = "sidebar-panel"
```

4. **Open a pull request** against `main`. The PR description should include:
   - What the plugin does
   - How to install it (e.g. `cargo install mado-yourplugin` or a release binary link)
   - A screenshot or short demo if you have one

## Review criteria

- Does the plugin repo exist and is it public?
- Does it actually run as a Mado sidebar panel?
- Is the description accurate and readable?
- Is the `kind` field correct? (Currently only `sidebar-panel` is supported.)

## Guidelines

- **One plugin per PR.**
- **Name your plugin `mado-<something>`** so it's easy to identify in search results and on `$PATH`.
- **No malicious or deceptive plugins.** Reviewers will look at the source.
- **Keep the description to one sentence.** Users can read your repo README for details.

## Plugin kinds

| Kind | Description |
|------|-------------|
| `sidebar-panel` | Spawned in an expandable panel in the Mado sidebar |

More kinds may be added as the plugin system evolves.
