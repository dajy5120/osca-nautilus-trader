---
description: 逐条处理复核队列中过时的中文注释
argument-hint: [路径前缀]
---

1. `osca queue $ARGUMENTS` 列出过时（stale）的注释，按 P0 签名 / P1 英文文档 / P2 实现排序。
2. 对每一项：`osca index --show <file>` 定位，阅读当前代码与上游 diff（报告或 `git log -p mirror/<branch> -- <file>`）。
   - 注释已不准确：修改 【zh】 行（编辑即重新翻译）。
   - 注释仍然正确：记录下来，**不要**自己运行 `osca review approve`——审核必须由人确认。
3. 改完后 `osca verify && osca index --write`。
4. 汇报：改了哪些、哪些你认为仍然正确（附理由），请用户确认后由用户执行
   `osca review approve <符号 ID…>`。
