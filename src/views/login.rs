use crate::utils::label_widget;
use crate::AppState;
use druid::{
    widget::{Align, Button, Either, Flex, Label, Scroll, Spinner, TextBox},
    LocalizedString, Widget, WidgetExt,
};

pub fn login_ui() -> impl Widget<AppState> {
    let button = Button::new("Login").on_click(|ctx, data: &mut AppState, _env| {
        println!("Login button clicked!");
        let homeserver = (*data).homeserver.clone();
        let mxid = (*data).mxid.clone();
        let password = (*data).password.clone();
        crate::matrix::login::create_client(homeserver);

        data.login_running = true;
        crate::matrix::login::login(ctx.get_external_handle(), mxid, password);
    });
    let button_placeholder = Flex::column()
        .with_child(Label::new(LocalizedString::new("Login processing...")).padding(5.0))
        .with_child(Spinner::new());
    let either = Either::new(|data, _env| data.login_running, button_placeholder, button);

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
            .with_child(either),
    ))
    .vertical()
}
