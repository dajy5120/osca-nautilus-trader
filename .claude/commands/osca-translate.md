---
description: 为指定路径的源码添加 【zh】 中文注释（交互式，适合精读；批量请用 `osca translate`）
argument-hint: <文件或目录>
---

为 `$ARGUMENTS` 添加中文注释。

1. 先读 `.osca/CLAUDE.md` 与 `.osca/project.yaml`，确认规则、翻译范围和术语库。
2. 运行 `osca queue $ARGUMENTS --state pending` 和 `osca queue $ARGUMENTS`，确定待翻译与过时的符号。
3. 通读目标文件，理解它在系统中的职责，必要时查看其调用方与被调用方。
4. 按规则插入 【zh】 注释行：先写模块 / 类型级说明，再写复杂函数与关键分支。不改动任何已有行。
   过时（stale）的注释：先读上游改动，不准确就修改；仍然正确就不要动，留给人工 `osca review approve`。
5. 完成后运行 `osca verify $ARGUMENTS`、`osca terms lint $ARGUMENTS`、`osca index --write`、`osca status`。
6. 汇报：注释了哪些符号、值得写进 `osca/docs/` 的设计要点、不确定之处。

批量翻译（结构化补丁、按符号、自动术语注入）请用：`osca translate $ARGUMENTS --dry-run`，确认后去掉 `--dry-run`。
