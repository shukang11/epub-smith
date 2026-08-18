# Topics 索引

**产品规格的唯一真源**在 `docs/topics/<kebab-case>/`。本页说明边界与字段含义；总图 [§4 新行为挂哪](../architecture.md#4-新行为挂哪) 只链到正式 topic。

## 边界

- 一个 topic = 一个可复述的产品问题域
- 跨 topic 只链接，不复述
- 禁止在 `docs/` 根再开「某 topic 的补充篇」

## Topic 入口 `00-index.md` 各节写什么

| 节 | 职责 |
|----|------|
| **范围** | 解决什么问题、不解决什么 |
| **行为契约** | 正常路径的输入、输出、状态 |
| **失败与边界** | 错误、空值、权限、并发等 |
| **相关链接** | 其他 topic / 原型；只链不复述 |

`spec-anchor verify` 对人写 `00-index.md` 检查上述四节；`spec-anchor.kind: generated` 的页面跳过四节检查。

同一 topic 下可有多篇文档：入口固定为 `00-index.md`；补充规格用 `01-…md`、`02-…md` 等（按文件名排序，可用 `nav.order` 微调）。

## 机写页（可选）

枚举型清单（配置字段表、API 列表等）放在同一 topic 下的独立 `.md`，用 frontmatter 标记，**默认进预览导航**：

```yaml
---
nav:
  title: 配置字段参考
spec-anchor:
  kind: generated
  source: site.config.yaml          # 必填，相对项目根
  sourceHash: "sha256:abc..."       # freshness 默认开启时必填
  freshness: false                  # 可选；仅关闭 hash 校验，不省略 source
---
```

人写 `00-index.md` 只链到机写页，不要手抄整张表。

## 新建 topic

1. **复制** [`_example/`](./_example/) 为 `docs/topics/<kebab-case>/`（`_example` 是隐藏骨架，不是正式规格）
2. 改 `00-index.md` 标题与各节正文
3. 在 `architecture.md` §4 增加一行并链到本 topic
