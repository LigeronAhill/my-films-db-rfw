use dioxus::prelude::*;

use crate::{
    components::Button,
    models::{ButtonType, FilmModalVisibility},
};

pub fn Header(cx: Scope) -> Element {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    cx.render(rsx!(
        header {
            class: "sticky top-0 z-10 text-white bg-red-950 body-font shadow-md",
            div { class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
                a {
                    class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                    img {
                        class: "bg-transparent p-2 animate-jump size-20",
                        alt: "logo",
                        src: "image3.png",
                        "loading": "lazy"
                    }
                    span { class: "ml-3 text-2xl", "Marvel films"}
                }
               Button {
                   button_type: ButtonType::Primary,
                   onclick: move |_| {
                       is_modal_visible.write().0 = true;
                   },
                   "Add new film"
                }
            }
        }
    ))
}
