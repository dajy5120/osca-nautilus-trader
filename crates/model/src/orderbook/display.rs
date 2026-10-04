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

//! Functions related to order book display.
//! 【zh】 把订单簿渲染为人类可读的表格（调试与日志用），可按 `group_size` 合并价位。

use rust_decimal::Decimal;
use tabled::{builder::Builder, settings::Style};

use super::{BookPrice, level::BookLevel, own::OwnBookLevel};
use crate::{
    enums::OrderSideSpecified,
    orderbook::{OrderBook, own::OwnOrderBook},
};

// 【zh】 订单簿表格中的一行展示数据，对应表格的“买盘 / 价格 / 卖盘”三列。
// 【zh】 三个字段都已格式化为字符串；买盘或卖盘一侧没有数据时为空字符串。
struct BookLevelDisplay {
    bids: String,
    price: String,
    asks: String,
}

/// Return a [`String`] representation of the order book in a human-readable table format.
/// 【zh】 把订单簿（Order Book）渲染为带头部统计信息的圆角表格字符串，用于调试与日志。
/// 【zh】 卖盘在上、买盘在下，各取最优的 `num_levels` 档，并先对卖盘做 `rev` 反转，
/// 【zh】 这样价格从上到下递减，最优卖价与最优买价在表格中间相邻。
/// 【zh】 传入 `group_size` 时按该粒度合并价位，只显示每个价格的总量；
/// 【zh】 否则逐档显示，并列出该档内每笔订单的数量（形如 `[a, b]`）。
/// 【zh】 头部包含买卖档位总数（不受 `num_levels` 限制）、序列号、更新次数和最近时间戳。
#[must_use]
#[expect(clippy::needless_collect)] // Collect needed for .rev() and .chain()
pub(crate) fn pprint_book(
    order_book: &OrderBook,
    num_levels: usize,
    group_size: Option<Decimal>,
) -> String {
    let data: Vec<BookLevelDisplay> = if let Some(group_size) = group_size {
        let bid_quantities = order_book.group_bids(group_size, Some(num_levels));
        let ask_quantities = order_book.group_asks(group_size, Some(num_levels));

        // 【zh】 合并价位时按 `group_size` 的小数位数格式化价格，使各行价格位数一致。
        // Use the precision of the group_size for consistent formatting
        let precision = group_size.scale();

        let mut data = Vec::new();

        // Add ask levels (highest to lowest)
        for (price, qty) in ask_quantities.iter().rev() {
            data.push(BookLevelDisplay {
                bids: String::new(),
                price: format!("{price:.precision$}", precision = precision as usize),
                asks: qty.to_string(),
            });
        }

        // Add bid levels (highest to lowest)
        for (price, qty) in &bid_quantities {
            data.push(BookLevelDisplay {
                bids: qty.to_string(),
                price: format!("{price:.precision$}", precision = precision as usize),
                asks: String::new(),
            });
        }

        data
    } else {
        let ask_levels: Vec<(&BookPrice, &BookLevel)> = order_book
            .asks
            .levels
            .iter()
            .take(num_levels)
            .rev()
            .collect();
        let bid_levels: Vec<(&BookPrice, &BookLevel)> =
            order_book.bids.levels.iter().take(num_levels).collect();
        let levels: Vec<(&BookPrice, &BookLevel)> =
            ask_levels.into_iter().chain(bid_levels).collect();

        levels
            .iter()
            .map(|(book_price, level)| {
                let is_bid_level = book_price.side == OrderSideSpecified::Buy;
                let is_ask_level = book_price.side == OrderSideSpecified::Sell;

                let bid_sizes: Vec<String> = level
                    .orders
                    .iter()
                    .filter(|_| is_bid_level)
                    .map(|order| format!("{}", order.1.size))
                    .collect();

                let ask_sizes: Vec<String> = level
                    .orders
                    .iter()
                    .filter(|_| is_ask_level)
                    .map(|order| format!("{}", order.1.size))
                    .collect();

                BookLevelDisplay {
                    bids: if bid_sizes.is_empty() {
                        String::new()
                    } else {
                        format!("[{}]", bid_sizes.join(", "))
                    },
                    price: format!("{}", level.price),
                    asks: if ask_sizes.is_empty() {
                        String::new()
                    } else {
                        format!("[{}]", ask_sizes.join(", "))
                    },
                }
            })
            .collect()
    };

    let table = render_book_levels(data);

    let header = format!(
        "bid_levels: {}\nask_levels: {}\nsequence: {}\nupdate_count: {}\nts_last: {}",
        order_book.bids.levels.len(),
        order_book.asks.levels.len(),
        order_book.sequence,
        order_book.update_count,
        order_book.ts_last,
    );

    format!("{header}\n{table}")
}

