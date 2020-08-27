use super::sync::EventCallback;
use matrix_sdk::SyncSettings;

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
    locked_client
        .login(&mxid, &password, None, Some("Daydream druid"))
        .await;
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
