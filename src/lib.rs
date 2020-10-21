#![feature(iter_order_by)]
#![type_length_limit = "1446569"]

use crate::matrix::room::EventListAsynSyncLogic;
use crochet::{AppHolder, Cx, DruidAppData, List, ListData};
use druid::{
    AppLauncher, Data, LocalizedString, PlatformError, Selector, Size, Widget, WindowDesc,
};
use matrix_sdk::{events::AnySyncMessageEvent, identifiers::RoomId, locks::Mutex, Room};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use utils::EventListAppedStruct;
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

const WINDOW_TITLE: LocalizedString<DruidAppData> = LocalizedString::new("Daydream");

const SET_VIEW: Selector<View> = Selector::new("event-daydream.set-view");
pub const APPEND_ROOMLIST: Selector<Vec<Room>> = Selector::new("event-daydream.append-roomlist");
pub const REMOVE_ROOMLIST_ITEMS: Selector<Vec<Room>> =
    Selector::new("event-daydream.remove-roomlist-items");
pub const FORCE_RERENDER: Selector<()> = Selector::new("event-daydream.force-rerender");

pub const SWITCH_ROOM: Selector<RoomId> = Selector::new("event-daydream.switch-room");

pub const SET_EVENTLIST: Selector<Vec<AnySyncMessageEvent>> =
    Selector::new("event-daydream.set-eventlist");
pub const APPEND_EVENTLIST: Selector<EventListAppedStruct> =
    Selector::new("event-daydream.append-eventlist");

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
            Some(_session) => View::MainView,
            None => View::LoginView,
        }
    }
}

#[derive(Default)]
pub struct AppState<'a> {
    homeserver: Arc<Cow<'a, str>>,
    mxid: Arc<Cow<'a, str>>,
    password: Arc<Cow<'a, str>>,
    access_token: Option<Cow<'a, str>>,
    login_running: bool,
    current_view: View,

    rooms_list_data: ListData<Arc<Room>>,
    events_list_data: ListData<Arc<AnySyncMessageEvent>>,
    rooms_list: List,
    events_list: List,

    new_message: Cow<'a, str>,
    current_room: Cow<'a, str>,
}

impl AppState<'_> {
    fn run(&mut self, cx: &mut Cx) {
        // The syntax matters here.
        match self.current_view {
            View::LoginView => {
                login_ui(cx, self);
            }
            View::MainView => {
                main_ui(cx, self);
            }
        }
    }
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .window_size(Size::new(800.0, 600.0))
        .title(WINDOW_TITLE);

    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();
    if EVENT_SINK.set(event_sink).is_err() {
        panic!();
    }
    let data = Default::default();
    launcher.launch(data)
}

fn ui_builder() -> impl Widget<DruidAppData> {
    let mut app_logic = AppState::default();

    AppHolder::new(move |cx| app_logic.run(cx))
}
