use std::rc::Rc;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Props)]
pub struct ButtonProps {
    pub text: String,
    pub extra_css_classes: Option<String>,
    pub onclick: Option<EventHandler<Rc<MouseData>>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let onclick = props.onclick.unwrap_or_default();
    rsx! {
        button {
            class: "bg-gradient-to-r from-orange-500 to-yellow-300 rounded-md text-red-800 orbitron-font py-1 px-2 border border-orange-700 whitespace-nowrap {props.extra_css_classes.unwrap_or_default()}",
            onclick: move |event| { onclick.call(event.data()) },
            "{props.text}",
            {props.children}
        }
    }
}
