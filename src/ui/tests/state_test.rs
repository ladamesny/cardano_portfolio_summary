use crate::models::user::User;
use crate::ui::state::{AppState, Page, AccountFocus, PositionsFocus, WatchListFocus};

// Helper function to create a basic AppState for testing
fn create_test_state() -> AppState {
    let user = User {
        id: "test_id".to_string(),
        name: "Test User".to_string(),
        taptools_api_key: "test_key".to_string(),
        wallets: vec![],
    };
    
    let portfolio_json = r#"{
        "numFTs": 0,
        "numNFTs": 0,
        "numLPs": 0,
        "positionsFt": [],
        "positionsNft": [],
        "positionsLp": [],
        "adaBalance": 1000.0,
        "adaValue": 1000.0,
        "liquidValue": 1000.0
    }"#.to_string();

    AppState::new(portfolio_json, user, 0.5, 30000.0)
}

#[test]
fn test_app_state_new() {
    let state = create_test_state();

    assert_eq!(state.current_page, Page::Positions);
    assert_eq!(state.ada_usd_price, 0.5);
    assert_eq!(state.btc_usd_price, 30000.0);
    assert_eq!(state.menu_items.len(), 5);
    assert_eq!(state.focused_menu_items.len(), 2);
}

#[test]
fn test_page_navigation() {
    let mut state = create_test_state();
    
    // Test initial state
    assert_eq!(*state.current_page(), Page::Positions);
    
    // Test navigation to different pages
    state.set_current_page(Page::WatchList);
    assert_eq!(*state.current_page(), Page::WatchList);
    
    state.set_current_page(Page::Account);
    assert_eq!(*state.current_page(), Page::Account);
    
    state.set_current_page(Page::Positions);
    assert_eq!(*state.current_page(), Page::Positions);
}

#[test]
fn test_account_focus() {
    let mut state = create_test_state();
    
    // Test initial focus
    assert_eq!(state.account_focus, AccountFocus::Menu);
    
    // Test focus toggle
    state.toggle_account_focus();
    assert_eq!(state.account_focus, AccountFocus::Content);
    
    state.toggle_account_focus();
    assert_eq!(state.account_focus, AccountFocus::Menu);
    
    // Test reset focus
    state.toggle_account_focus();
    assert_eq!(state.account_focus, AccountFocus::Content);
    state.reset_account_focus();
    assert_eq!(state.account_focus, AccountFocus::Menu);
}

#[test]
fn test_positions_navigation() {
    let mut state = create_test_state();
    
    // Test initial position
    assert_eq!(state.selected_positions_menu_item, 0);
    
    // Test next navigation
    state.next_positions_menu_item();
    assert_eq!(state.selected_positions_menu_item, 1);
    
    state.next_positions_menu_item();
    assert_eq!(state.selected_positions_menu_item, 2);
    
    // Test circular navigation
    state.next_positions_menu_item();
    assert_eq!(state.selected_positions_menu_item, 0);
    
    // Test previous navigation
    state.previous_positions_menu_item();
    assert_eq!(state.selected_positions_menu_item, 2);
}

#[test]
fn test_watch_list_focus() {
    let mut state = create_test_state();
    
    // Test initial focus
    assert_eq!(state.watch_list_focus, WatchListFocus::Menu);
    
    // Test focus toggle
    state.toggle_watch_list_focus();
    assert_eq!(state.watch_list_focus, WatchListFocus::Content);
    
    state.toggle_watch_list_focus();
    assert_eq!(state.watch_list_focus, WatchListFocus::Menu);
}

#[test]
fn test_portfolio_update() {
    let mut state = create_test_state();
    
    let updated_portfolio = r#"{
        "numFTs": 0,
        "numNFTs": 0,
        "numLPs": 0,
        "positionsFt": [],
        "positionsNft": [],
        "positionsLp": [],
        "adaBalance": 2000.0,
        "adaValue": 2500.0,
        "liquidValue": 3000.0
    }"#.to_string();
    
    state.update_portfolio(updated_portfolio);
    
    assert_eq!(state.ada_balance, 2000.0);
    assert_eq!(state.ada_value, 2500.0);
    assert_eq!(state.liquid_value, 3000.0);
}

#[test]
fn test_is_content_focused() {
    let mut state = create_test_state();
    
    // Test initial state (should be false)
    assert!(!state.is_content_focused());
    
    // Test Account page focus
    state.set_current_page(Page::Account);
    state.toggle_account_focus();
    assert!(state.is_content_focused());
    
    // Test Positions page focus
    state.set_current_page(Page::Positions);
    state.toggle_positions_focus();
    assert!(state.is_content_focused());
    
    // Test WatchList page focus
    state.set_current_page(Page::WatchList);
    state.toggle_watch_list_focus();
    assert!(state.is_content_focused());
}
