use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use reactive_stores::{Field, Store, StoreFieldIterator};

#[derive(Store, Debug, Clone)]
pub struct BoardState {
    #[store(key: i32 = |row| row.key.clone())]
    rows: Vec<Row>,
}

#[derive(Store, Debug, Clone)]
pub struct Row {
    key: i32,
    #[store(key: i32 = |tile| tile.key.clone())]
    tiles: Vec<Tile>,
}

#[derive(Store, Debug, Clone)]
pub struct Tile {
    key: i32,
    value: i32,
}

#[component]
pub fn Board() -> impl IntoView {
    let board = Store::new(init_board());

    view! {
        <div class="board">
            <For each=move || board.rows()
                key=|row| row.read().key.clone()
                children=|r| {
                view! { <Row row=r/> }
            }/>
        </div>
        <button on:click=move |_| {
            board.set(init_board());
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
                .map(|j| view! {
                    <div class=move || format!("tile tile-{}", j.value().get())>
                        {j.value()}
                    </div>
                })
                .collect_view()
        }
    }
}

fn init_board() -> BoardState {
    let mut rows = Vec::<Row>::new();

    let mut tiles: Vec<i32> = (0..15).collect::<Vec<_>>();
    let mut seed_tiles = Vec::<i32>::new();

    for _ in 0..3 {
        let idx = (getrandom::u32().unwrap_or(0) % tiles.len() as u32) as usize;
        seed_tiles.push(tiles[idx]);
        tiles.remove(idx);
    }

    for i in 0..4 {
        let mut row = Vec::<Tile>::new();
        for j in 0..4 {
            let key = i * 4 + j;
            let value = if seed_tiles.contains(&key) {
                ((getrandom::u32().unwrap_or(0u32) % 2 + 1) * 2) as i32
            } else {
                0
            };

            row.push(Tile { key, value })
        }
        rows.push(Row { key: i, tiles: row })
    }

    BoardState { rows }
}
