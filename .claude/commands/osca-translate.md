---
description: 为指定路径的源码添加 【zh】 中文注释
argument-hint: <文件或目录>
---

为 `$ARGUMENTS` 添加中文注释。

1. 先读 `.osca/CLAUDE.md` 与 `.osca/project.yaml`，确认规则、翻译范围和术语库。
2. 通读目标文件，理解它在系统中的职责，必要时查看其调用方与被调用方。
3. 按规则插入 `【zh】` 注释行：先写模块/类型级说明，再写复杂函数与关键分支。不改动任何已有行。
4. 完成后运行 `osca verify $ARGUMENTS` 和 `osca status`，确认通过。
5. 汇报：注释了哪些条目、发现的值得写进 `osca/docs/` 的设计要点、不确定之处。
