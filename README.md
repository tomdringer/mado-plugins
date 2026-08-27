# mado-plugins

Community plugins for [Mado](https://github.com/tomdringer/mado), a terminal multiplexer built with Rust and Slint.

## What is a plugin?

A Mado plugin is any binary that renders to a terminal. Mado spawns it inside a sidebar panel, handles sizing the PTY, and forwards scroll and resize events. The plugin just writes ANSI output to stdout.

No Mado SDK. No special build tools. Any language works — Rust, Go, Python, a shell script.

## Installing a plugin

1. Install the plugin binary so it's on your `$PATH` (or note its absolute path).
2. Add a `[[plugins]]` entry to `~/.config/mado/config.toml`:

```toml
[[plugins]]
id      = "clock"
command = "mado-clock"

[[plugins]]
id      = "ai"
command = "mado-ai"
```

3. Restart Mado. The plugin will appear as a panel in the sidebar.

`id` is the label shown in the sidebar. `command` is the binary Mado spawns — it must be on `$PATH` or an absolute path.

## Writing a plugin

A sidebar panel plugin only needs to do three things:

1. **Render to stdout** using ANSI escape codes. Use whatever TUI library you like (ratatui, crossterm, blessed, rich, etc.) or write raw escape sequences.

2. **Read the initial size** from the `COLUMNS` and `LINES` environment variables that Mado sets before spawning.

3. **Handle `SIGWINCH`** to redraw when the user resizes the panel.

That's the entire contract.

### Environment variables

Mado sets the following before spawning a plugin:

| Variable | Description |
|----------|-------------|
| `COLUMNS` | Panel width in columns |
| `LINES` | Panel height in rows |
| `TERM` | Always `xterm-256color` |
| `MADO_PLUGIN_ID` | The `id` from the plugin's config entry |

### Minimal example (shell)

```sh
#!/usr/bin/env bash
# A plugin that just shows the current time, refreshing every second.

draw() {
    clear
    echo ""
    printf "  %s\n" "$(date '+%H:%M:%S')"
}

trap draw WINCH

while true; do
    draw
    sleep 1
done
```

Save it as `mado-clock-minimal`, make it executable, put it on your `$PATH`, and add:

```toml
[[plugins]]
id      = "clock"
command = "mado-clock-minimal"
```

### Minimal example (Rust)

See the [`template/`](template/) directory for a commented Rust starter that handles sizing and `SIGWINCH` correctly.

## Known plugins

See [`plugins.toml`](plugins.toml) for the curated list of community plugins.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).
