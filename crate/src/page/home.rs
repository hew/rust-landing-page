use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

pub fn view(email: &str, form_completed: &bool, display_err: &str) -> impl View<Msg> {
    div![
        // Hero Section
        div![
            class![ 
                C.flex,
                C.flex_grow,
                C.justify_center,
                C.items_start,
            ],
            div![
                class![
                    C.block,
                    C.w_full,
                    C.max_w_6xl,
                    C.p_12
                ],
                h1![ 
                    class![
                        C.text_4xl,
                        C.md__text_6xl,
                        C.font_extrabold,
                        C.text_green_500
                    ],
                    "HomeBakers is connecting food lovers with local, 
                     high-quality chefs." 
                ]
            ],
        ],

        // Form Section
        div![
            class![
                C.flex,
                C.flex_col,
                C.flex_grow,
                C.items_center,
                C.justify_center,
                C.px_6
            ],
            // Form
            // NOTE: when the page first loads, this string is "",
            // and after a successful form submission, is "{is_form_complete: true}"
            // Not sure how to deal with this serially, so this ghetto string check works instead
            div![
               class![""],
               match form_completed {
                   false => landing_page_form(&email),
                   true => landing_page_thanks()
               }
            ],
            div![
                class![""],
                match display_err.chars().count() {
                    0 => h1![class![""], ""],
                    _ => h1![class![""], display_err]
                }
            ]
                
        ],

   ]
}

fn landing_page_thanks() -> Node<Msg> {
    div![
        h3![
            class![
                C.text_2xl,
                C.md__text_4xl,
                C.font_bold,
                C.text_gray_600,
                C.mb_4
            ],
            "Thanks for your interest. We'll keep you updated."
        ],
    ]
}

fn landing_page_form(edit_text: &str) -> Node<Msg> {
    div![
        class![
            C.flex, 
            C.flex_col,
            C.w_full,
            C.md__max_w_lg,
            C.text_xl,
            C.md__text_2xl
        ],
        h3![
            class![
                C.text_2xl,
                C.md__text_4xl,
                C.font_bold,
                C.text_gray_600,
                C.mb_4
            ],
            "Enter your email to join the waiting list",
        ],
        div![
            class![
                C.md__flex,
                C.md__items_center
            ],
            div![
                class![
                    C.md__w_1of3,
                ],
                label![
                    class![
                        C.block,
                        C.text_gray_500,
                        C.font_bold,
                        C.md__px_4
                    ],
                    "Your email"
                ],
            ],
            div![
                class![
                    C.md__w_2of3,
                    C.py_4
                ],
                input![
                    class![ 
                        C.bg_gray_200,
                        C.appearance_none,
                        C.border_2,
                        C.border_gray_200,
                        C.rounded
                        C.w_full,
                        C.py_2,
                        C.px_4,
                        C.text_gray_700, 
                        C.leading_tight,
                        C.outline_none,
                        C.bg_white,
                        C.border_green_500
                    ],
                    attrs! {At::Class => "edit", At::Value => edit_text},
                    input_ev(Ev::Input, Msg::EditChange),
                ],
            ],  
            ],
        button![
            // see styles.css for this btn code
            class![
               C.bg_green_700,
               C.text_white,
               C.font_bold,
               C.py_3
            ], 
            simple_ev(Ev::Click, Msg::SubmitForm(edit_text.to_string())),
            "Keep me posted!"
        ]
    ]

}
