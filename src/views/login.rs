use crate::utils::label_widget;
use crate::AppState;
use crochet::{Button, Column, Cx, Label, Row};

pub fn login_ui(cx: &mut Cx, state: &mut AppState) {
    Row::new().build(cx, |cx| {
        /*Scroll::new(Align::centered())
        .vertical()*/
        // TODO add scroll and align

        Column::new().build(cx, |cx| {
            label_widget(cx, state.homeserver.clone(), "Homeserver");
            label_widget(cx, state.mxid.clone(), "Username");
            label_widget(cx, state.password.clone(), "Password");
            if state.login_running {
                Column::new().build(cx, |cx| {
                    Label::new(String::from("Login processing...")).build(cx);
                    // TODO add spinner
                });
            } else if Button::new("Login").build(cx) {
                println!("Login button clicked!");
                let homeserver = state.homeserver.to_string();
                let mxid = state.mxid.to_string();
                let password = state.password.to_string();
                let client = crate::matrix::login::create_client(homeserver);

                state.login_running = true;
                //crate::matrix::login::login(ctx.get_external_handle(), client, mxid, password);
            }
        });
    });
}
