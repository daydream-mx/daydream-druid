use crate::utils::label_widget;
use crate::AppState;
use druid::{
    widget::{Align, Button, Flex, Scroll, TextBox},
    Widget, WidgetExt,
};
use matrix_sdk::{Client, ClientConfig};
use url::Url;
pub fn login_ui() -> impl Widget<AppState> {
    Scroll::new(Align::centered(
        Flex::column()
            .with_child(label_widget(
                TextBox::new().lens(AppState::homeserver),
                "Homeserver",
            ))
            .with_child(label_widget(
                TextBox::new().lens(AppState::mxid),
                "Username",
            ))
            .with_child(label_widget(
                TextBox::new().lens(AppState::password),
                "Password",
            ))
            .with_child(
                Button::new("Login").on_click(|ctx, data: &mut AppState, _env| {
                    println!("Login button clicked!");
                    let homeserver = (*data).homeserver.clone();
                    let mxid = (*data).mxid.clone();
                    let password = (*data).password.clone();
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

                    crate::CLIENT.get_or_init(|| client.clone());

                    data.login_running = true;
                    crate::matrix::login::login(ctx.get_external_handle(), mxid, password);
                }),
            ),
    ))
    .vertical()
}
