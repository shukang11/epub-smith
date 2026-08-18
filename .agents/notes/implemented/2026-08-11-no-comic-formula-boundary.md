# Agent Note: 不支持漫画与公式（输入模型边界）

Status: implemented

## Problem

讨论未来扩展场景（漫画、数学公式）时，需评估当前 CSS 结构是否应为其预留。

## Alternatives considered

- 为漫画/公式添加「漫画容器」「公式」等 class：为不存在的输入添加死代码，重蹈 `.container` 死代码覆辙。
- 保持最小结构：范围聚焦纯文字书，未来扩展时作为独立特性规划。

## Decision

不为此调整结构：EpubSmith 输入是 TXT 纯文本，漫画（图片）与公式（LaTeX/MathML）在输入侧就无法表达，属于输入模型问题而非 CSS 结构问题。若未来扩展，需换输入格式（如 Markdown/HTML）并新增渲染器，是独立特性，单独规划。

## Consequences

- 范围聚焦纯文字书（TXT→EPUB），CSS 结构保持最小（标题/段落/目录/封面）。
- 页面结构规范化（`.container` 内容容器）是通用能力，对未来扩展不冲突（容器内可加元素）。
- 页面结构细节见 [styling](../../../docs/topics/styling/)。
