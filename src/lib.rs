#![feature(iter_order_by)]

use crate::matrix::room::EventListAsynSyncLogic;
use druid::{
    widget::ViewSwitcher, AppLauncher, Data, Lens, LocalizedString, PlatformError, Selector, Size,
    Widget, WindowDesc,
};
use matrix_sdk::{events::AnySyncMessageEvent, identifiers::RoomId, Client, Room};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
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
pub const APPEND_ROOMLIST: Selector<Vec<Room>> = Selector::new("event-daydream.append-roomlist");
pub const REMOVE_ROOMLIST_ITEMS: Selector<Vec<Room>> =
    Selector::new("event-daydream.remove-roomlist-items");
pub const FORCE_RERENDER: Selector<()> = Selector::new("event-daydream.force-rerender");

pub const SWITCH_ROOM: Selector<RoomId> = Selector::new("event-daydream.switch-room");

pub const APPEND_EVENTLIST: Selector<Vec<AnySyncMessageEvent>> =
    Selector::new("event-daydream.append-eventlist");

static CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();

static ROOM_TO_EVENTS_MAP: OnceCell<Mutex<HashMap<String, Mutex<EventListAsynSyncLogic>>>> =
    OnceCell::new();

// WARNING this might have bad problems
static EVENT_SINK: OnceCell<druid::ExtEventSink> = OnceCell::new();

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

    rooms_list: Arc<Vec<Arc<Room>>>,
    events_list: Arc<Vec<Arc<AnySyncMessageEvent>>>,

    new_message: String,
    current_room: String,
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .window_size(Size::new(800.0, 600.0))
        .title(WINDOW_TITLE);

    // create the initial app state
    let initial_state = AppState::default();
    let delegate = utils::Delegate {};

    let launcher = AppLauncher::with_window(main_window).delegate(delegate);

    let event_sink = launcher.get_external_handle();
    if EVENT_SINK.set(event_sink).is_err() {
        panic!();
    }
    launcher.launch(initial_state)
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
