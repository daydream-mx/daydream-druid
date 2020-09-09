use super::sync::EventCallback;
use crate::matrix::room::RoomListAsynSyncLogic;
use druid::Target;
use matrix_sdk::{Client, ClientConfig, Session as SDKSession, SyncSettings};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

pub fn relogin(sink: druid::ExtEventSink) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            wasm_bindgen_futures::spawn_local(async move {
                relogin_real(sink).await;
            });
        } else {
            tokio::spawn(async move {
                relogin_real(sink).await;
            });
        }
    }
}

async fn relogin_real(sink: druid::ExtEventSink) {
    if let Some(session) = Session::load() {
        {
            println!("Starting relogin");
            create_client(session.homeserver);
            let locked_client = crate::CLIENT.get().unwrap().lock().await;

            let session = SDKSession {
                access_token: session.access_token,
                device_id: session.device_id.into(),
                user_id: matrix_sdk::identifiers::UserId::try_from(session.user_id.as_str())
                    .unwrap(),
            };

            if let Err(e) = locked_client.restore_login(session).await {
                eprintln!("{}", e);
            };
            println!("Finished relogin");
            sink.submit_command(crate::SET_VIEW, crate::View::MainView, Target::Auto)
                .expect("command failed to submit");
        }
        start_sync(sink).await;
    }
}

pub fn create_client(homeserver: String) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            let client_config = ClientConfig::new();
        } else {
            let mut data_dir = dirs::data_dir().unwrap();
            data_dir.push("daydream/store");
            let client_config = ClientConfig::new().store_path(data_dir);
        }
    }

    let homeserver_url = Url::parse(&homeserver).unwrap();
    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    crate::CLIENT.get_or_init(|| Mutex::new(client.clone()));
}

pub fn login(sink: druid::ExtEventSink, mxid: String, password: String) {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            wasm_bindgen_futures::spawn_local(async move {
                login_real(sink, mxid, password).await;
            });
        } else {
            tokio::spawn(async move {
                login_real(sink, mxid, password).await;
            });
        }
    }
}

async fn login_real(sink: druid::ExtEventSink, mxid: String, password: String) {
    {
        let locked_client = crate::CLIENT.get().unwrap().lock().await;
        let login_response = locked_client
            .login(&mxid, &password, None, Some("Daydream druid"))
            .await;

        if let Ok(login_response) = login_response {
            let session = Session {
                homeserver: locked_client.homeserver().to_string(),
                user_id: login_response.user_id.to_string(),
                access_token: login_response.access_token,
                device_id: login_response.device_id.into(),
            };
            session.save();
        }
        println!("Login done!");
        sink.submit_command(crate::SET_VIEW, crate::View::MainView, Target::Auto)
            .expect("command failed to submit");
    }
    start_sync(sink).await;
}

pub async fn start_sync(sink: druid::ExtEventSink) {
    if crate::ROOM_TO_EVENTS_MAP
        .set(Mutex::new(HashMap::new()))
        .is_err()
    {
        panic!();
    }

    let room_list_logic = Arc::new(Mutex::new(RoomListAsynSyncLogic::default()));
    {
        let mut locked_client = crate::CLIENT.get().unwrap().lock().await;
        println!("StartSync");

        locked_client
            .add_event_emitter(Box::new(EventCallback {
                sink,
                room_list_logic: room_list_logic.clone(),
            }))
            .await;

        let client: Client = locked_client.clone();

        tokio::spawn(async move {
            match client.clone().sync_token().await {
                Some(token) => {
                    let sync_settings = SyncSettings::new().token(token);
                    client
                        .clone()
                        .sync_forever(sync_settings, |_| async {})
                        .await;
                }
                None => {
                    let sync_settings = SyncSettings::new();
                    client
                        .clone()
                        .sync_forever(sync_settings, |_| async {})
                        .await;
                }
            }
        });
        println!("After sync");
    }
    // Get the cache once
    room_list_logic.lock().await.update_data().await;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    /// The homeserver used for this session.
    pub homeserver: String,
    /// The access token used for this session.
    pub access_token: String,
    /// The user the access token was issued for.
    pub user_id: String,
    /// The ID of the client device
    pub device_id: String,
}

impl Session {
    pub fn save(&self) {
        cfg_if::cfg_if! {
            if #[cfg(any(target_arch = "wasm32"))] {
                // TODO. No-OP for now
            } else {
                let mut data_dir = dirs::data_dir().unwrap();
                data_dir.push("daydream/session.json");
                serde_json::to_writer(&std::fs::File::create(data_dir).unwrap(), self).unwrap();
            }
        }
    }

    pub fn load() -> Option<Self> {
        cfg_if::cfg_if! {
            if #[cfg(any(target_arch = "wasm32"))] {
                // TODO. No-OP for now
                None
            } else {
                let mut data_dir = dirs::data_dir().unwrap();
                data_dir.push("daydream/session.json");
                let file = std::fs::File::open(data_dir);
                match file {
                    Ok(file) => {
                        let session: Result<Self,serde_json::Error> = serde_json::from_reader(&file);
                        match session {
                            Ok(session) => Some(session),
                            Err(_) => None,
                        }
                    }
                    Err(_) => None
                }
            }
        }
    }
}
