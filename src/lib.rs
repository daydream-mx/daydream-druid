use druid::{
    widget::ViewSwitcher, AppLauncher, Data, Lens, LocalizedString, PlatformError, Selector, Size,
    Widget, WindowDesc,
};
use matrix::room::RoomList;
use matrix_sdk::Client;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::Mutex;
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

static CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();

#[derive(Clone, Copy, Data, PartialEq, Debug)]
enum View {
    LoginView,
    MainView,
}

// Do not derive as we want more control
impl Default for View {
    fn default() -> Self {
        match matrix::login::Session::load() {
            Some(_session) => {
                // TODO relogin
                View::MainView
            }
            None => View::LoginView,
        }
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

    // TODO proper types
    rooms_list: RoomList,
    events_list: Arc<Vec<u32>>,

    new_message: String,
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .with_min_size(Size::new(800.0, 400.0))
        .title(WINDOW_TITLE);

    // create the initial app state
    let mut initial_state = AppState::default();

    initial_state.events_list = Arc::new(vec![1, 2, 3, 4, 5, 6]);
    let delegate = utils::Delegate {};

    AppLauncher::with_window(main_window)
        .delegate(delegate)
        .launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            View::LoginView => Box::new(login_ui()),
            View::MainView => Box::new(main_ui()),
        },
    )
}
