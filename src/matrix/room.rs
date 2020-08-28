use druid::{widget::ListIter, Data};
use matrix_sdk::Room;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RoomList {
    data_cache: Arc<Mutex<Vec<Arc<Room>>>>,
    fetching_rooms: Arc<AtomicBool>,
}

impl Default for RoomList {
    fn default() -> Self {
        // RELOGIN if required (Hack but we know that the room list only exists if we are in the Main View ^^)
        crate::matrix::login::relogin();

        let data = Arc::new(Mutex::new(vec![]));
        RoomList {
            data_cache: data,
            fetching_rooms: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl RoomList {
    async fn get_rooms_from_client(data: Arc<Mutex<Vec<Arc<Room>>>>) {
        if let Some(available_client) = crate::CLIENT.get() {
            if let Ok(locked_client) = available_client.try_lock() {
                // TODO consider showing other types (this might involve refactoring)
                let rooms = locked_client.joined_rooms();

                let hashmap_rooms = rooms.read().await;
                println!("hashmap_rooms locked");
                if let Ok(mut data) = data.try_lock() {
                    println!("data locked");

                    for value in hashmap_rooms.values() {
                        {
                            println!("pre read_clone");
                            let read_clone = value.read().await;
                            println!("value: {}", read_clone.display_name());
                            let clean_room = (*read_clone).clone();
    
                            data.push(Arc::new(clean_room));
                        }
                    }
                    // TODO smarter dedup
                    data.dedup();
                    println!("Done with rooms inner");
                }
                println!("Done with rooms outer");
            }
        }
    }

    /// Get the actual data of the roomlist from the matrix Client global
    // While this also would work as a sttic method this should only get called in the context of ListIter
    fn data(&self, other: bool) -> Option<Vec<Arc<Room>>> {
        if !other && !self.fetching_rooms.swap(true, Ordering::Acquire) {
            let self_cache_data_clone = self.data_cache.clone();
            futures::executor::block_on(async move {
                RoomList::get_rooms_from_client(self_cache_data_clone).await;
                // FIXME we should really force a rerender
            });
            self.fetching_rooms.store(false, Ordering::Release);
        }

        if let Ok(data_cache) = self.data_cache.try_lock() {
            if !data_cache.is_empty() {
                println!("data_cache_first: {}", data_cache[0].display_name());
                return Some(data_cache.clone());
            }
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
            if let Some(self_data) = self_data {
                if let Some(other_data) = other_data {
                    let same = self_data == other_data;
                    return same;
                } else {
                    return false;
                }
            }

            if let Some(other_data) = other_data {
                if let Some(self_data) = self_data {
                    let same = self_data == other_data;
                    return same;
                } else {
                    return false;
                }
            }
        } else if self_data.is_none() == other_data.is_none() {
            // this compares if they are both are none
            return true;
        }
        // Fallback to true to redurce renders
        true
    }
}

impl ListIter<Arc<Room>> for RoomList {
    fn for_each(&self, mut cb: impl FnMut(&Arc<Room>, usize)) {
        // Todo provide method to check emptiness without copying the whole list.
        if let Some(data_cache) = self.data(true) {
            if !data_cache.is_empty() {
                for (i, item) in data_cache.iter().enumerate() {
                    cb(item, i);
                }
            }
        }
    }

    fn for_each_mut(&mut self, mut cb: impl FnMut(&mut Arc<Room>, usize)) {
        //let mut new_data = Vec::with_capacity(self.data_len());
        //let mut any_changed = false;

        if let Some(data_cache) = self.data(true) {
            for (i, item) in data_cache.iter().enumerate() {
                let mut d = item.to_owned();
                cb(&mut d, i);

                /*
                // The sdk should handle this already
                if !any_changed && !item.same(&d) {
                    any_changed = true;
                }
                new_data.push(d);*/
            }
        }

        /*if any_changed {
            *self = Arc::new(new_data);
        }*/
    }

    fn data_len(&self) -> usize {
        if let Some(data_cache) = self.data(true) {
            return data_cache.len();
        }
        0
    }
}
