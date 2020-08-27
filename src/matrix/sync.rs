use matrix_sdk::{
    events::{
        room::message::{MessageEventContent, TextMessageEventContent},
        SyncMessageEvent,
    },
    EventEmitter, SyncRoom,
};
use matrix_sdk_common_macros::async_trait;
pub struct EventCallback;

#[async_trait]
impl EventEmitter for EventCallback {
    async fn on_room_message(&self, room: SyncRoom, event: &SyncMessageEvent<MessageEventContent>) {
        if let SyncRoom::Joined(_room) = room {
            if let SyncMessageEvent {
                content: MessageEventContent::Text(TextMessageEventContent { body: msg_body, .. }),
                sender,
                ..
            } = event
            {
                // TODO actual logic
                println!("<{}>: {}", sender, msg_body);
            }
        }
    }
}
