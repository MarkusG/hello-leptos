use crate::components::game::board_state::*;
use leptos::ev;
use leptos::prelude::*;
use reactive_stores::{Field, Store, StoreFieldIterator};

#[component]
pub fn Board() -> impl IntoView {
    let mut board = Store::new(BoardState::new());

    let handle = window_event_listener(ev::keydown, move |e| {
        let key = e.key();

        let board_changed = match key.as_str() {
            "ArrowRight" => board.move_right(),
            "ArrowLeft" => board.move_left(),
            "ArrowUp" => board.move_up(),
            "ArrowDown" => board.move_down(),
            _ => return,
        };

        if board_changed {
            board.generate_tile()
        }
    });

    on_cleanup(move || handle.remove());

    view! {
        <div class="board">
            <For each=move || board.rows()
                key=|row| row.read().key.clone()
                children=|r| {
                view! { <Row row=r/> }
            }/>
        </div>
        <button on:click=move |_| {
            board.set(BoardState::new());
        }>reset</button>
    }
}

#[component]
fn Row(#[prop(into)] row: Field<Row>) -> impl IntoView {
    view! {
        {
            move || row
                .tiles()
                .iter_unkeyed()
                .map(|j| {
                    if j.value().get() != 0 {
                        return view! {
                            <div class=move || format!("tile tile-{}", j.value().get())>
                                {j.value()}
                            </div>
                        }.into_any()
                    }

                    return view! {<div class="tile"></div>}.into_any();
                })
                .collect_view()
        }
    }
}
