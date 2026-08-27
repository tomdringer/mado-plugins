/// Mado plugin template — minimal Rust sidebar panel.
///
/// This renders a static message and redraws on resize.
/// Use it as a starting point for your own plugin.
///
/// Build:   cargo build --release
/// Install: cp target/release/mado-template ~/.local/bin/
/// Config:  add to ~/.config/mado/config.toml:
///
///   [[plugins]]
///   id      = "template"
///   command = "mado-template"

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn cols() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(80)
}

fn rows() -> usize {
    std::env::var("LINES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(24)
}

fn draw(cols: usize, _rows: usize) {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Clear screen and move cursor to top-left.
    write!(out, "\x1b[2J\x1b[H").unwrap();

    // Centered message.
    let msg = "mado plugin template";
    let pad = cols.saturating_sub(msg.len()) / 2;
    writeln!(out, "\n{}{}", " ".repeat(pad), msg).unwrap();

    out.flush().unwrap();
}

fn main() {
    // SIGWINCH signals a terminal resize. Set a flag so the main loop redraws.
    let resized = Arc::new(AtomicBool::new(false));
    {
        let resized = Arc::clone(&resized);
        // SAFETY: signal handler only sets an atomic bool — safe.
        unsafe {
            libc::signal(libc::SIGWINCH, handle_sigwinch as libc::sighandler_t);
        }
        // Store the Arc so the closure lives long enough.
        // (In a real plugin you'd use signal-hook or nix crate instead.)
        let _ = resized;
    }

    draw(cols(), rows());

    loop {
        if RESIZED.load(Ordering::Relaxed) {
            RESIZED.store(false, Ordering::Relaxed);
            draw(cols(), rows());
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

static RESIZED: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigwinch(_: libc::c_int) {
    RESIZED.store(true, Ordering::Relaxed);
}
