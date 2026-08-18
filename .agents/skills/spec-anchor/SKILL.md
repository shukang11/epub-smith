---
name: spec-anchor
description: >-
  Preview spec docs and HTML prototypes (spec-anchor CLI); annotation clipboard
  workflow for aligning docs, prototypes, and code. Installed via spec-anchor
  skills install. Use ONLY when the user @spec-anchor or pastes a spec-anchor
  annotations block and asks to process it. Not for passive activation.
---

# spec-anchor

阅读 + 关联 + 标记：预览站入口，标注剪贴板对接 Agent。CLI：**`spec-anchor`**（`spec-anchor dev`）。

## 何时加载本 skill

| 加载 | 不加载 |
|------|--------|
| 用户 **@spec-anchor** | 普通写代码 / 改文档 |
| 用户粘贴 `# spec-anchor annotations` 并要求处理 | 仅提到「文档」但未给标注块 |
| 用户明确「按 spec-anchor 标注处理」 | |

剪贴板内 **「技能」** 节与上表一致：未满足条件时 **不要**套用本 skill。

## 前提

- `which spec-anchor` 成功（见 references/path-setup.md）
- `spec-anchor skills check` 通过（项目 `.agents/skills/spec-anchor`）
- 工作在含 `site.config.yaml` 的项目根

## 快速判断

| 情况 | 动作 |
|------|------|
| 无 `site.config.yaml` | `spec-anchor init` |
| 技能缺失 | `spec-anchor skills install` |
| 预览失败 | `spec-anchor verify --verbose` |
| 原型/文档链接问题 | `spec-anchor verify --verbose`（默认 strict） |
| 写到一半的 docs | `spec-anchor verify --warn-only`（草稿，不能代替提交前检查） |
| 机写页更新 sourceHash | `spec-anchor hash <source 路径>` |
| 处理标注剪贴板 | references/annotation-workflow.md |
| 文档结构修订 | 读 `docs/AGENTS.md`、`docs/topics/README.md`（先 `spec-anchor init`） |

## 文档门禁（修订 `docs/` 时）

### `verify`

| 时机 | 命令 |
|------|------|
| **提交前 / CI 同款** | `spec-anchor verify --verbose`（**默认 strict**，不要加 `--warn-only`） |
| **写到一半（WIP）** | `spec-anchor verify --warn-only`（arch 未登记、stale hash、坏链等仅 warn） |

`--warn-only` 只用于中间态；**不得以 warn-only 通过代替合入前检查**。

### 机写页（`spec-anchor.kind: generated`）

- **`source`**：这份文档同步自哪个文件，**相对项目根**；不限 `site.config.yaml`（示例常用它，因 init 项目必有）
- **`sourceHash`**：写在**机写页 `.md` 的 frontmatter**里，**不是**写在 `source` 源文件里
- 更新流程：改正文 → `spec-anchor hash <source 路径>` → 将输出贴入机写页 `sourceHash`

```yaml
# docs/topics/billing/config-keys.md（机写页）
spec-anchor:
  kind: generated
  source: site.config.yaml      # 或 openapi.yaml、src/schema.json 等
  sourceHash: "sha256:..."      # spec-anchor hash site.config.yaml 的输出
```

```bash
spec-anchor hash site.config.yaml
# → sourceHash: "sha256:abc..."
```

`freshness: false` 仅关闭 hash 比对，**不**省略 `source`。详见 `docs/AGENTS.md`「机写页同步契约」。

## 标注 → Agent

用户从预览「复制选中」后粘贴。含：意图说明、定位指引、**enriched JSON**（存储字段 + `hints` 展开）。

详见 [annotation-workflow.md](references/annotation-workflow.md)。**无** `annotations export` CLI。

要点：按 `content` 理解作者意图；`sourcePath` = 标注位置；定位看 `hints`；完整工作流请用户 @spec-anchor。

## 详细参考

- [path-setup.md](references/path-setup.md)
- [annotation-workflow.md](references/annotation-workflow.md)
- [agent-integration.md](references/agent-integration.md)
