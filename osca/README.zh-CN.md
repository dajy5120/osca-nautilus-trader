# NautilusTrader 源码学习版（OSCA）

> 本仓库是 [NautilusTrader](https://github.com/nautechsystems/nautilus_trader) 的**非官方**中文源码学习版本，属于
> [OpenSource Code Atlas](https://github.com/dajy5120/opensource-code-atlas) 项目。
> 上游代码保持原样（许可证：LGPL-3.0，见根目录 LICENSE），仅增加以 `【zh】` 开头的中文注释；
> 所有版权归上游作者所有。

## 分支

| 分支 | 内容 |
|------|------|
| `study/zh-CN`（默认） | 上游源码 + `【zh】` 中文注释 + 分析文档 |
| `mirror/develop` | 官方原始源码镜像 |
| `exp/*` | 实验分支 |

当前对应的上游版本见 [`.osca/sync.yaml`](../.osca/sync.yaml)，翻译进度见 [`.osca/status.json`](../.osca/status.json)。

## 阅读路线

> 待补充：推荐的阅读顺序、核心模块与关键流程。

## 分析文档

- [架构](docs/architecture/)
- [模块](docs/modules/)
- [流程](docs/flows/)
- [笔记](docs/notes/)

## 如何查看去掉中文注释后的原文

```bash
git diff mirror/develop -- <path>     # 只会看到 【zh】 行
osca strip <path>                                   # 输出剥离后的源码
```
