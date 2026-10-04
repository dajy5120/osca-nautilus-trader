---
description: 同步上游新版本，并整理需要复核的中文注释
---

1. 确认工作区干净且位于 `study/zh-CN`：`git status`、`git switch study/zh-CN && git pull`。
2. `osca sync --dry-run` 查看目标版本；然后 `osca sync`。
3. 阅读生成的 `.osca/reports/sync-*.md`：
   - 在报告开头补一段“本版本要点”摘要（3～6 条，面向源码学习者，基于上游提交与符号变化）；
   - 检查“挂到符号开头的注释”，必要时把注释移到更合适的位置（只移动 【zh】 行）。
4. `osca verify`，提交上述改动，然后 `git push origin mirror/<branch> sync/<tag>` 并开 PR。
5. 提醒用户：合并 sync PR 必须选择 “Create a merge commit”。之后用 `/osca-review` 处理复核队列。
