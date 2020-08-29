use crate::AppState;
use druid::{
    widget::{Controller, Flex, Label, List, Scroll, TextBox},
    Color, Env, Event, EventCtx, UnitPoint, Widget, WidgetExt,
};
use matrix_sdk::Room;
use std::sync::Arc;

struct ForceRerender;

impl<W: Widget<AppState>> Controller<AppState, W> for ForceRerender {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event {
            if cmd.is(crate::FORCE_RERENDER) {
                ctx.request_update();
            }

            if let Some(room_list) = cmd.get(crate::APPEND_ROOMLIST) {
                let mut new_room_list = Vec::with_capacity(data.rooms_list.len() + room_list.len());
                for room in data.rooms_list.iter() {
                    new_room_list.push(room.clone());
                }
                for room in room_list {
                    new_room_list.push(Arc::new(room.clone()));
                }
                data.rooms_list = Arc::new(new_room_list);
                ctx.request_update();
            }

            if let Some(items) = cmd.get(crate::REMOVE_ROOMLIST_ITEMS) {
                let mut new_room_list = Vec::with_capacity(data.rooms_list.len() - items.len());
                for room in data.rooms_list.iter() {
                    if !items.iter().any(|x| x.room_id == room.room_id) {
                        new_room_list.push(room.clone());
                    }
                }
                data.rooms_list = Arc::new(new_room_list);
                ctx.request_update();
            }
        }
        child.event(ctx, event, data, env)
    }
}

pub fn main_ui() -> impl Widget<AppState> {
    // RELOGIN if required (Hack but we know that the room list only exists if we are in the Main View ^^)
    crate::matrix::login::relogin(crate::EVENT_SINK.get().unwrap().clone());
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
    })
    .lens(AppState::rooms_list)
    .controller(ForceRerender {});
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
