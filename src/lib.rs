use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

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

#[derive(Clone, Data, Lens)]
struct MainState{
    count: u32
}

pub fn rmain() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title(WINDOW_TITLE);
    
    // create the initial app state
    let initial_state = MainState {
        count: 0
    };

    if cfg!(debug_assertions) {
        AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
    } else {
        AppLauncher::with_window(main_window)
        .launch(initial_state)
    }
    
}

fn ui_builder() -> impl Widget<MainState> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &MainState, _env| (*data).count.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut MainState, _env| (*data).count += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}
