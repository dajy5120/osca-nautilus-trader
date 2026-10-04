# OSCA 学习仓库规则（NautilusTrader）

本仓库 = **NautilusTrader 上游源码 + `【zh】` 中文注释 + OSCA 元数据**。
锚点（对应的官方提交）见 `.osca/sync.yaml`。以下规则优先于上游 CLAUDE.md / AGENTS.md 中的开发指引——
在这里你是在**阅读和注释**源码，而不是开发 NautilusTrader。

## 你只被允许做两件事

1. 在上游源码中插入**独占一行**、以全角标记 `// 【zh】`、`/// 【zh】`、`//! 【zh】` 或 `# 【zh】` 开头的注释行；
2. 新增或修改 `.osca/`、`osca/` 下的文件。

## 绝对禁止

- 修改、删除、移动任何非 `【zh】` 行——包括空白、空行、英文注释、docstring；
- 行尾注释；在字符串、docstring、rustdoc 代码块（```）内部插入；
- 运行 rustfmt / cargo fmt / ruff / black / pre-commit 等格式化工具；
- 提交构建脚本生成的文件（`.osca/project.yaml` 的 `study.generated`），构建后用 `git checkout` 恢复；
- 在 `mirror/*` 分支提交；向上游仓库发 PR；
- 在 `study/zh-CN` 上做代码实验（实验用 `exp/<category>/<name>` 分支）。

每次编辑后会自动运行 `osca verify`（`.claude/settings.json` hook）。若报告 `code-modified`，立刻撤销那处改动。
完成一批注释后运行 `osca verify && osca index --write && osca status`。

## 批量翻译

`osca translate <路径> --dry-run` 查看计划与 token 估算，确认后去掉 `--dry-run`（默认通过 `claude -p` 使用订阅额度）。
它只产出结构化注释并由工具插入，结果仍需人工审核。斜杠命令：`/osca-translate`、`/osca-sync`、`/osca-review`、`/osca-analyze`。

## 复核过时的注释

`osca queue` 列出上游改动后需要复核的注释（P0 签名 / P1 英文文档 / P2 实现）。对每一项先阅读上游的改动，再：
- 注释已不准确：修改 【zh】 行（编辑即重新翻译）；
- 注释仍然正确：不要修改，留给人工 `osca review approve`——你不能替人确认审核。

## 注释写法

- 规范：dajy5120/opensource-code-atlas 仓库 `docs/conventions/comment-style.md`。
- Rust：有英文 `///` 文档的条目，把 `/// 【zh】` 追加在英文文档之后、`#[...]` 之前；其余位置用 `// 【zh】`。
- Python / Cython：不改 docstring，在 `def` / `class` 及装饰器之上写 `# 【zh】`。
- 翻译 + 讲解：说明为什么这样写、在系统中的位置、不变式与边界情况；不复述代码。
- 术语首次出现写“中文（English）”。术语库：general, rust, python, trading, distributed-systems（见 dajy5120/opensource-code-atlas 的 `terminology/`）。
- 更长的分析写进 `osca/docs/`，并在注释中引用路径。
