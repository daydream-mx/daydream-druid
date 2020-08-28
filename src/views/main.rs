use crate::AppState;
use druid::{
    widget::{Flex, Label, List, Scroll, TextBox},
    Color, UnitPoint, Widget, WidgetExt,
};
use matrix_sdk::Room;
use std::sync::Arc;

pub fn main_ui() -> impl Widget<AppState> {
    // RELOGIN if required
    crate::matrix::login::relogin();

    let mut flex = Flex::row();
    // TODO Use AppState or ListIter
    // TODO Only hold a Arc<Vec<Room>> in AppState (ARC ist important!)
    // TODO keep content of that list to a minimum
    let room_list = List::new(|| {
        Label::new(|room: &Arc<Room>, _env: &_| room.display_name())
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb8(41, 41, 41))
            .border(Color::BLACK, 1.0)
    });
    let event_list = List::new(|| {
        Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
            .align_vertical(UnitPoint::LEFT)
            .padding(10.0)
            .expand()
            .height(50.0)
            .background(Color::rgb8(41, 41, 41))
            .border(Color::BLACK, 1.0)
    });

    flex.add_flex_child(
        Scroll::new(room_list)
            .vertical()
            .expand_height()
            .lens(AppState::rooms_list)
            .background(Color::rgb8(41, 41, 41))
            .border(Color::WHITE, 1.0),
        1.0,
    );

    let mut event_side = Flex::column();
    let event_list_full = Scroll::new(event_list)
        .vertical()
        //.expand_height()
        .lens(AppState::events_list)
        .background(Color::rgb8(41, 41, 41))
        .border(Color::WHITE, 1.0);
    event_side.add_child(event_list_full);
    event_side.add_flex_spacer(1.0);
    event_side.add_child(TextBox::new().lens(AppState::new_message).expand_width());
    event_side.add_spacer(4.0);
    flex.add_flex_child(event_side, 3.0);
    flex
}
