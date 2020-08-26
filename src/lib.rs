use druid::{
    widget::{Align, Button, Flex, Label, Scroll, TextBox},
    AppLauncher, Color, Data, Lens, LocalizedString, PlatformError, Size, Widget, WidgetExt,
    WindowDesc,
};
use matrix_sdk::{Client, ClientConfig};
use once_cell::sync::OnceCell;
use url::Url;

// This wrapper function is the primary modification we're making to the vanilla desktop
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_main() {
    // This hook is necessary to get panic messages in the console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    rmain();
}

/////////////////////////////
//// START OF ACTUAL APP ////
/////////////////////////////

const WINDOW_TITLE: LocalizedString<MainState> = LocalizedString::new("Daydream");

// Replace with https://docs.rs/once_cell/1.4.1/once_cell/#lazy-initialized-global-data
static CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Clone, Data, Lens, Default)]
struct MainState {
    homeserver: String,
    mxid: String,
    password: String,
    access_token: Option<String>,
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .with_min_size(Size::new(800.0, 400.0))
        .title(WINDOW_TITLE);

    // create the initial app state
    let initial_state = MainState::default();

    if cfg!(debug_assertions) {
        AppLauncher::with_window(main_window)
            .use_simple_logger()
            .launch(initial_state)
    } else {
        AppLauncher::with_window(main_window).launch(initial_state)
    }
}

fn ui_builder() -> impl Widget<MainState> {
    Scroll::new(Align::centered(
        Flex::column()
            .with_child(label_widget(
                TextBox::new().lens(MainState::homeserver),
                "Homeserver",
            ))
            .with_child(label_widget(
                TextBox::new().lens(MainState::mxid),
                "Username",
            ))
            .with_child(label_widget(
                TextBox::new().lens(MainState::password),
                "Password",
            ))
            .with_child(
                Button::new("Login").on_click(|_event, data: &mut MainState, _env| {
                    println!("Login button clicked!");
                    let homeserver = (*data).homeserver.clone();
                    let mxid = (*data).mxid.clone();
                    let password = (*data).password.clone();
                    let client_config = ClientConfig::new();
                    //.store_path(config.matrix.store_path.clone());
                    let homeserver_url = Url::parse(&homeserver).unwrap();
                    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

                    CLIENT.get_or_init(|| {
                        client.clone()
                    });
                    tokio::spawn(async move {
                        client.login(&mxid, &password, None, Some("Daydream druid")).await;
                        println!("Login done!");
    
                    });
                    // Clear password from memory
                    // This could potentially be unsafe as we dont know if the login is done. But it should also not break things
                    (*data).password = "".into()
                }),
            ),
    ))
    .vertical()
}

fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new(label).align_left().fix_height(40.0))
        .with_spacer(8.0)
        .with_child(widget.align_left().fix_width(400.0))
        .border(Color::WHITE, 1.0)
}
