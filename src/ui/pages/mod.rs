pub mod account;
pub mod positions;
pub mod watch_list;
pub mod common;

#[derive(Clone, PartialEq)]
pub enum Page {
    Positions,
    WatchList,
    Account,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccountFocus {
    Menu,
    Content,
}
