---
description: 生成或更新模块级架构分析文档（osca/docs/）
argument-hint: <模块路径，如 crates/model/src/orderbook>
---

为 `$ARGUMENTS` 撰写或更新架构分析文档 `osca/docs/modules/<模块名>.md`。

1. 通读模块源码（含已有 【zh】 注释）与它的主要调用方；`osca index --show $ARGUMENTS` 获取符号列表。
2. 文档结构：一句话定位 → 核心抽象与数据结构 → 关键流程（Mermaid 图）→ 不变式与边界情况 → 与其他模块的关系 → 阅读顺序建议。
3. front matter 必须包含 `upstream_commit`（取 `.osca/sync.yaml` 的锚点）和 `anchors`（文档依赖的符号 ID 列表），
   规范见 opensource-code-atlas 的 `docs/conventions/analysis-docs.md`。
4. 只陈述能从源码确认的事实；推断要标明。完成后 `osca verify`。
