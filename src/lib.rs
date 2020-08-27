use druid::{
    widget::{Align, Button, Flex, Label, Scroll, TextBox, ViewSwitcher},
    AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, ExtEventSink, Lens,
    LocalizedString, PlatformError, Selector, Size, Target, Widget, WidgetExt, WindowDesc,
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

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Daydream");

const SET_VIEW: Selector<View> = Selector::new("event-daydream.set-view");

// Replace with https://docs.rs/once_cell/1.4.1/once_cell/#lazy-initialized-global-data
static CLIENT: OnceCell<Client> = OnceCell::new();

#[derive(Clone, Copy, Data, PartialEq, Debug)]
enum View {
    LoginView,
    MainView,
}

// Do not derive as we want more control
impl Default for View {
    fn default() -> Self {
        View::LoginView
    }
}

#[derive(Clone, Data, Lens, Default)]
struct AppState {
    homeserver: String,
    mxid: String,
    password: String,
    access_token: Option<String>,
    login_running: bool,
    current_view: View,
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder)
        .with_min_size(Size::new(800.0, 400.0))
        .title(WINDOW_TITLE);

    // create the initial app state
    let initial_state = AppState::default();
    let delegate = Delegate {};
    if cfg!(debug_assertions) {
        AppLauncher::with_window(main_window)
            .delegate(delegate)
            .use_simple_logger()
            .launch(initial_state)
    } else {
        AppLauncher::with_window(main_window)
            .delegate(delegate)
            .launch(initial_state)
    }
}

fn ui_builder() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            View::LoginView => Box::new(login_ui()),
            View::MainView => Box::new(main_ui()),
            _ => panic!("wrong state"),
        },
    )
}

fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new(label).align_left().fix_height(40.0))
        .with_spacer(8.0)
        .with_child(widget.align_left().fix_width(400.0))
        .border(Color::WHITE, 1.0)
}

fn login(sink: ExtEventSink, mxid: String, password: String) {
    // TODO add non tokio variant for wasm
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "wasm32"))] {
            wasm_bindgen_futures::spawn_local(async move {
                CLIENT.get()
                    .login(&mxid, &password, None, Some("Daydream druid"))
                    .await;
                println!("Login done!");
                sink.submit_command(SET_VIEW, View::MainView, None).expect("command failed to submit");
            });
        } else {
            tokio::spawn(async move {
                CLIENT.get().unwrap()
                    .login(&mxid, &password, None, Some("Daydream druid"))
                    .await;
                println!("Login done!");
                sink.submit_command(SET_VIEW, View::MainView, None).expect("command failed to submit");
            });
        }
    }
}

fn login_ui() -> impl Widget<AppState> {
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

                    CLIENT.get_or_init(|| client.clone());

                    data.login_running = true;
                    login(ctx.get_external_handle(), mxid, password);
                }),
            ),
    ))
    .vertical()
}

fn main_ui() -> impl Widget<AppState> {
    Flex::column().with_child(label_widget(
        TextBox::new().lens(AppState::homeserver),
        "BLUB",
    ))
}
struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(view) = cmd.get(SET_VIEW) {
            data.login_running = false;

            // Clear password from memory
            data.password = "".into();

            // Change View
            data.current_view = *view;

            println!("Set View to {:?}", view);
        }
        true
    }
}
