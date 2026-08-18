# Agent Note: 视觉预览形态（示例预览页）

Status: implemented

## Problem

用户需要「调整 CSS 后查看效果」的视觉预览。讨论过真实书籍预览（`preview serve` 渲染前 N 章）与示例预览两种路线。

## Alternatives considered

- 新增 `preview serve` 命令渲染真实书籍前 N 章：增量价值低，且需新命令与更多代码。
- 增强导出预览页（示例章节 + 翻页交互）：零新命令，满足「调 CSS 看样式」核心诉求。

## Decision

不新增 `preview serve` 命令，增强 `template export` 导出的 `preview.html`：示例章节扩至 6 个（小说/散文/科技/超长标题/对话/短段落），新增目录 + ◀/▶ 翻页 + 键盘 ←→ 交互，封面作为可切换的第 0 页。预览页是 style.css 的**注入快照**，编辑 CSS 后需重新注入/导出。

## Consequences

- 零新依赖、零运行时风险（仅改 `export.rs` 的 HTML 模板）。
- 用户工作流：`template export` → 改 style.css → 看 preview.html → `--style` 生成全书。
- 真实书籍预览若未来需要，可重新评估（roadmap 候选）。
- 预览页用法见 [styling](../../../docs/topics/styling/)。
