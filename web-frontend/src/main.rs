use dioxus::prelude::*;

use crate::abbreviation::{Entry, ABBREVIATIONS};
use crate::filter::Filter;

mod abbreviation;
mod filter;

fn main() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(APP);
}

static APP: fn(Scope) -> Element = |ctx| {
    let filter_query = use_state(ctx, || Filter::from(""));

    let entries: Vec<&Entry> = ABBREVIATIONS
        .iter()
        .filter(|entry| filter_query.match_against(entry).is_some())
        .take(70)
        .collect();

    let entries_rendered = entries.into_iter().map(|entry| {
        rsx!(
            tr {
                td {entry.abbreviation()}
                td {entry.description()}
            }
        )
    });

    ctx.render(rsx! {
        div {
            input {
                value: "{filter_query}",
                r#type: "text",
                oninput: move |e| filter_query.set(e.value.as_str().into()),
                autofocus: true,
            }
        }

        table {
            entries_rendered
        }
    })
};
