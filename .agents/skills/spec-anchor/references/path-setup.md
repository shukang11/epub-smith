# PATH 与项目技能安装

## 引擎（本机一次）

在 **spec-anchor 引擎仓库**：

```bash
npm run setup    # install + build + ~/.local/bin/spec-anchor
```

（引擎包管理统一为 pnpm；`npm run setup` 内部即 `pnpm install` + build + link）

验证：`which spec-anchor` && `spec-anchor --version`

## 项目技能（每个消费者仓库）

技能**不**从 `~/.agents/skills` 读取，而从 **spec-anchor 引擎包内嵌的 `skills/`** 安装到**当前项目**：

```bash
cd /path/to/my-project
spec-anchor init              # 默认安装到 .agents/skills + .cursor/skills
# 或
spec-anchor skills install
spec-anchor skills check
```

| 路径 | 含义 |
|------|------|
| `<engine>/packages/cli/skills/` | 源（`spec-anchor`） |
| `<project>/.agents/skills/spec-anchor/` | 预览、标注剪贴板 |
| `<project>/.cursor/skills/spec-anchor/` | Cursor 项目技能（同上） |

开发引擎时可用符号链接（链到引擎源码，改完即生效）：

```bash
spec-anchor skills install --link
```

## 同捆技能

| 技能 | 用途 |
|------|------|
| `spec-anchor` | 预览、标注剪贴板、verify/dev |

## 引擎更新后

```bash
cd spec-anchor && npm run build
cd my-project && spec-anchor skills install
```
