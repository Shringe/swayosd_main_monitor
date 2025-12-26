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
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    let monitor = get_focused_monitor();

    if let Some(m) = monitor {
        args.push("--monitor".to_string());
        args.push(m);
    }

    let _ = Command::new("swayosd-client").args(args).exec();
}
