use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Next,
    Previous,
    Help,
}

fn parse_action(args: &[String]) -> Result<Action, String> {
    match args {
        [] => Ok(Action::Help),
        [arg] if arg == "next" => Ok(Action::Next),
        [arg] if arg == "prev" || arg == "previous" => Ok(Action::Previous),
        [arg] if arg == "help" || arg == "--help" || arg == "-h" => Ok(Action::Help),
        _ => Err("Unknown command. Use next, prev, or help.".to_string()),
    }
}

fn print_help() {
    println!("Lyco Desktop Switcher (prototype)");
    println!("Usage: lyco-desktop-switcher [next|prev|help]");
    println!("Uses Windows Win+Ctrl+Left/Right shortcuts. Requires an interactive user session.");
}

#[cfg(windows)]
fn switch_desktop(action: Action) {
    use std::thread::sleep;
    use std::time::Duration;

    #[link(name = "user32")]
    extern "system" {
        fn keybd_event(vk: u8, scan: u8, flags: u32, extra_info: usize);
    }

    const KEYUP: u32 = 0x0002;
    const WIN: u8 = 0x5B;
    const CTRL: u8 = 0x11;
    const LEFT: u8 = 0x25;
    const RIGHT: u8 = 0x27;

    let arrow = match action {
        Action::Next => RIGHT,
        Action::Previous => LEFT,
        Action::Help => return,
    };
    unsafe {
        keybd_event(WIN, 0, 0, 0);
        keybd_event(CTRL, 0, 0, 0);
        keybd_event(arrow, 0, 0, 0);
        sleep(Duration::from_millis(35));
        keybd_event(arrow, 0, KEYUP, 0);
        keybd_event(CTRL, 0, KEYUP, 0);
        keybd_event(WIN, 0, KEYUP, 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_action(&args) {
        Ok(Action::Help) => print_help(),
        Ok(action) => {
            #[cfg(windows)]
            switch_desktop(action);
            #[cfg(not(windows))]
            {
                let _ = action;
                eprintln!("Desktop switching is only available on Windows.");
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            print_help();
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_commands() {
        assert_eq!(parse_action(&[]).unwrap(), Action::Help);
        assert_eq!(parse_action(&["next".into()]).unwrap(), Action::Next);
        assert_eq!(parse_action(&["prev".into()]).unwrap(), Action::Previous);
        assert!(parse_action(&["invalid".into()]).is_err());
    }
}
