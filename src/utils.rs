use crate::AppState;
use druid::{
    widget::{Flex, Label},
    AppDelegate, Color, Command, Data, DelegateCtx, Env, Target, Widget, WidgetExt,
};
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
        .with_child(Label::new(label).align_left().fix_height(40.0))
        .with_spacer(8.0)
        .with_child(widget.align_left().fix_width(400.0))
        .border(Color::WHITE, 1.0)
}
