# 模块总览与依赖关系

## 模块结构

```
booksmith/
├── src/                         # 源代码目录
│   ├── main.rs                  # 主入口文件
│   ├── lib.rs                   # 库入口文件
│   ├── cli.rs                   # CLI参数定义与解析
│   ├── config.rs                # 配置管理
│   ├── models.rs                # 核心数据模型
│   ├── output.rs                # 输出管理
│   ├── export.rs                # 模板导出功能
│   ├── parser/                  # 文本解析模块
│   │   ├── mod.rs               # 解析模块入口
│   │   ├── encoding.rs          # 编码处理
│   │   └── chapter.rs           # 章节解析
│   ├── renderer/                # 渲染模块
│   │   └── mod.rs               # 渲染模块入口
│   ├── packager.rs              # EPUB打包模块
│   ├── templates/               # 模板文件
│   │   ├── chapter.xhtml.tera   # 章节XHTML模板
│   │   ├── nav.xhtml.tera       # 导航XHTML模板
│   │   └── content.opf.tera     # EPUB内容元数据模板
│   ├── utils/                   # 工具函数模块
│   │   ├── coherence.rs         # 章节一致性检查
│   │   ├── html.rs              # HTML处理函数
│   │   ├── number.rs            # 数字转换函数
│   │   └── mod.rs               # 工具模块入口
│   └── resources/               # 资源文件
│       └── css/                 # CSS样式
│           └── default.css      # 默认样式表
├── examples/                    # 示例文件
├── tests/                       # 测试目录
├── docs/                        # 文档目录
├── Cargo.toml                   # 项目配置
├── Cargo.lock                   # 依赖锁定
├── README.md                    # 项目说明
└── .gitignore                   # Git忽略文件
```

## 文件作用与边界

### 主入口文件

| 文件 | 作用 | 边界 |
|------|------|------|
| `main.rs` | 程序主入口，处理命令行参数，协调各模块工作 | 仅负责流程控制，不包含业务逻辑 |
| `lib.rs` | 库入口，导出公共API | 定义模块结构，不包含实现细节 |

### 核心模块

| 文件 | 作用 | 边界 |
|------|------|------|
| `cli.rs` | 定义和解析命令行参数 | 仅处理CLI相关逻辑，不涉及业务处理 |
| `config.rs` | 管理应用配置，加载规则文件 | 仅处理配置相关逻辑 |
| `models.rs` | 定义核心数据结构 | 仅包含数据模型定义，不包含业务逻辑 |
| `output.rs` | 管理输出格式化和显示 | 仅处理输出相关逻辑，不涉及业务处理 |
| `export.rs` | 导出样式模板到指定目录 | 仅处理模板导出功能，不涉及其他业务逻辑 |

### 解析模块

| 文件 | 作用 | 边界 |
|------|------|------|
| `parser/mod.rs` | 解析模块入口，协调编码处理和章节解析 | 仅负责模块间协调 |
| `parser/encoding.rs` | 自动探测和转换文件编码 | 仅处理编码相关逻辑 |
| `parser/chapter.rs` | 基于正则表达式解析章节 | 仅处理章节解析逻辑 |

### 渲染模块

| 文件 | 作用 | 边界 |
|------|------|------|
| `renderer/mod.rs` | 将Book数据渲染为XHTML | 仅处理XHTML渲染，不涉及EPUB打包 |

### 打包模块

| 文件 | 作用 | 边界 |
|------|------|------|
| `packager.rs` | 将XHTML文件打包为EPUB | 仅处理EPUB打包，不涉及XHTML渲染 |

### 工具模块

| 文件 | 作用 | 边界 |
|------|------|------|
| `utils/mod.rs` | 工具模块入口 | 仅负责导出工具函数 |
| `utils/coherence.rs` | 检查章节编号的一致性 | 仅处理章节一致性检查 |
| `utils/html.rs` | HTML处理函数 | 仅处理HTML相关逻辑，如转义、修复等 |
| `utils/number.rs` | 数字转换函数 | 仅处理数字转换，如中文数字转阿拉伯数字等 |

### 模板与资源

| 文件 | 作用 | 边界 |
|------|------|------|
| `templates/chapter.xhtml.tera` | 章节XHTML模板 | 仅定义模板结构，不包含业务逻辑 |
| `templates/nav.xhtml.tera` | 导航XHTML模板 | 仅定义模板结构，不包含业务逻辑 |
| `templates/content.opf.tera` | EPUB内容元数据模板，定义EPUB的manifest、spine和metadata | 仅定义模板结构，不包含业务逻辑 |
| `resources/css/default.css` | 默认CSS样式 | 仅定义样式，不包含业务逻辑 |
