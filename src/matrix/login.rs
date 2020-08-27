pub fn login(sink: druid::ExtEventSink, mxid: String, password: String) {
    // TODO add non tokio variant for wasm
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            wasm_bindgen_futures::spawn_local(async move {
                crate::CLIENT.get()
                    .login(&mxid, &password, None, Some("Daydream druid"))
                    .await;
                println!("Login done!");
                sink.submit_command(crate::SET_VIEW, crate::View::MainView, None).expect("command failed to submit");
            });
        } else {
            tokio::spawn(async move {
                crate::CLIENT.get().unwrap()
                    .login(&mxid, &password, None, Some("Daydream druid"))
                    .await;
                println!("Login done!");
                sink.submit_command(crate::SET_VIEW, crate::View::MainView, None).expect("command failed to submit");
            });
        }
    }
}
