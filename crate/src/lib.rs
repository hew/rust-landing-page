// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]

#[macro_use]
extern crate validator_derive;
extern crate validator;

mod generated;
mod page;

use futures::Future;
use generated::css_classes::C;
use seed::{prelude::*, *};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use Visibility::*;

const TITLE_SUFFIX: &str = "HomeBakers";
const AIRTABLE_LINK: &str =
    "https://api.airtable.com/v0/appu9e2cWb8s1OGoo/Table%201";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub email: String,
    pub display_err: String,
    pub form_err: bool,
    pub form_completed: bool,
    pub in_prerendering: bool,
    pub page: Page,
    pub menu_visibility: Visibility
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
}

impl Page {
    pub fn to_href(self) -> &'static str {
        match self {
            Self::Home => "/",
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        match url.path.first().map(String::as_str) {
            None | Some("") => Self::Home,
            _ => Self::Home,
        }
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // @TODO: Seed can't hydrate prerendered html (yet).
    // https://github.com/David-OConnor/seed/issues/223
    if let Some(mount_point_element) = document().get_element_by_id("app") {
        mount_point_element.set_inner_html("");
    }

    orders.send_msg(Msg::UpdatePageTitle);

    Model {
        page: url.into(),
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        email: String::from(""),
        display_err: String::from(""),
        form_err: false,
        form_completed: false
    }
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//    Routes
// ------ ------

pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url))
}

// ------ ------
//    Request
// ------ ------

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct MessageRecords {
    #[validate(email(message = "This email is not valid"))]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFields {
    pub fields: MessageRecords,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequestBody {
    pub records: Vec<MessageFields>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageResponseBody {
    pub records: Vec<MessageFields>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FormCompleteObject {
    pub is_form_complete: bool,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Url),
    UpdatePageTitle,
    EditChange(String),
    SubmitForm(String),
    DisplayError(String),
    MessageSent(fetch::ResponseDataResult<SendMessageResponseBody>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DisplayError(email) => {
            let data = MessageRecords {
                email
            };
            let _ = data.validate().map_err(|err: ValidationErrors| {
                let errors: HashMap<&str, validator::ValidationErrorsKind> =
                    err.into_errors();
                for (_field, err) in errors {
                    match err {
                        validator::ValidationErrorsKind::Field(vec_err) => {
                            match &vec_err[0].message {
                                Some(m) => {
                                    model.display_err = m.to_string();
                                    model.form_err = true
                                }
                                None => (),
                            };
                            orders.skip();
                        }
                        _ => {
                            orders.skip();
                        }
                    };
                }
            });
        }
        Msg::RouteChanged(url) => {
            model.page = url.into();
            orders.send_msg(Msg::UpdatePageTitle);
        }
        Msg::UpdatePageTitle => {
            let title = match model.page {
                Page::Home => TITLE_SUFFIX.to_owned(),
            };
            document().set_title(&title);
        }
        Msg::EditChange(email) => model.email = email,
        Msg::SubmitForm(e) => {

            // TODO: learn how to do this better
            let email = e.clone();
            let email_success = email.clone();
            let email_failure = email.clone();
            let data = MessageRecords {
                email
            };

            match data.validate() {
                Ok(_) => orders.perform_cmd(send_message(email_success)),
                Err(_) => orders.send_msg(Msg::DisplayError(email_failure)),
            };
        }
        Msg::MessageSent(Ok(response_data)) => {
            log!(format!("Response data: {:#?}", response_data));
            model.form_completed = true;
        }

        Msg::MessageSent(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Sending message failed - {:#?}",
                fail_reason
            ));
            model.display_err = String::from("A network error occurred!");
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
    // @TODO: Setup `prerendered` properly once https://github.com/David-OConnor/seed/issues/223 is resolved
    let prerendered = true;
    div![
        class![
            C.fade_in => !prerendered,
            C.min_h_screen,
            C.flex,
            C.flex_col,
        ],
        match model.page {
            Page::Home => page::home::view(
                &model.email,
                &model.form_completed,
                &model.display_err
            )
            .els(),
        },
        // page::partial::header::view(model).els(),
        page::partial::footer::view().els(),
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

pub fn send_message(email: String) -> impl Future<Item = Msg, Error = Msg> {
    let mut vec = Vec::new();
    let payload = MessageFields {
        fields: MessageRecords {
            email: email.into(),
        },
    };
    vec.push(payload);
    let message = SendMessageRequestBody {
        records: vec,
    };
    Request::new(AIRTABLE_LINK)
        .header("Authorization", "Bearer keyAAXLNtnbfQtzV5")
        .method(Method::Post)
        .send_json(&message)
        .fetch_json_data(Msg::MessageSent)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::build(init, update, view).routes(routes).finish().run();

    log!("App started.");
}
