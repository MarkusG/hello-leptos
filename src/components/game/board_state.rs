use std::collections::HashSet;
use leptos::prelude::{Get, Set};
use reactive_stores::{AtIndex, KeyedSubfield, Store, StoreFieldIterator};

#[derive(Store, Debug, Clone)]
pub struct BoardState {
    #[store(key: i32 = |row| row.key.clone())]
    pub rows: Vec<Row>,
}

#[derive(Store, Debug, Clone)]
pub struct Row {
    pub key: i32,
    #[store(key: i32 = |tile| tile.key.clone())]
    tiles: Vec<Tile>,
}

#[derive(Store, Debug, Clone)]
pub struct Tile {
    key: i32,
    value: i32,
}

impl BoardState {
    pub fn new() -> Self {
        let mut rows = Vec::<Row>::new();

        let mut tiles: Vec<i32> = (0..15).collect::<Vec<_>>();
        let mut seed_tiles = HashSet::<i32>::new();

        for _ in 0..3 {
            let idx = (getrandom::u32().unwrap_or(0) % tiles.len() as u32) as usize;
            seed_tiles.insert(tiles[idx]);
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
}

pub trait Playable {
    fn move_right(&self) -> ();
    fn move_left(&self) -> ();
    fn move_up(&self) -> ();
    fn move_down(&self) -> ();
}

impl Playable for Store<BoardState> {
    fn move_right(&self) -> () {
        for row in self.rows().iter_unkeyed() {
            merge(row.tiles().iter_unkeyed());
            pack(row.tiles().iter_unkeyed());
        }
    }

    fn move_left(&self) -> () {
        for row in self.rows().iter_unkeyed() {
            merge(row.tiles().iter_unkeyed().rev());
            pack(row.tiles().iter_unkeyed().rev());
        }
    }

    fn move_up(&self) -> () {
        // TODO figure out how we can have just one loop and avoid the use-after-move incurred by merge(col); pack(col);
        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
                .rev()
        }) {
            merge(col);
        }

        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
                .rev()
        }) {
            pack(col);
        }
    }

    fn move_down(&self) -> () {
        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
        }) {
            merge(col);
        }

        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
        }) {
            pack(col);
        }
    }
}

fn merge(
    // TODO could this be simpler?
    tiles: impl Iterator<
        Item = AtIndex<
            KeyedSubfield<
                AtIndex<KeyedSubfield<Store<BoardState>, BoardState, i32, Vec<Row>>, Vec<Row>>,
                Row,
                i32,
                Vec<Tile>,
            >,
            Vec<Tile>,
        >,
    >,
) {
    let mut last_tile = None;
    let mut last_value = 0;

    // caller-defined order determines orientation (row/col) and direction (right/left/up/down)
    for t in tiles {
        let tile_value = t.value().get();

        // skip empty tiles
        if tile_value == 0 {
            continue;
        }

        // non-zero tile differing from last non-zero tile
        if last_value != tile_value {
            last_value = tile_value;
            last_tile = Some(t);
        } else if let Some(l) = last_tile {
            // non-zero tile matching the last non-zero tile
            // duplicate its value and store it in the new row
            l.value().set(0);
            t.value().set(last_value * 2);
            last_tile = None;
            last_value = 0;
        }
    }
}

fn pack(
    tiles: impl DoubleEndedIterator<
        Item = AtIndex<
            KeyedSubfield<
                AtIndex<KeyedSubfield<Store<BoardState>, BoardState, i32, Vec<Row>>, Vec<Row>>,
                Row,
                i32,
                Vec<Tile>,
            >,
            Vec<Tile>,
        >,
    >,
) {
    let mut next_free_tile = None;

    for t in tiles.rev() {
        let value = t.value().get();

        if value == 0 {
            if next_free_tile.is_none() {
                next_free_tile = Some(t);
            }
            continue;
        }

        if let Some(f) = next_free_tile {
            f.value().set(value);
            t.value().set(0);
            next_free_tile = Some(t);
        }
    }
}