/// Return a [`String`] representation of the own order book in a human-readable table format.
/// 【zh】 把自有订单簿（`OwnOrderBook`）渲染为表格字符串，布局与 `pprint_book` 相同。
/// 【zh】 区别在于数据来自自有订单，且头部不含 `sequence` 字段。
/// 【zh】 分组模式下的数量查询不启用任何过滤，因此展示的是全部自有订单。
#[must_use]
#[expect(clippy::needless_collect)] // Collect needed for .rev() and .chain()
pub(crate) fn pprint_own_book(
    own_order_book: &OwnOrderBook,
    num_levels: usize,
    group_size: Option<Decimal>,
) -> String {
    let data: Vec<BookLevelDisplay> = if let Some(group_size) = group_size {
        // Rendering is membership-neutral, so acceptance-time filtering stays disabled.
        let bid_quantities =
            own_order_book.bid_quantity(None, Some(num_levels), Some(group_size), None, None);
        let ask_quantities =
            own_order_book.ask_quantity(None, Some(num_levels), Some(group_size), None, None);

        // Use the precision of the group_size for consistent formatting
        let precision = group_size.scale();

        let mut data = Vec::new();

        // Add ask levels (highest to lowest)
        for (price, qty) in ask_quantities.iter().rev() {
            data.push(BookLevelDisplay {
                bids: String::new(),
                price: format!("{price:.precision$}", precision = precision as usize),
                asks: qty.to_string(),
            });
        }

        // Add bid levels (highest to lowest)
        for (price, qty) in &bid_quantities {
            data.push(BookLevelDisplay {
                bids: qty.to_string(),
                price: format!("{price:.precision$}", precision = precision as usize),
                asks: String::new(),
            });
        }

        data
    } else {
        let ask_levels: Vec<(&BookPrice, &OwnBookLevel)> = own_order_book
            .asks
            .levels
            .iter()
            .take(num_levels)
            .rev()
            .collect();
        let bid_levels: Vec<(&BookPrice, &OwnBookLevel)> =
            own_order_book.bids.levels.iter().take(num_levels).collect();
        let levels: Vec<(&BookPrice, &OwnBookLevel)> =
            ask_levels.into_iter().chain(bid_levels).collect();

        levels
            .iter()
            .map(|(book_price, level)| {
                let is_bid_level = book_price.side == OrderSideSpecified::Buy;
                let is_ask_level = book_price.side == OrderSideSpecified::Sell;

                let bid_sizes: Vec<String> = level
                    .orders
                    .iter()
                    .filter(|_| is_bid_level)
                    .map(|order| format!("{}", order.1.size))
                    .collect();

                let ask_sizes: Vec<String> = level
                    .orders
                    .iter()
                    .filter(|_| is_ask_level)
                    .map(|order| format!("{}", order.1.size))
                    .collect();

                BookLevelDisplay {
                    bids: if bid_sizes.is_empty() {
                        String::new()
                    } else {
                        format!("[{}]", bid_sizes.join(", "))
                    },
                    price: format!("{}", level.price),
                    asks: if ask_sizes.is_empty() {
                        String::new()
                    } else {
                        format!("[{}]", ask_sizes.join(", "))
                    },
                }
            })
            .collect()
    };

    let table = render_book_levels(data);

    let header = format!(
        "bid_levels: {}\nask_levels: {}\nupdate_count: {}\nts_last: {}",
        own_order_book.bids.levels.len(),
        own_order_book.asks.levels.len(),
        own_order_book.update_count,
        own_order_book.ts_last,
    );

    format!("{header}\n{table}")
}

// 【zh】 用 `tabled` 把若干行数据构建成三列（bids、price、asks）的圆角样式表格，
// 【zh】 表头固定为首行，返回最终字符串。
fn render_book_levels(data: Vec<BookLevelDisplay>) -> String {
    let mut builder = Builder::with_capacity(data.len() + 1, 3);
    builder.push_record(["bids", "price", "asks"]);

    for level in data {
        builder.push_record([level.bids, level.price, level.asks]);
    }

    builder.build().with(Style::rounded()).to_string()
}
