use crate::AppState;
use druid::{
    widget::{Flex, Label},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Target, Widget, WidgetExt,
};

use matrix_sdk::{events::AnySyncMessageEvent, identifiers::RoomId};

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(view) = cmd.get(crate::SET_VIEW) {
            data.login_running = false;

            // Clear password from memory
            data.password = "".into();

            // Change View
            data.current_view = *view;

            println!("Set View to {:?}", view);
        }
        true
    }
}

pub fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new(label).fix_height(40.0))
        .with_spacer(8.0)
        .with_flex_child(widget.expand_width(), 1.0)
        .with_spacer(8.0)
        .border(Color::WHITE, 1.0)
}

pub struct EventListAppedStruct {
    pub room_id: RoomId,
    pub events: Vec<AnySyncMessageEvent>,
}

pub fn switch_room_helper(room_id: RoomId) {
    tokio::spawn(async move {
        let room_to_events_map = crate::ROOM_TO_EVENTS_MAP.get().unwrap().lock().await;
        let event_list_logic = room_to_events_map[&room_id.to_string()].lock().await;
        event_list_logic.switch_room();
    });
}
