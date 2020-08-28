use druid::{widget::ListIter, Data};
use matrix_sdk::Room;
use std::sync::Arc;

#[derive(Clone)]
pub struct RoomList {
    data_cache: Vec<Arc<Room>>,
}

impl Default for RoomList {
    fn default() -> Self {
        RoomList { data_cache: vec![] }
    }
}

impl RoomList {
    /// Get the actual data of the roomlist from the matrix Client global
    // While this also would work as a sttic method this should only get called in the context of ListIter
    fn data(&self, other: bool) -> Option<Vec<Arc<Room>>> {
        if !other {
            let mut self_cache_data_clone = self.data_cache.clone();
            futures::executor::block_on(async move {
                if let Some(available_client) = crate::CLIENT.get() {
                    let locked_client = available_client.lock().await;

                    // TODO consider showing other types (this might involve refactoring)
                    let rooms = locked_client.joined_rooms();

                    let hashmap_rooms = rooms.read().await;
                    for value in hashmap_rooms.values() {
                        let read_clone = value.read().await;
                        let clean_room = (*read_clone).clone();

                        self_cache_data_clone.push(Arc::new(clean_room));
                    }
                    // TODO smarter dedup
                    self_cache_data_clone.dedup();
                }
            });
        }

        if !self.data_cache.is_empty() {
            return Some(self.data_cache.clone());
        }
        None
    }
}

impl Data for RoomList {
    fn same(&self, other: &Self) -> bool {
        let self_data = self.data(false);
        let other_data = other.data(true);
        // TODO fix this horrible thing
        // Fast exit if empty
        if self_data.is_some() || other_data.is_some() {
            return self_data.unwrap() == other_data.unwrap();
        } else if self_data == other_data {
            // this compares if they are both are none
            return true;
        }
        // Fallback to false to redurce renders
        false
    }
}

impl ListIter<Arc<Room>> for RoomList {
    fn for_each(&self, mut cb: impl FnMut(&Arc<Room>, usize)) {
        if !self.data_cache.is_empty() {
            for (i, item) in self.data_cache.iter().enumerate() {
                cb(item, i);
            }
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Arc<Room>, usize)) {
        //let mut new_data = Vec::with_capacity(self.data_len());
        //let mut any_changed = false;

        for (i, item) in self.data_cache.iter().enumerate() {
            let mut d = item.to_owned();
            cb(&mut d, i);

            /*
            // The sdk should handle this already
            if !any_changed && !item.same(&d) {
                any_changed = true;
            }
            new_data.push(d);*/
        }

        /*if any_changed {
            *self = Arc::new(new_data);
        }*/
    }

    fn data_len(&self) -> usize {
        self.data_cache.len()
    }
}
