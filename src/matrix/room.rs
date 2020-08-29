use matrix_sdk::Room;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
        if self
            .data_cache
            .iter()
            .any(|room| room.room_id == new_room.room_id)
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

                let mut new_list: Vec<Room> = Vec::with_capacity(hashmap_rooms.len());
                for value in hashmap_rooms.values() {
                    {
                        //println!("pre read_clone");
                        let clean_room = value.read().await.clone();

                        new_list.push(clean_room);
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
