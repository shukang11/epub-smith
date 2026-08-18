# AGENTS 宪章（BookSmith / EpubSmith）

本文件用于指导自动化/协作者在本仓库中的工作方式，确保代码质量、文档一致性与可维护性。

## 目标

- 保持 TXT → EPUB 输出的稳定性与可预测性。
- 变更范围小而清晰，优先解决根因。
- 文档结构清晰，状态信息可同步维护。

## 工作流约定

- 开始修改前必须阅读文档：
  1. 首先阅读 `README.md` 了解项目概况
  2. 阅读 `docs/AGENTS.md` 了解文档分层与写作规则
  3. 阅读 `docs/README.md` → `docs/architecture.md` 获取文档地图，明确文档结构与查询路径
  4. 根据任务需要阅读 `docs/topics/` 相关具体规格
- 代码文件更新时，务必同步更新 `docs/architecture.md` 与相关 `docs/topics/`，确保规格精确反映代码结构与边界
- 修改后优先运行与变更范围匹配的测试；推荐完整流程：
  - `cargo test`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo fmt`
  - 动过 `docs/` 时跑 `spec-anchor verify --verbose`（提交前必须通过）
- 避免无关重构；不更改公共行为时不新增配置项。

## 代码风格

- 遵循 Rust 2024 风格与现有模块边界。
- 模块职责清晰：解析、渲染、打包、工具函数避免交叉污染。
- 错误处理优先使用 `anyhow`/`thiserror` 体系，保持错误信息可读。
- 变量命名使用 `snake_case`，避免单字母命名。

## 文档规范

- 积极阅读文档：在开始任何工作前，先了解相关文档内容，确保理解项目结构和设计意图
- 积极更新文档：代码变更时同步更新相关文档，保持文档与代码的一致性
- 产品规格一律写在 `docs/topics/<kebab-case>/`（唯一真源）；`docs/architecture.md` 只导航不重复细则
- 非平凡改动（改变用户可见行为/API 契约/共享约定/文件格式/安全假设）必须新增或更新一篇 `.agents/notes/`
- 默认使用中文撰写；如需双语，先中文后英文
- 提交前运行 `spec-anchor verify --verbose`，确保机械门禁通过

## 不做的事

- 不做与需求无关的重构。
- 不引入新的格式化/构建工具链。
- 不更改发布流程除非明确要求。

<!-- spec-anchor:docs-framework -->

## 开工读序

1. 本文件
2. **`docs/AGENTS.md`** — 文档宪法：`docs/` 的职责、分层与写作规则（**动 `docs/` 前先读**）
3. 与任务相关的 `.agents/notes/**/*.md` — 决策记录
4. `docs/README.md` → `docs/architecture.md` → `docs/topics/<kebab-case>/` → 具体规格

产品规格一律写进 `docs/`；`docs/` 里放什么、怎么分层、写作规则，**以 [`docs/AGENTS.md`](docs/AGENTS.md) 为准**，本文件不重复。

## 目录分工

| 路径 | Git | 用途 |
|------|-----|------|
| `docs/` | 是 | 产品规格 Markdown（spec-anchor 预览；职责见 `docs/AGENTS.md`） |
| `prototypes/` | 可选 | HTML 原型；有界面时从 `_template.html` 复制 |
| `.agents/notes/` | 是 | 决策记录（proposed / implemented / rejected / archived） |
| `.agents/skills/` | 是 | 项目内 Agent 技能 |

<!-- /spec-anchor:docs-framework -->
