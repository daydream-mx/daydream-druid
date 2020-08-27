use super::sync::EventCallback;
use matrix_sdk::SyncSettings;
use serde::{Deserialize, Serialize};

pub fn login(sink: druid::ExtEventSink, mxid: String, password: String) {
    // TODO add non tokio variant for wasm
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
    let mut locked_client = crate::CLIENT.get().unwrap().lock().await;
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
    sink.submit_command(crate::SET_VIEW, crate::View::MainView, None)
        .expect("command failed to submit");
    println!("StartSync");
    locked_client
        .add_event_emitter(Box::new(EventCallback {}))
        .await;

    match locked_client.clone().sync_token().await {
        Some(token) => {
            let sync_settings = SyncSettings::new().token(token);
            //client.clone().sync(sync_settings.clone()).await?;
            locked_client
                .clone()
                .sync_forever(sync_settings, |_| async {})
                .await;
        }
        None => {
            let sync_settings = SyncSettings::new();
            locked_client
                .clone()
                .sync_forever(sync_settings, |_| async {})
                .await;
        }
    }
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
                data_dir.push("daydream");
                serde_json::to_writer(&std::fs::File::create("session.json").unwrap(), self).unwrap();
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
                data_dir.push("daydream");
                let file = std::fs::File::open("session.json");
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
