use crate::drag_state::DragState;
use leptos::prelude::*;

#[component]
pub fn Draggable<T: Copy + Sized + Sync + Send + 'static>(
    children: Children,
    data: T,
) -> impl IntoView {
    let state = expect_context::<ReadSignal<DragState<T>>>();
    let set = expect_context::<WriteSignal<DragState<T>>>();

    view! {
        <div
            draggable="true"
            on:dragstart=move |_| set.set(DragState {data: Some(data)})
            on:dragend=move |_| set.set(DragState {data: None})>
        {children()}
        </div>
    }
}
