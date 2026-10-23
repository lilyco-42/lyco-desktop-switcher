# 研究资料和选型

- Upstream: https://github.com/lilyco-42/lilyco/tree/main/tiny_lilyco
- UDA bridge: https://github.com/lilyco-42/lilyco/tree/main/tiny_lilyco_uda
- UDA: https://github.com/UniDesktop/SDK (Apache-2.0, 当前上游桥固定 v0.2.1)
- Install: https://unidesktop.github.io/en/getting-started/installation/
- Windows native UI capabilities: https://unidesktop.github.io/
- Reference: https://github.com/choyy/VirtualDesktopSwitcher (direct numbered desktops)
- Reference: https://github.com/RobertMueller2/WorkspaceSwitch.NET (hotkeys + moving windows; notes unstable virtual desktop COM internals)
- Reference: https://github.com/dankrusi/WindowsVirtualDesktopHelper (desktop indicator)

## 选择

建立独立新仓库，以 Rust 实现轻量 Windows 原生桌面切换器，借鉴上述项目的成熟交互；复用 lilyco 中稳定抽象，UDA 只承担已支持的桌面集成。优先跑通按编号切换、可观察状态和异常降级，不自行发明通用虚拟桌面协议。

## 待确认

- 针对实际 Windows 构建版本确定 `IVirtualDesktopManager` 支持范围及内部 COM 的版本映射。
- 选择托盘事件循环方案后验证与全局快捷键监听相容。
- 对用户可配置快捷键提供冲突检测，默认不要抢占系统/应用热键。
