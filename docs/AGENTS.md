# 文档宪法

本页定义 `docs/` 的写作与分层规则。不进 spec-anchor 预览导航。

## 文档契约（唯一档位）

本项目采用 **spec-anchor 文档框架（L 档）**：文档即可执行契约，值得长期遵守的约定由 `spec-anchor verify` 机械检查，违反则非零退出。

- **思想来源**：DeepSeek Harness 文档标准（单一事实源、现状与决策分离、薄总图 + 参考页）
- **形态**：`docs/topics/<kebab-case>/` 为产品规格唯一真源（**不用** `modules/` / `subsystems/`）
- **决策**：`.agents/notes/`（不进预览）
- **机写页**：`spec-anchor.kind: generated`（跳过 topic 四节检查；`source` + `sourceHash` 新鲜度由 `docs.generated-freshness` 检查）

### 当前机械保障（`spec-anchor verify`）

| Gate | 检查内容 | 默认（strict） | `--warn-only` |
|------|----------|----------------|---------------|
| `docs.framework` | topics 结构、四节骨架、AGENTS 不进预览 | error | error |
| `docs.arch-topics` | architecture §4 ↔ topics 双向登记 | error | warn |
| `notes.format` | Note 头部、生命周期章节 | error | error |
| `docs.budgets` | 站立文档字数上限 | error | warn |
| `content.mermaid` | Mermaid 语法（开启时） | error | error |
| `docs.slop-engine` | 产品页出现引擎字样 | warn | warn |
| `docs.generated-freshness` | 机写页 `source` / `sourceHash` 与源文件一致 | error（stale） | warn |
| `docs.links` | Markdown / 原型链接与锚点 | error | warn |

本地与 CI：`spec-anchor verify --verbose`（默认 strict；写到一半可用 `--warn-only`）

### 未纳入（刻意不做）

- 生成 catalog 链（工具表、配置表从源码 `--check`）
- 双语 `i18n/` 配对
- `type-equiv` / 文档内 TypeScript 编译
- Cookbook / Postmortem 独立层
- tutorial / reference 机械分类 gate

### 字数预算（`docs.budgets`）

预算单位 = **CJK 汉字数 + 英文词数**（`wc` 风格分词）。上限：

- `docs/architecture.md`：**1800**
- 本文件（`docs/AGENTS.md`）：**1200**

超限先搬迁细则到 `docs/topics/`，再考虑提高上限。

## 文档结构

产品规格以 **`docs/topics/<kebab-case>/`** 为唯一真源。`architecture.md` §4 扩展点只链 topics。

人写 topic `00-index.md` 四节：范围 / 行为契约 / 失败与边界 / 相关链接。同目录可增 `01-…md` 等补充页。机写页用 `spec-anchor.kind: generated`。

## 机写页同步契约

- `spec-anchor.source` **必填**（相对项目根）；`freshness: false` 仅关闭 hash 校验，**不**省略 `source`
- 默认 `freshness: true`：须填写 `sourceHash: "sha256:<hex>"`（对 `source` 整文件 UTF-8 内容做 SHA-256）
- Agent 更新机写页正文时：重算 hash 并写入 `sourceHash`；无正文生成 CLI
- stale（源文件已变、hash 未更新）：默认 **error**；`verify --warn-only` 仅 **warn**

示例：

```bash
spec-anchor hash site.config.yaml
# 输出：sourceHash: "sha256:..."
```

备选（macOS/Linux）：`shasum -a 256 site.config.yaml`（须手动加 `sha256:` 前缀）

frontmatter 示例：

```yaml
spec-anchor:
  kind: generated
  source: site.config.yaml
  sourceHash: "sha256:abc..."
  freshness: false   # 可选；仅关闭 hash 校验
```

## 七条军令

1. **每个事实一个家**；复述改为链接。`architecture.md` 只导航，不写操作级细则。
2. **先判 tutorial 还是 reference**；两者分开， substantial 混写要拆（无机械 gate，靠编辑判断）。
3. **写当前状态**。变更史进 git 或 `.agents/notes/`；不要写「以前 / 现在改成」。
4. **非平凡改动**同变更新增或更新一篇 `.agents/notes/`。
5. **`docs/` 产品规格**禁止出现 spec-anchor、预览命令、`_template.html` / `_tokens.css` 等引擎字样（宪法页与本索引除外）。
6. **注释与公开文档**写完整契约（行为、失败、所有权），不写推理过程与测试走读。
7. **链接用相对 Markdown 路径**；改名必须同变更修完全部入链。

## Slop 检查单

- 同一规则是否在多个家重复？只保留一处，其余改链接
- 是否在 `docs/` 产品页写了变更史叙事（「以前 / 现在」）？
- `implemented/` Note 是否仍含 `## Proposal` 或验收清单语气？
- 交叉引用是否用相对 Markdown 路径？

## 分层

| 路径 | 职责 |
|------|------|
| `docs/README.md` | 规格文档索引 |
| `docs/architecture.md` | 薄总图：组成、主链路、扩展点表 |
| `docs/topics/<kebab-case>/` | 产品规格唯一真源 |
| `.agents/notes/` | 决策与提案 |

## 参考

- 根 [`AGENTS.md`](../AGENTS.md) — 项目级 Agent 约定
- [`.agents/notes/README.md`](../.agents/notes/README.md) — Note 格式与生命周期
