# Agent Notes

决策与提案记录。不进 spec-anchor 预览。

## 布局

```text
.agents/notes/
  proposed/      # 尚未落地或部分落地的提案
  implemented/   # 已落地的决策（保持与现状一致）
  rejected/      # 已否决的提案
  archived/      # 冻结的历史（可选）
```

文件名：`yyyy-mm-dd-topic.md`（日期为首次提出日）。

## 何时写一篇（非平凡）

以下任一成立即应新增或更新 Note（verify 检查正文骨架）：

- 改变用户可见行为或 API 契约
- 改变跨模块/跨目录的共享约定
- 改变磁盘、网络或配置文件格式
- 改变安全假设或信任边界

纯机械编辑（错别字、格式、无契约变化的更名）可豁免。

## 文件头（全生命周期共有）

```markdown
# Agent Note: <title>

Status: proposed | implemented | rejected — <一行原因>

## Problem
…

## Alternatives considered
…
```

`Status:` 必须与所在生命周期目录一致。**所有目录**都必须有 `## Problem` 与 `## Alternatives considered`。

### `proposed/`

在共有骨架之上追加：

```markdown
## Proposal
## Acceptance criteria
## Risks
```

### `implemented/`

在共有骨架之上追加：

```markdown
## Decision
## Consequences
```

`implemented/` 中不得出现 `## Proposal`（表示未改写的提案残留）。

### `rejected/`

保留提案期章节；裁决写在 `Status:` 行。

## Note 专属 Slop 检查

- `Status:` 是否与所在生命周期目录一致？
- `implemented/` Note 是否仍含 `## Proposal` 或验收清单语气？
- 第一个 `##` 是否为 `## Problem`？

## 交叉引用

Note 之间用相对 Markdown 链接，不要裸写编号。
