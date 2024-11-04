use dioxus::prelude::*;
use crate::gui::components::ShowModal;
use crate::gui::components::button::Button;

#[component]
pub fn FillTributesButton() -> Element {
    let mut state = use_context::<Signal<ShowModal>>();

    rsx! {
        Button {
            text: "Fill game",
            onclick: move |_| {
                state.write().show = true;
            },
            extra_css_classes: "b-1 w-min"
        }
    }
}
