# 架构分析

规范见 OSCA 总仓库（opensource-code-atlas） `docs/conventions/analysis-docs.md`。

每篇文档在 front matter 中声明 `anchors`（依赖的源码符号）；写完运行 `osca docs update`，上游改动这些符号后 `osca docs list` 会提示文档过时。
