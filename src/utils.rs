use crochet::{Cx, Label, Row, TextBox};
use matrix_sdk::{events::AnySyncMessageEvent, identifiers::RoomId};
use std::borrow::Cow;
use std::sync::Arc;

/*pub struct Delegate;

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
}*/

pub fn label_widget(cx: &mut Cx, mut content: Arc<Cow<str>>, label: &str) {
    Row::new().build(cx, |cx| {
        Label::new(label).build(cx);
        if let Some(text) = TextBox::new(content.to_string()).build(cx) {
            content = Arc::from(Cow::from(text));
        }
    });
}

pub struct EventListAppedStruct {
    pub room_id: RoomId,
    pub events: Vec<AnySyncMessageEvent>,
}

pub fn switch_room_helper(room_id: RoomId) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            wasm_bindgen_futures::spawn_local(async move {
                let room_to_events_map = crate::ROOM_TO_EVENTS_MAP.get().unwrap().lock().await;
                let event_list_logic = room_to_events_map[&room_id.to_string()].lock().await;
                event_list_logic.switch_room();
            });
        } else {
            tokio::spawn(async move {
                let room_to_events_map = crate::ROOM_TO_EVENTS_MAP.get().unwrap().lock().await;
                let event_list_logic = room_to_events_map[&room_id.to_string()].lock().await;
                event_list_logic.switch_room();
            });
        }
    }
}
