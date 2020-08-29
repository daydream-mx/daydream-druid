use crate::matrix::room::RoomListAsynSyncLogic;
use matrix_sdk::{
    events::{
        room::member::MemberEventContent,
        room::message::{MessageEventContent, TextMessageEventContent},
        SyncMessageEvent, SyncStateEvent,
    },
    EventEmitter, SyncRoom,
};
use matrix_sdk_common_macros::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EventCallback {
    pub sink: druid::ExtEventSink,
    pub room_list_logic: Arc<Mutex<RoomListAsynSyncLogic>>,
}

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
    async fn on_room_member(&self, room: SyncRoom, event: &SyncStateEvent<MemberEventContent>) {
        #[allow(irrefutable_let_patterns)]
        if let SyncStateEvent { sender, .. } = event {
            let locked_client = crate::CLIENT.get().unwrap().lock().await;
            if sender == &locked_client.user_id().await.unwrap() {
                match room {
                    SyncRoom::Joined(room) => {
                        let clean_room = room.read().await.clone();

                        self.room_list_logic
                            .lock()
                            .await
                            .add_room_if_new(clean_room);
                    }
                    SyncRoom::Left(room) => {
                        let clean_room = room.read().await.clone();

                        self.room_list_logic.lock().await.remove_room(clean_room);
                    }
                    _ => {
                         //TODO add a way of displaying Invites in the UI. This will need to be done in `on_stripped_state_member`
                    }
                }
            }
        }
    }
}
