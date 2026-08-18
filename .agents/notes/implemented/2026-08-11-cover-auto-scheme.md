# Agent Note: 默认封面生成方案（--cover auto）

Status: implemented

## Problem

需要为 EPUB 提供封面，但用户环境可能无网络、无大模型，无法使用在线生图 API 或本地扩散模型。

## Alternatives considered

- 在线生图 API：依赖网络与付费服务，违背离线可预测承诺。
- 本地扩散模型：依赖重、无网络环境不可用。
- 不提供封面：功能缺失，书籍观感差。

## Decision

采用**离线 SVG 模板 + resvg 栅格化 PNG** 方案：`--cover auto` 用内嵌 `cover.svg` 模板填充书名/作者，经 resvg（纯 Rust，含系统字体库）栅格化为 1600×2560 PNG。封面来源收敛为三个：`--cover auto`（生成）、`--cover <文件>`（用户图片）、省略（无封面）。生成封面同时产出 `cover.xhtml` 封面页置于 spine 首位，兼容 Kindle 等对纯 manifest 声明支持不稳的阅读器。自定义封面模板（`--cover-template`）接线暂缓，纳入 roadmap。

## Consequences

- 无网络/模型依赖，输出确定性（同一输入生成相同封面），契合「可预测」理念。
- 封面渲染依赖系统中文字体（PingFang/雅黑/Noto fallback），跨设备字体可能不同。
- 引入 resvg 依赖，编译时间增加约 30-60s。
- 样式与封面细节见 [styling](../../../docs/topics/styling/)。
