// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Order book components which can handle L1/L2/L3 data.
//! 【zh】 订单簿（Order Book）组件，支持 L1 / L2 / L3 三种粒度的市场数据。
//! 【zh】
//! 【zh】 模块分层（自顶向下）：
//! 【zh】 - `book`：`OrderBook`，对外入口。把增量（delta）、10 档深度快照、报价 / 成交 tick 应用到盘口。
//! 【zh】 - `ladder`：`BookLadder`，单侧（买盘或卖盘）的价格阶梯，本质是 `BTreeMap<BookPrice, BookLevel>`。
//! 【zh】 - `level`：`BookLevel`，单个价位，按 FIFO（时间优先）保存该价位上的订单。
//! 【zh】 - `aggregation`：按簿类型预处理订单——L1 / L2 用合成 order_id 实现“同价位只留一个订单”。
//! 【zh】 - `own`：`OwnOrderBook`，自己挂出的订单，可从公共盘口中扣除，得到“别人的流动性”。
//! 【zh】 - `analysis`：均价、可成交量、完整性检查等纯函数。
//! 【zh】 - `display`：盘口的表格化打印。
//! 【zh】 - `error`：完整性错误（`BookIntegrityError`）与非法操作错误（`InvalidBookOperation`）。

pub mod aggregation;
pub mod analysis;
pub mod book;
pub mod display;
pub mod error;
pub mod ladder;
pub mod level;
pub mod own;

#[cfg(test)]
mod tests;

// Re-exports
pub use crate::orderbook::{
    book::OrderBook,
    error::{BookIntegrityError, BookViewError, InvalidBookOperation, OwnBookError},
    ladder::BookPrice,
    level::BookLevel,
    own::OwnBookOrder,
};
