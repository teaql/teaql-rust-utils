# 让 AI 少猜 API：我们用 Rust 统一了 52 个常用开发工具

Rust 生态从来不缺优秀的 crate。

生成 UUID 有 `uuid`，处理时间有 `chrono`，精确金额可以使用 `rust_decimal`，HTTP 有 `reqwest`，JSON 有 `serde_json`。真正进入业务开发后，问题往往不是“有没有库”，而是“团队需要记住多少套不同的 API”。

这个问题在 AI 辅助编程出现之后更加明显。人可以查文档确认方法名，AI 却经常把不同语言、不同版本甚至不同 crate 的调用方式混在一起。生成的代码看起来很合理，编译器却告诉你这个方法根本不存在。

[TeaQL Tool](https://github.com/teaql/teaql-rust-utils) 是我们对这个问题的一次尝试：在成熟的 Rust crate 之上建立一个稳定的工具门面，把 52 个常用工具统一到 `T::xxx()`，再利用类型系统要求应用层代码说明一次操作的目的。

它有点像 Rust 版 Hutool，但我们真正想讨论的并不是“工具够不够多”，而是：能否为人和 AI 提供一套更可预测的业务 API？

## Rust 生态不缺 crate，缺的是统一的业务词汇

一个普通业务服务很快就会用到时间、ID、金额、JSON、正则、编码、文件和 HTTP。直接使用各个底层 crate 并没有错，而且在需要完整能力时通常是最好的选择。

问题出现在大量重复的常规操作中：每个 crate 有不同的构造方式、错误类型、命名习惯和版本演进节奏。业务代码逐渐知道了太多基础设施细节，AI 也需要在更大的 API 空间里猜测正确答案。

TeaQL Tool 没有重新实现这些基础能力，而是提供了一层窄而稳定的 facade：

```rust
use teaql_tool::T;

let id = T::id().uuid();
let now = T::time().now();
let value = T::json().parse(r#"{"name":"TeaQL"}"#)?;
let digest = T::hash().sha256(b"hello");
```

调用者只需要先找到 `T`，再从含义明确的工具名进入。底层究竟使用哪个 crate，仍然可以由工具层选择和升级。

## 52 个工具如何分层

项目目前由五个 crate 组成：

```text
teaql-tool-core
       │
       ├── teaql-tool-std       26 个标准工具
       ├── teaql-tool-extra     26 个扩展工具
       │
       └── teaql-tool           统一的 T:: facade
                    │
                    └── teaql-tool-context
                        UserContext 与业务意图适配
```

`teaql-tool-std` 包含文本、时间、ID、金额、Decimal、JSON、正则、编码、哈希、文件、集合、脱敏等常用能力。

`teaql-tool-extra` 放置依赖更重或带有 IO 的工具，包括 HTTP、命令执行、ZIP、Excel、CSV、图片、邮件、JWT、加密、二维码、模板、KV、静态文件服务器和文件监听等。

`teaql-tool` 本身很薄，只负责 feature 和 `T::xxx()` 入口。默认的 `minimal` feature 启用标准工具，需要网络、图片或自动化能力时再显式启用 `extra`：

```toml
[dependencies]
teaql-tool = {
    git = "https://github.com/teaql/teaql-rust-utils",
    features = ["std", "extra"]
}
```

完整的 52 个工具清单可以在项目 README 中查看。这里更值得介绍的是 facade 之上的另一层设计。

## 把“为什么调用”放进类型系统

统一方法名只能减少 API 猜测，不能解释业务代码为什么要读取一个文件、获取当前时间或者执行一条命令。

`teaql-tool-context` 为 `UserContext` 增加了三类意图约束：

- `comment()`：说明一次计算的业务含义；
- `purpose()`：说明一次读取的目的；
- `audit_as()`：说明并执行一次带副作用的操作。

例如：

```rust
use teaql_tool_context::prelude::*;

let now = ctx.time()
    .now()
    .comment("读取当前时间以计算付款期限");

let deadline = ctx.time()
    .add_days(now, 7)
    .comment("计算订单付款宽限期");

ctx.file()
    .write_string("deadline.txt", deadline.to_rfc3339())
    .audit_as("导出付款期限")?;
```

计算和读取结果使用私有字段包装。调用者如果希望取得内部值，就必须显式调用对应的意图方法。

副作用需要更严格一些。`MustAuditAs<T>` 保存的不是已经执行完的结果，而是一个待执行动作：

```rust
pub struct MustAuditAs<T> {
    action: Option<Box<dyn FnOnce(String) -> T + Send + 'static>>,
}

impl<T> MustAuditAs<T> {
    pub fn audit_as(mut self, description: impl Into<String>) -> T {
        let action = self.action.take().expect("action executes once");
        action(description.into())
    }
}
```

如果调用者没有执行 `.audit_as(...)`，而是直接丢弃返回值，文件写入、命令执行或邮件发送都不会发生。这不是 lint 提示，而是由 API 结构决定的执行顺序。

目前这些包装负责强制收集意图，描述如何进入日志、链路追踪或审计存储，则由应用运行时决定。我们刻意把“要求提供意图”和“把意图写到哪里”拆成两个问题。

## 为什么这对 AI 编程有帮助

大模型生成代码时，一个常见问题是 API 表面积过大。它可能知道某项能力存在，却混淆具体 crate、版本或方法名。

统一 facade 后，生成空间会明显收缩：

- 工具入口固定为 `T::xxx()` 或 `ctx.xxx()`；
- 相似能力使用一致的命名方式；
- 底层依赖升级不会必然扩散到业务代码；
- 编译器可以通过包装类型提示缺少业务意图；
- 项目规则可以明确禁止应用层绕过 context 直接访问 IO。

换句话说，与其期待 AI 每次都正确理解几十个 crate，不如主动缩小它可以使用的 API 表面积。

当然，这并不能消除幻觉。它只是把错误从“任意猜测第三方 API”收敛为“在一个有限工具集合里选择”，再交给 Rust 编译器完成最后校验。

## Facade 的代价

这套设计并不适合所有场景，也不准备取代底层 crate。

首先，facade 只能覆盖高频能力。如果需要 `reqwest` 的连接池细节、`chrono` 的全部日期类型或图片库的高级编码参数，直接依赖底层 crate 更合理。

其次，`extra` 会引入网络、图片、Excel、SMTP 等较重依赖。项目通过 feature 将它与标准工具分开，但编译时间和二进制体积仍然是需要持续测量的问题。

再次，稳定 facade 意味着维护者需要认真对待命名、兼容性和错误语义。一旦上层业务广泛依赖这些入口，随意改名反而会放大迁移成本。

最后，context 适配仍在完善。目前标准工具已经全部覆盖，扩展工具中 `cron`、`proxy`、`server` 和 `watcher` 还没有 context 入口。审计描述与具体日志后端的集成也需要继续建设。

## 接下来准备做什么

我们计划继续补充以下内容：

1. 为 facade 增加 API 兼容性和 compile-fail 测试；
2. 测量不同 feature 组合的编译时间与二进制体积；
3. 完善异步 IO 和剩余 context 适配；
4. 将意图描述接入 TeaQL runtime 的审计与链路追踪；
5. 根据真实业务使用情况收缩或调整工具 API，而不是单纯追求数量。

项目地址：<https://github.com/teaql/teaql-rust-utils>

我们也很想听听 Rust 社区的意见：Rust 项目是否需要类似 Hutool 的统一工具门面？这种设计降低了业务复杂度，还是隐藏了原本清晰的 crate 边界？对于 AI 生成代码，稳定的小型 API 是否真的比直接使用底层生态更有效？

这些问题，可能比“还应该再加哪个工具”更值得讨论。
