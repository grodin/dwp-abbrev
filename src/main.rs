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

    const FILTER_INPUT_ID: &str = "filter_input";

    // Call out to JS to set the window onfocus event.
    // Trying to do so via web-sys is too painful and overkill for this tiny use case.
    use_effect(move || {
        document::eval(&format!(
            r#"
            window.onfocus = (_event) => {{
                const filter_input = document.getElementById("{FILTER_INPUT_ID}")
                filter_input.focus()
                filter_input.select()
            }}
        "#
        ));
    });
    rsx! {
        div {
            input {
                id: FILTER_INPUT_ID,
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
