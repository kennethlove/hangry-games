use std::rc::Rc;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct InputProps {
    pub label: String,
    pub value: String,
    pub name: String,
    pub placeholder: String,
    pub oninput: EventHandler<Rc<FormData>>,
    pub extra_css_classes: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
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
