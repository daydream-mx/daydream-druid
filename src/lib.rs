use druid::{
    widget::ViewSwitcher, AppLauncher, Data, Lens, LocalizedString, PlatformError, Selector, Size,
    Widget, WindowDesc,
};
use matrix_sdk::Client;
use once_cell::sync::OnceCell;
use views::login::login_ui;
use views::main::main_ui;

mod matrix;
mod utils;
mod views;

// This wrapper function is the primary modification we're making to the vanilla desktop
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    rmain();
}

/////////////////////////////
//// START OF ACTUAL APP ////
/////////////////////////////

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Daydream");

const SET_VIEW: Selector<View> = Selector::new("event-daydream.set-view");

// Replace with https://docs.rs/once_cell/1.4.1/once_cell/#lazy-initialized-global-data
static CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Clone, Copy, Data, PartialEq, Debug)]
enum View {
    LoginView,
    MainView,
}

// Do not derive as we want more control
impl Default for View {
    fn default() -> Self {
        View::LoginView
    }
}

#[derive(Clone, Data, Lens, Default)]
pub struct AppState {
    homeserver: String,
    mxid: String,
    password: String,
    access_token: Option<String>,
    login_running: bool,
    current_view: View,
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .with_min_size(Size::new(800.0, 400.0))
        .title(WINDOW_TITLE);

    // create the initial app state
    let initial_state = AppState::default();
    let delegate = utils::Delegate {};
    if cfg!(debug_assertions) {
        AppLauncher::with_window(main_window)
            .delegate(delegate)
            .use_simple_logger()
            .launch(initial_state)
    } else {
        AppLauncher::with_window(main_window)
            .delegate(delegate)
            .launch(initial_state)
    }
}

fn ui_builder() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            View::LoginView => Box::new(login_ui()),
            View::MainView => Box::new(main_ui()),
            _ => panic!("wrong state"),
        },
    )
}
