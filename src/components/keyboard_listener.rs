use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::{IntoView, component, ev};

#[component]
pub fn KeyboardListener(children: Children) -> impl IntoView {
    let handle = window_event_listener(ev::keydown, |e| {
        let code = e.key();
        console_log(format!("{code:?}").as_str());
    });

    on_cleanup(move || handle.remove());

    children()
}
