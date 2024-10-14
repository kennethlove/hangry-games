use dioxus::prelude::*;
use crate::tributes::actors::Tribute;
use crate::gui::components::tribute_actions_group::TributeActionsGroup;
#[component]
pub fn TributeTable(tributes: Signal<Vec<Tribute>>) -> Element {
    rsx! {
        table {
            class: "min-w-full divide-y-2 divide-gray-200 bg-white text-sm",
            thead {
                class: "ltr:text-left rtl:text-right",
                tr {
                    th {
                        class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                        "Name"
                    }
                    th {
                        class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                        "District"
                    }
                    th {
                        class: "whitespace-nowrap px-4 py-2 font-medium text-gray-900",
                        "Status"
                    }
                    th {
                        class: "px-4 py-2",
                        "Actions"
                    }
                }
            }
            tbody {
                class: "divide-y divide-gray-200",
                for tribute in tributes.read().iter() {
                    tr {
                        td {
                            class: "whitespace-nowrap px-4 py-2 text-gray-700",
                            "{tribute.name}"
                        }
                        td {
                            class: "whitespace-nowrap px-4 py-2 text-gray-700",
                            "{tribute.district}"
                        }
                        td {
                            class: "whitespace-nowrap px-4 py-2 text-gray-700",
                            "{tribute.status}"
                        }
                        td {
                            class: "flex-end",
                            TributeActionsGroup { tribute: tribute.clone(), signal: tributes.clone() }
                        }
                    }
                }
            }
        }
    }
}