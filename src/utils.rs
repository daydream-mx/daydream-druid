use crate::AppState;
use druid::{
    widget::{Flex, Label},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Target, Widget, WidgetExt,
};
use std::sync::Arc;

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(view) = cmd.get(crate::SET_VIEW) {
            data.login_running = false;

            // Clear password from memory
            data.password = "".into();

            data.rooms_list = Arc::new(vec![1, 2, 3, 4, 5, 6]);
            data.events_list = Arc::new(vec![1, 2, 3, 4, 5, 6]);

            // Change View
            data.current_view = *view;

            println!("Set View to {:?}", view);
        }
        true
    }
}

pub fn label_widget<T: Data>(widget: impl Widget<T> + 'static, label: &str) -> impl Widget<T> {
    Flex::row()
        .must_fill_main_axis(true)
        .with_child(Label::new(label).fix_height(40.0))
        .with_spacer(8.0)
        .with_flex_child(widget.expand_width(), 1.0)
        .with_spacer(8.0)
        .border(Color::WHITE, 1.0)
}
