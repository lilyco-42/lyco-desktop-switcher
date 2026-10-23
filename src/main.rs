use serde_json::{json, Value};
use std::env;
use std::io::{self, BufRead, Write};

fn usage() {
    println!("lyco-desktop-switcher [next|prev|status|capabilities|mcp] [--json]");
    println!(
        "Switching requires an interactive Windows desktop; status reports capabilities only."
    );
}

fn command(name: &str) -> Result<Value, String> {
    match name {
        "capabilities" | "status" => Ok(json!({
            "platform": env::consts::OS,
            "switch_supported": cfg!(windows),
            "requires_interactive_session": true,
            "current_desktop": null,
            "desktop_count": null,
            "backend": "keyboard_shortcut",
            "limitations": "No native desktop enumeration or confirmation of switch"
        })),
        "next" | "prev" | "previous" => {
            #[cfg(windows)]
            {
                switch_desktop(name == "next");
                Ok(json!({"accepted":true,"operation":name,"confirmed":false}))
            }
            #[cfg(not(windows))]
            {
                Err("unsupported_platform: switching requires Windows".into())
            }
        }
        _ => Err(format!("unknown_command: {name}")),
    }
}

#[cfg(windows)]
fn switch_desktop(next: bool) {
    use std::thread::sleep;
    use std::time::Duration;
    #[link(name = "user32")]
    extern "system" {
        fn keybd_event(vk: u8, scan: u8, flags: u32, extra_info: usize);
    }
    const KEYUP: u32 = 2;
    const WIN: u8 = 0x5B;
    const CTRL: u8 = 0x11;
    let arrow = if next { 0x27 } else { 0x25 };
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

fn tools() -> Value {
    json!([
        {"name":"desktop_status","description":"Read desktop switching capability (does not enumerate virtual desktops)","inputSchema":{"type":"object","properties":{},"additionalProperties":false}},
        {"name":"desktop_next","description":"Request switch to the next Windows virtual desktop; not confirmed","inputSchema":{"type":"object","properties":{},"additionalProperties":false}},
        {"name":"desktop_previous","description":"Request switch to the previous Windows virtual desktop; not confirmed","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}
    ])
}

fn response(req: &Value) -> Option<Value> {
    let id = req.get("id")?.clone();
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let result = match method {
        "initialize" => json!({
            "protocolVersion":"2025-03-26",
            "capabilities":{"tools":{}},
            "serverInfo":{"name":"lyco-desktop-switcher","version":env!("CARGO_PKG_VERSION")}
        }),
        "ping" => json!({}),
        "tools/list" => json!({"tools":tools()}),
        "tools/call" => {
            let name = req
                .pointer("/params/name")
                .and_then(Value::as_str)
                .unwrap_or("");
            let op = match name {
                "desktop_status" => "status",
                "desktop_next" => "next",
                "desktop_previous" => "prev",
                _ => "invalid",
            };
            match command(op) {
                Ok(value) => {
                    json!({"content":[{"type":"text","text":value.to_string()}],"isError":false})
                }
                Err(error) => json!({"content":[{"type":"text","text":error}],"isError":true}),
            }
        }
        _ => {
            return Some(
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"Method not found"}}),
            )
        }
    };
    Some(json!({"jsonrpc":"2.0","id":id,"result":result}))
}

fn mcp() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line?;
        if let Ok(request) = serde_json::from_str::<Value>(&line) {
            if let Some(reply) = response(&request) {
                writeln!(stdout, "{reply}")?;
                stdout.flush()?;
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let json_mode = args.iter().any(|arg| arg == "--json");
    let args: Vec<&str> = args
        .iter()
        .filter(|arg| arg.as_str() != "--json")
        .map(String::as_str)
        .collect();
    match args.as_slice() {
        ["mcp"] => {
            if let Err(error) = mcp() {
                eprintln!("mcp_io_error: {error}");
                std::process::exit(1);
            }
        }
        ["--help"] | ["-h"] | ["help"] | [] => usage(),
        [name] => match command(name) {
            Ok(value) => {
                if json_mode {
                    println!("{value}");
                } else {
                    println!("{value:#}");
                }
            }
            Err(error) => {
                if json_mode {
                    println!("{}", json!({"error":error}));
                } else {
                    eprintln!("{error}");
                }
                std::process::exit(2);
            }
        },
        _ => {
            eprintln!("invalid_arguments");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_is_machine_readable() {
        assert!(command("status").unwrap().get("backend").is_some());
    }
    #[test]
    fn mcp_lists_three_tools() {
        let req = json!({"jsonrpc":"2.0","id":1,"method":"tools/list"});
        let value = response(&req).unwrap();
        assert_eq!(value["result"]["tools"].as_array().unwrap().len(), 3);
    }
    #[test]
    fn notification_has_no_reply() {
        assert!(response(&json!({"jsonrpc":"2.0","method":"notifications/initialized"})).is_none());
    }
}
