use dioxus::prelude::*;
use crate::gui::components::input::{Input, InputProps};

#[component]
pub fn InputWithLabel(props: InputProps) -> Element {
    let labeled_input = Input(props.clone());
    rsx! {
        div {
            label {
                class: "leading-8 cursor-pointer capitalize",
                r#for: "{props.name}",
                span {
                    class: "block text-sm w-full overflow-hidden whitespace-nowrap text-ellipsis",
                    title: "{props.label}",
                    "{props.label}"
                }
            }
            {labeled_input}
        }
    }
}