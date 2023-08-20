pub use cancel_all_orders::*;
pub use cancel_and_place_orders::*;
pub use cancel_order::*;
pub use cancel_order_by_client_order_id::*;
pub use close_market::*;
pub use close_open_orders_account::*;
pub use close_open_orders_indexer::*;
pub use consume_events::*;
pub use create_market::*;
pub use create_open_orders_account::*;
pub use create_open_orders_indexer::*;
pub use deposit::*;
pub use place_order::*;
pub use place_take_order::*;
pub use prune_orders::*;
pub use set_delegate::*;
pub use set_market_expired::*;
pub use settle_funds::*;
pub use settle_funds_expired::*;
pub use stub_oracle_close::*;
pub use stub_oracle_create::*;
pub use stub_oracle_set::*;
pub use sweep_fees::*;

mod cancel_all_orders;
mod cancel_and_place_orders;
mod cancel_order;
mod cancel_order_by_client_order_id;
mod close_market;
mod close_open_orders_account;
mod close_open_orders_indexer;
mod consume_events;
mod create_market;
mod create_open_orders_account;
mod create_open_orders_indexer;
/*  edit orders is missing
t=0 place bid b: 5 BTC @ 27k
t=1 sign transaction t2 to cancel bid b, place bid c: 5 BTC @ 28k
	cancel_order_size = 5 BTC
t=2 fill b for 1 BTC @ 27k
t=3 execute t2: cancel b, place c: 4 BTC @ 28k
	remaining_cancel_size = 4BTC
	 filled_amount = 1BTC
t=4 fill c for 4 BTC @ 28k


t=0 place bid b: 5 BTC @ 27k
t=1 sign transaction t2 to cancel bid b, place bid c: 10 BTC @ 27k
	cancel_order_size = 5 BTC
t=2 fill b for 1 BTC @ 27k
t=3 execute t2: cancel b, place c: 9 BTC @ 27k
	remaining_cancel_size = 4BTC
	filled_amount = 1BTC
t=4 fill c for 9 BTC @ 27k
*/

mod deposit;
mod place_order;
mod place_take_order;
mod prune_orders;
mod set_delegate;
mod set_market_expired;
mod settle_funds;
mod settle_funds_expired;
mod stub_oracle_close;
mod stub_oracle_create;
mod stub_oracle_set;
mod sweep_fees;
