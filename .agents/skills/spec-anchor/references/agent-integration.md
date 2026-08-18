# Agent 工具对接

## 原则

1. 项目技能在 `.agents/skills/`（`spec-anchor skills install`）
2. 标注经用户**粘贴剪贴板**进入上下文；**无** export CLI
3. **spec-anchor 显式激活**：@spec-anchor 或用户要求处理标注块；勿被动加载
4. 改完后 **`spec-anchor verify --verbose`**（默认 strict，与 CI 一致；写到一半可用 `--warn-only`，提交前不得使用）

## Agent

1. 用户粘贴 `# spec-anchor annotations`（需要完整工作流时 @spec-anchor）
2. 先读 `content` 理解意图，用剪贴板定位指引 + `hints` 找文件
3. 若 @spec-anchor → 读取 skill，按 [annotation-workflow.md](./annotation-workflow.md) 执行
4. 改 `docs/` / `prototypes/` / `links` 路径
5. 提醒用户侧栏 resolve

- 会话前可选：`spec-anchor skills check` && `spec-anchor verify --verbose`
- 机写页：改 `source` 对应正文后运行 `spec-anchor hash <source>` 更新 `sourceHash`（写在机写页 frontmatter，非源文件内）
- 输入 = 用户粘贴的 enriched JSON（含前言），非管道 export

## 文档结构

- 文档框架初始化 → `spec-anchor init`；修订 topics / notes → 读 `docs/AGENTS.md`、`docs/topics/README.md`
- 预览与标注 → **spec-anchor**（显式触发时）

## 不要

- 未 @spec-anchor 时自动套用标注解析规则
- 在消费者项目 `npm i spec-anchor`
- 把 `sourcePath` 当成唯一要改的文件而忽略 `links`
