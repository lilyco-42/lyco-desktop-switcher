# Lyco Desktop Switcher

轻量级 Windows 10/11 虚拟桌面切换工具，Rust 优先，沿用 [lilyco-42/lilyco](https://github.com/lilyco-42/lilyco) 的 `tiny_lilyco` 抽象和 `tiny_lilyco_uda` 桌面能力桥。

> 状态：规划 / Preflight。当前仓库不是已完成的切换器，尚未接入 Windows 虚拟桌面 API。

## MVP

- Windows 10/11：探测虚拟桌面数量、当前桌面，切换前后桌面。
- 支持 `Alt+1..9` 直达桌面（默认关闭覆盖常用应用快捷键；可改为 `Ctrl+Alt+1..9`）。
- 托盘展示当前桌面编号；单击或菜单切换；可暂停热键。
- 快捷键冲突提示，启动/退出清理，日志和错误反馈。
- 不依赖管理员权限，不调用关机/注销等危险系统功能。

## 架构

```
app (native shell)
  ├─ desktop-controller (trait + domain logic)
  ├─ platform/windows (virtual desktop API adapter / version capability check)
  ├─ shell/tray-hotkey (UDA tray if supported; native fallback for global hotkeys)
  └─ tiny_lilyco (future agent/orchestration integration, opt-in)
```

**边界**：UDA v0.2.1 的主题、托盘、通知等能力不等于虚拟桌面 API；切换桌面要另做 Windows 专用适配器。先调研受支持的 API 和跨 Win10/Win11 版本差异，必要时以原生快捷键模拟作为有明确限制的回退路径。

**复用策略**：不复制 `tiny_lilyco` 源码；等最小 Windows 原型稳定后，用 git tag/rev 固定依赖；UDA 保留独立桥层避免给无头 core 加重依赖。遵循上游 AGENTS.md 与 CODEGRAPH 规则。

## 计划

详见 [docs/PLAN.md](docs/PLAN.md)、[docs/RESEARCH.md](docs/RESEARCH.md)。

## 测试策略

本机不编译运行；用 GitHub Actions Windows runner 执行 fmt、clippy、test、构建、产物打包；真实 Win10/11 虚拟桌面切换仍需要在交互式 Windows 会话人工验收。
