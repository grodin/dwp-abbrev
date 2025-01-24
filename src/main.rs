use crate::abbreviation::{Entry, ABBREVIATIONS};
use crate::filter::Filter;
use dioxus::prelude::*;

mod abbreviation;
mod filter;

fn main() {
    console_error_panic_hook::set_once();
    launch(app);
}

fn app() -> Element {
    let mut filter_query = use_signal(|| Filter::from(""));

    let entries: Vec<&Entry> = ABBREVIATIONS
        .iter()
        .filter(|entry| filter_query.read().match_against(entry).is_some())
        .take(70)
        .collect();

    rsx! {
        div {
            input {
                value: "{filter_query}",
                r#type: "text",
                oninput: move |e| filter_query.set(e.value().as_str().into()),
                autofocus: true,
            }
        }

        table {
            for entry in entries.iter() {
                tr {
                    td {"{entry.abbreviation()}"}
                    td {"{entry.description()}"}
                }
            }
        }
    }
}
