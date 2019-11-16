use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    footer![class![
        C.h_16,
        C.flex,
        C.justify_center,
        // sm__
        C.sm__h_24,
    ]]
}
