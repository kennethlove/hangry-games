use std::rc::Rc;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    label: String,
    value: String,
    name: String,
    placeholder: String,
    oninput: EventHandler<Rc<FormData>>,
    extra_css_classes: Option<String>,
}

#[component]
pub fn InputWithLabel(props: InputProps) -> Element {
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
                input {
                    r#type: "text",
                    class: "w-12 rounded-md border border-orange-700 bg-yellow-200 px-2 py-1 text-gray-900 placeholder-gray-900 focus:outline-none {props.extra_css_classes.unwrap_or_default()}",
                    id: "{props.name}",
                    name: "{props.name}",
                    placeholder: "{props.placeholder}",
                    value: "{props.value}",
                    oninput: move |event| { props.oninput.call(event.data()) },
                }
            }
        }
    }
}