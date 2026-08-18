# 标注工作流（剪贴板）

## 存储

- 路径：`spec-anchor.local/annotations.json`
- 是否进 git：以项目 `.gitignore` 为准（init 默认 gitignore `spec-anchor.local/`）
- 本地持久化；预览面板可删除/清空（写回文件）

## 字段语义（v1，不扩 schema）

| 字段 | 含义 |
|------|------|
| `sourcePath` + `selector` | **锚点上下文** — 在哪标的、跳转高亮用 |
| `content` | **任务** — 要做什么（Agent 第一优先级） |
| `links` | **其它要改/要对的文件**（doc / prototype / code） |
| `agentHints` | 可选，一句防误解约束 |

`sourcePath` **不等于**唯一编辑目标。无 `links` 时，默认在 `sourcePath` 对应文件内按 task 修改。

## 两套坐标（不要混用）

| 坐标系 | 用途 | 字段 |
|--------|------|------|
| **runtime（预览 / DOM）** | 标注点选、跳转后高亮 | `selector` / `selectors[]` |
| **file（仓库源文件）** | Agent 打开文件、缩小范围 | `section lines` / `line hint` / `searchTexts`（复制时展开） |

**不强制** DOM → 行号。原型改版后 runtime 可能 stale；`content` 与 `links` 仍有效。

### 原型锚点（`proto-anchor-quote`）

| 字段 | 含义 |
|------|------|
| `quote` | 块内短可见文案（≤80），HTML 源 grep 用 |
| `anchor` | 可选，区块标题（如 pkg 的 `h3` 文本），同页重复结构时消歧 |
| `id` | 可选，有则最稳 |

点选时：嵌套 `div`/`span` 会上浮到线框容器（`.pkg-group`、`.region-col` 等）或交互控件；不要求作者给每块写 `id`。可选 `data-spec-anchor` 加分。

旧 `dom-snapshot`（含 `css` 路径）仍可解析与高亮，但新标注不再生成。

## 创建标注

1. **标注模式** — `features.annotations: true` → 顶栏「标注」或 **⌘⇧M** 循环（关 / 单选 / 多选）
2. **手改 JSON** — 编辑 `spec-anchor.local/annotations.json`，预览 ↻
3. **侧栏** — 列表、复制、删除、标为已解决、点击 `sourcePath` 跳转
4. **右栏** — 文档页 `大纲 | 标注`；`⌘.` 收起；拖拽左缘调宽

建议每条含：`sourcePath`、`kind`、`content`；跨文件加 `links`；块很大时依赖复制展开的 **section lines**（见下）。

## 给 Agent：剪贴板（唯一推荐路径）

用户在预览 **复制选中** → 粘贴到 Cursor / Codex / pi-agent。

**无** `spec-anchor annotations export` CLI。

### 剪贴板结构

意图说明 + 定位指引 + 单一 enriched JSON 数组。每条含存储字段与 `hints`（复制时计算，**非**写回 `annotations.json`）。

- **意图**：按每条 `content` 理解作者要求（修改、讨论、备忘等均可）；不预设「必须对齐修改」
- **定位**：剪贴板内嵌简短定位指引；`hints.sectionLines` / `lineHints` / `searchTexts` 帮 AI 找文件
- **skill**：剪贴板仅一行提示「如需完整工作流请 @spec-anchor」；完整规则在本文件，用户显式 @ 时加载

### 长 quote / 整块标选

存储仍保留完整 `quote`（跳转与韧性）。`hints` 中额外提供（不写回存储）：

| `hints` 字段 | 何时出现 | Agent 用法 |
|--------------|----------|------------|
| **sectionLines** | 有 `selector.anchor`（章节标题） | 优先读 `start`–`end` 整节 |
| **quoteExcerpt** | quote > 120 字 | 首尾摘录；完整见顶层 `selector.quote` |
| **searchTexts** | 短 quote + anchor | 子串搜索；超长 quote 已省略 |
| **lineHints** | 源文件命中 | 行号提示（`confidence: low` 时读上下文） |
| **runtime** | 有 selector | 预览/DOM 锚点状态 |

**推荐**：文档多选标列表项，少标整张表；必须大块时靠 `anchor` + **sectionLines**。

### 解析优先级

1. `content` / `agentHints` / `links`
2. `hints.sectionLines`（章节行号）
3. `hints.lineHints` / `hints.searchTexts`
4. `hints.quoteExcerpt`
5. `hints.runtime.selector`（理解预览意图；原型优先 `quote` / `anchor`）

### 粘贴后步骤

1. 读 JSON → 先理解 `content`（作者意图），再看 `links`、`sourcePath`
2. 用 `hints.sectionLines` / `lineHints` / `searchTexts` 定位源文件
3. 若用户 @spec-anchor → 读本文件（工作流、原型约束、resolve）
4. 按 `content` 意图行动（讨论则分析，修改则改文件）
5. 若已修改完成 → 提醒用户侧栏 **标为已解决**；`spec-anchor verify`

## 原型改动约束（指哪打哪）

改 `prototypes/*.html` 时，收窄自由度、只动标注命中处：

1. **先读基线**：`prototypes/_design.md`（设计原则、Screen 思考模板）与 `_tokens.css`（可用变量 / `sa-*` 类）。二者若不存在则跳过（消费者可能已删）。
2. **只改命中元素**：不重排、不重样式无关部分；样式复用 `_tokens.css` 变量与类，不新造内联魔法数值。
3. **贴合规格**：原型 `<head>` 的 `<link rel="spec-anchor-spec" href="/docs/…">` 指明它实现的规格；改动前对照该文档，别只按感觉发挥。
4. 旧原型可能仍用可见页脚 `spec-anchor-proto-footer`；兼容读取，但新页面用隐藏 `<link>` 元数据，不再加可见页脚。

## 标签建议

| tag | 含义 |
|-----|------|
| `drift` | 文档/原型/代码不一致 |
| `todo` | 待补充 |
| `blocked` | 依赖其它任务 |
