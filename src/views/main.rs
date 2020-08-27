use crate::AppState;
use druid::{
    widget::{Flex, Label, List, Scroll},
    Color, UnitPoint, Widget, WidgetExt,
};
pub fn main_ui() -> impl Widget<AppState> {
    let mut flex = Flex::row();
    let room_list = List::new(|| {
        Label::new(|item: &u32, _env: &_| format!("List item #{}", item))
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
    flex.add_flex_child(
        Scroll::new(event_list)
            .vertical()
            .expand_height()
            .lens(AppState::events_list)
            .background(Color::rgb8(41, 41, 41))
            .border(Color::WHITE, 1.0),
        3.0,
    );
    flex
}
