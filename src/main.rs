use std::{os::unix::process::CommandExt, process::Command};

use hyprshell_hyprland::{data::Monitors, shared::HyprData};

/// Gets the focused monitor if it can be found
fn get_focused_monitor() -> Option<String> {
    let monitors = match Monitors::get() {
        Ok(m) => m,
        Err(_) => return None,
    };

    for m in monitors.iter() {
        if m.focused {
            return Some(m.name.clone());
        }
    }

    None
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let monitor = get_focused_monitor();

    let swayosd_args = match monitor {
        Some(monitor) => {
            let mut out = Vec::with_capacity(args.len() + 2);
            out.push("--monitor".to_string());
            out.push(monitor);
            out.extend(args);
            out
        }
        _ => args,
    };

    let _ = Command::new("swayosd-client").args(swayosd_args).exec();
}
