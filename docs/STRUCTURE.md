# 文档目录结构

```
docs/
├── README.md                 # 文档入口
├── MAP.md                    # 文档索引地图
├── STRUCTURE.md              # 目录结构维护（本文件）
├── architecture/             # 架构与设计
│   ├── system-map.md         # 模块总览与依赖关系
│   ├── data-flow.md          # 数据流与时序
│   ├── rendering.md          # 渲染与模板策略
│   └── packaging.md          # EPUB 打包策略
├── reference/                # 参考文档
│   ├── cli.md                # CLI 参数参考
│   ├── config.md             # 配置项参考
│   └── templates.md          # 模板与样式参考
├── delivery/                 # 构建与发布
│   ├── build.md              # 构建说明
│   ├── release.md            # 发布流程
│   └── distribution.md       # 分发与兼容性
└── llm_ctx/                  # 状态同步与决策
    ├── STATUS.md             # 当前状态快照
    ├── ROADMAP.md            # 里程碑规划
    ├── CHANGELOG.md          # 变更日志
    └── decisions/            # ADR/技术决策记录
```
