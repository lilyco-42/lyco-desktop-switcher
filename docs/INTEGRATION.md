# Agent Integration

## CLI

```sh
lyco-desktop-switcher.exe status --json
lyco-desktop-switcher.exe capabilities --json
lyco-desktop-switcher.exe next --json
lyco-desktop-switcher.exe prev --json
```

Windows interactive session required for switching. JSON status does not claim to know virtual desktop count/current. A switch response `accepted:true, confirmed:false` means only that the keyboard shortcut was injected; it does **not** guarantee a switch occurred.

## MCP over stdio

Example MCP client config:

```json
{
  "mcpServers": {
    "lyco-desktop": {
      "command": "C:\\\\path\\\\to\\\\lyco-desktop-switcher.exe",
      "args": ["mcp"]
    }
  }
}
```

MCP `initialize`, `ping`, `tools/list`, `tools/call` supported, with tools:
`desktop_status`, `desktop_next`, `desktop_previous`. Runs using newline-delimited JSON-RPC messages on stdin/stdout; no HTTP listener, no network authentication. Client should explicitly approve mutating desktop tools. Unknown methods return JSON-RPC errors.

## Build / CI

Github Actions: `Windows Build`, release artifact zip includes the executable and README.
Do not equate compilation success with interactive virtual desktop functionality; validate on a real Windows user desktop.
