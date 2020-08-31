use matrix_sdk::{
    events::{
        room::message::MessageEventContent, AnyPossiblyRedactedSyncMessageEvent,
        AnySyncMessageEvent, SyncMessageEvent,
    },
    Room,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RoomListAsynSyncLogic {
    data_cache: Vec<Room>,
    fetching_rooms: Arc<AtomicBool>,
}

impl Default for RoomListAsynSyncLogic {
    fn default() -> Self {
        RoomListAsynSyncLogic {
            data_cache: vec![],
            fetching_rooms: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl RoomListAsynSyncLogic {
    pub fn add_room_if_new(&mut self, new_room: Room) {
        if !self
            .data_cache
            .iter()
            .any(|room| room.room_id == new_room.room_id)
            && new_room.tombstone.is_none()
        {
            self.data_cache.push(new_room.clone());
            let subset_vec = vec![new_room];
            crate::EVENT_SINK
                .get()
                .unwrap()
                .clone()
                .submit_command(crate::APPEND_ROOMLIST, subset_vec, None)
                .expect("command failed to submit");
        }
    }

    pub fn remove_room(&mut self, room: Room) {
        if let Some(index) = self
            .data_cache
            .iter()
            .position(|lroom| lroom.room_id == room.room_id)
        {
            self.data_cache.remove(index);

            let subset_vec = vec![room];
            crate::EVENT_SINK
                .get()
                .unwrap()
                .clone()
                .submit_command(crate::REMOVE_ROOMLIST_ITEMS, subset_vec, None)
                .expect("command failed to submit");
        }
    }

    async fn get_rooms_from_client(&mut self) {
        if let Some(available_client) = crate::CLIENT.get() {
            if let Ok(locked_client) = available_client.try_lock() {
                // TODO consider showing other types (this might involve refactoring)
                let rooms = locked_client.joined_rooms();

                let hashmap_rooms = rooms.read().await;

                let mut room_to_events_map = crate::ROOM_TO_EVENTS_MAP.get().unwrap().lock().await;

                let mut new_list: Vec<Room> = Vec::with_capacity(hashmap_rooms.len());
                for value in hashmap_rooms.values() {
                    {
                        //println!("pre read_clone");
                        let clean_room = value.read().await.clone();

                        // TODO check if we are in the new room and if not display a way to get to the new one
                        if clean_room.tombstone.is_none() {
                            new_list.push(clean_room.clone());
                            let mut event_list_async_sync_logic = EventListAsynSyncLogic::default();
                            event_list_async_sync_logic
                                .update_data(clean_room.clone())
                                .await;
                            room_to_events_map.insert(
                                clean_room.room_id.to_string(),
                                Mutex::new(event_list_async_sync_logic),
                            );
                        }
                    }
                }

                if !new_list.eq(&self.data_cache) {
                    self.data_cache = new_list.clone();

                    crate::EVENT_SINK
                        .get()
                        .unwrap()
                        .clone()
                        .submit_command(crate::APPEND_ROOMLIST, new_list, None)
                        .expect("command failed to submit");
                }
            } else {
                eprintln!("Failed to aquire client");
            }
        }
    }

    /// Get the actual data of the roomlist from the matrix Client global
    pub async fn update_data(&mut self) {
        if !self.fetching_rooms.swap(true, Ordering::Acquire) {
            self.get_rooms_from_client().await;
            self.fetching_rooms.store(false, Ordering::Release);
        }
    }
}

#[derive(Clone)]
pub struct EventListAsynSyncLogic {
    data_cache: Vec<AnySyncMessageEvent>,
    fetching_events: Arc<AtomicBool>,
}

impl Default for EventListAsynSyncLogic {
    fn default() -> Self {
        EventListAsynSyncLogic {
            data_cache: vec![],
            fetching_events: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl EventListAsynSyncLogic {
    // Abstract this for other types of messages
    pub fn add_event_if_new(&mut self, new_event: SyncMessageEvent<MessageEventContent>) {
        if !self
            .data_cache
            .iter()
            .any(|event| event.event_id() == &new_event.event_id)
        {
            self.data_cache
                .push(AnySyncMessageEvent::RoomMessage(new_event.clone()));
            let subset_vec = vec![AnySyncMessageEvent::RoomMessage(new_event)];
            crate::EVENT_SINK
                .get()
                .unwrap()
                .clone()
                .submit_command(crate::APPEND_EVENTLIST, subset_vec, None)
                .expect("command failed to submit");
        }
    }

    // TODO redactions

    async fn get_events_from_room(&mut self, room: Room) {
        let new_list: Vec<AnySyncMessageEvent> = room
            .messages
            .iter()
            .filter_map(|d| match d {
                AnyPossiblyRedactedSyncMessageEvent::Regular(event) => Some(event.to_owned()),
                _ => None,
            })
            .collect();

        if !self
            .data_cache
            .iter()
            .eq_by(&new_list, |x, y| x.event_id() == y.event_id())
        {
            self.data_cache = new_list.clone();

            crate::EVENT_SINK
                .get()
                .unwrap()
                .clone()
                .submit_command(crate::APPEND_EVENTLIST, new_list, None)
                .expect("command failed to submit");
        }
    }

    /// Get the actual data of the roomlist from the matrix Client global
    pub async fn update_data(&mut self, room: Room) {
        if !self.fetching_events.swap(true, Ordering::Acquire) {
            self.get_events_from_room(room).await;
            self.fetching_events.store(false, Ordering::Release);
        }
    }
}
