use crate::abbreviation::{Entry, ABBREVIATIONS};
use crate::filter::Filter;
use crate::on_event::use_on_window;
use dioxus::prelude::*;
use std::rc::Rc;

mod abbreviation;
mod filter;
mod on_event;

fn main() {
    console_error_panic_hook::set_once();
    launch(app);
}

fn app() -> Element {
    let mut filter_query = use_signal(|| Filter::from(""));

    let mut filter_query_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let mut entries: Vec<&Entry> = ABBREVIATIONS
        .iter()
        .filter(|entry| filter_query.read().match_against(entry).is_some())
        .take(70)
        .collect();

    entries.sort();

    let entries = entries;

    const FILTER_INPUT_ID: &str = "filter_input";

    // Call out to JS to set the window onfocus event.
    // Trying to do so via web-sys is too painful and overkill for this tiny use case.
    use_effect(move || {
        document::eval(&format!(
            r#"
            const filter_input = document.getElementById("{FILTER_INPUT_ID}")
            window.onfocus = (_event) => {{
                filter_input.focus()
                filter_input.select()
            }}
            filter_input.onfocus = (_event) => {{
                filter_input.select()
            }}
        "#
        ));
    });

    use_on_window("keydown", move |event: web_sys::KeyboardEvent| {
        if event.key() == "Escape" {
            filter_query.set(Filter::from(""));
            if let Some(filter_query_element) = filter_query_element.cloned() {
                let _ignored = filter_query_element.set_focus(true);
            }
        }
    });

    rsx! {
        div { id: "container", tabindex: 0,
            div {
                input {
                    id: FILTER_INPUT_ID,
                    value: "{filter_query}",
                    r#type: "text",
                    oninput: move |e| filter_query.set(e.value().as_str().into()),
                    autofocus: true,
                    onmounted: move |elt| filter_query_element.set(Some(elt.data))
                }
            }

            table {
                for entry in entries.iter() {
                    tr {
                        td { "{entry.abbreviation()}" }
                        td { "{entry.description()}" }
                    }
                }
            }
        }
    }
}
