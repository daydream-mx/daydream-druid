use crate::utils::label_widget;
use crate::AppState;
use druid::{
    widget::{Flex, TextBox},
    Widget, WidgetExt,
};
pub fn main_ui() -> impl Widget<AppState> {
    Flex::column().with_child(label_widget(
        TextBox::new().lens(AppState::homeserver),
        "BLUB",
    ))
}
