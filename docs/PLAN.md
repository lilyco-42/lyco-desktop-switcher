# 实施计划

## 需求/验收

1. Windows 10/11 用户无需管理员权限安装/运行，退出后不残留热键钩子。
2. Win+Tab 创建多个桌面后能正确显示数量、当前桌面并快速切换。
3. 多屏环境不误判虚拟桌面与物理显示器；无效编号不执行操作。
4. explorer.exe 重启、睡眠唤醒、桌面删除、热键冲突时可恢复或清楚提示。

## Milestone 0 — 可行性

- [x] 发现上游 `tiny_lilyco` 和 `tiny_lilyco_uda`，均为 lilyco 仓库内目录，不是分支。
- [x] 确认 UniDesktop SDK v0.2.1 覆盖主题/托盘等，但没有确认其提供虚拟桌面切换。
- [ ] 验证 Windows 虚拟桌面 API 在目标 Win10/11 build 的兼容性，决策使用官方 API / COM / 输入模拟。
- [ ] 验证系统托盘框架和可配置全局快捷键的事件循环兼容。

## Milestone 1 — MVP

- [ ] `VirtualDesktopBackend` trait：count/current/switch_to/switch_next/switch_previous。
- [ ] Windows 后端与 feature/capability probe；错误类型与超时处理。
- [ ] 托盘切换和状态提示；设置持久化，热键可配置。
- [ ] 非交互单元测试（domain、mock backend、非法索引、热键冲突模拟）。
- [ ] GitHub Actions Windows 打包 zip/exe（本机禁止编译）。

## Milestone 2 — 增强

- [ ] 将当前窗口移至指定桌面；可选择跟随切换。
- [ ] 桌面命名、桌面预览、轻量覆盖层。
- [ ] 与 `tiny_lilyco` 工具 schema 对接：只读查询默认开放；切换操作显式授权。
- [ ] 可选 Linux 适配（Wayland compositor/X11 环境差异要分别验证）。

## 风险

- Windows 虚拟桌面内部 COM 接口可能随 Windows 更新变化：隔离版本依赖并提供降级。
- 高权限程序的全局快捷键/窗口操作不能保证普通进程完全控制。
- 远端 CI 通常不具交互式桌面会话：只能测试纯逻辑和编译，真机功能需要现场验收。
