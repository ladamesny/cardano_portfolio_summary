pub mod account;
pub mod top_nft_positions;
pub mod watch_list;
pub mod common;

#[derive(Clone, PartialEq)]
pub enum Page {
    TopNftPositions,
    WatchList,
    Account,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccountFocus {
    Menu,
    Content,
}
