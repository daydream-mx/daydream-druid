use crate::utils::switch_room_helper;
use crate::AppState;
use crochet::{Button, Column, Cx, Id, Label, Row, TextBox};
use matrix_sdk::events::{
    room::message::{MessageEventContent, TextMessageEventContent},
    AnySyncMessageEvent, SyncMessageEvent,
};
use std::borrow::Cow;

pub fn main_ui(cx: &mut Cx, state: &mut AppState) {
    // RELOGIN if required (Hack but we know that the room list only exists if we are in the Main View ^^)
    // TODO force to run once
    //crate::matrix::login::relogin(crate::EVENT_SINK.get().unwrap().clone());
    Row::new().build(cx, |cx| {
        state.rooms_list.run(
            cx,
            &state.rooms_list_data,
            |cx, _is_selected, _id: Id, room| {
                if Button::new(room.display_name()).build(cx) {
                    // TODO fix
                    //state.current_room = Cow::from(room.room_id.to_string());
                    switch_room_helper(room.room_id.clone());
                }
            },
        );

        Column::new().build(cx, |cx| {
            state.events_list.run(
                cx,
                &state.events_list_data,
                |cx, _is_selected, _id: Id, event| {
                    let mut event_content = {
                        #[allow(clippy::single_match)]
                        match **event {
                            AnySyncMessageEvent::RoomMessage(ref event) => {
                                if let SyncMessageEvent {
                                    content:
                                        MessageEventContent::Text(TextMessageEventContent {
                                            body: msg_body,
                                            ..
                                        }),
                                    sender,
                                    ..
                                } = event
                                {
                                    format!("<{}>: {}", sender, msg_body)
                                } else {
                                    "".to_string()
                                }
                            }
                            _ => "".to_string(),
                        }
                    };

                    Label::new(event_content).build(cx);
                },
            );
        });

        if let Some(content) = TextBox::new(state.new_message.to_string()).build(cx) {
            state.new_message = Cow::from(content);
        }
    });
}
