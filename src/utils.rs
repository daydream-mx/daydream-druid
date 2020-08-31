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
        if let Some(room_id) = cmd.get(crate::SWITCH_ROOM) {
            data.current_room = room_id.to_string();
            println!("Set current room to {:?}", room_id);
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
