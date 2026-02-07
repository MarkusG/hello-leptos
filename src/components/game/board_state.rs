use leptos::prelude::{Get, Read, Set, Update};
use reactive_stores::{AtIndex, KeyedSubfield, Store, StoreFieldIterator};
use std::collections::{HashSet, VecDeque};

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
    fn generate_tile(&self) -> ();
    fn move_right(&self) -> bool;
    fn move_left(&self) -> bool;
    fn move_up(&self) -> bool;
    fn move_down(&self) -> bool;
}

impl Playable for Store<BoardState> {
    fn generate_tile(&self) {
        let empty_tiles = self
            .rows()
            .iter_unkeyed()
            .flat_map(|r| r.tiles().iter_unkeyed())
            .filter(|t| t.value().read() == 0)
            .collect::<Vec<_>>();

        let idx = (getrandom::u32().unwrap_or(0) % empty_tiles.len() as u32) as usize;
        let value = ((getrandom::u32().unwrap_or(0u32) % 2 + 1) * 2) as i32;
        empty_tiles[idx].update(|t| t.value = value);
    }

    fn move_right(&self) -> bool {
        let mut state_changed = false;

        for row in self.rows().iter_unkeyed() {
            state_changed |= merge(row.tiles().iter_unkeyed());
            state_changed |= pack(row.tiles().iter_unkeyed());
        }

        state_changed
    }

    fn move_left(&self) -> bool {
        let mut state_changed = false;

        for row in self.rows().iter_unkeyed() {
            state_changed |= merge(row.tiles().iter_unkeyed().rev());
            state_changed |= pack(row.tiles().iter_unkeyed().rev());
        }

        state_changed
    }

    fn move_up(&self) -> bool {
        let mut state_changed = false;

        // TODO figure out how we can have just one loop and avoid the use-after-move incurred by merge(col); pack(col);
        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
                .rev()
        }) {
            state_changed |= merge(col);
        }

        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
                .rev()
        }) {
            state_changed |= pack(col);
        }

        state_changed
    }

    fn move_down(&self) -> bool {
        let mut state_changed = false;

        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
        }) {
            state_changed |= merge(col);
        }

        for col in (0..4).map(|i| {
            self.rows()
                .iter_unkeyed()
                .map(move |r| r.tiles().iter_unkeyed().nth(i).unwrap())
        }) {
            state_changed |= pack(col);
        }

        state_changed
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
) -> bool {
    // TODO we want to give precedence to the forward-most merges
    // TODO e.g. 0 2 2 2 -> 0 0 2 4 not 0 0 4 2
    let mut state_changed = false;
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

            // remove the first "parent" of the merge
            l.value().set(0);

            // duplicate the value of the second "parent"
            t.value().set(last_value * 2);

            // reset state
            last_tile = None;
            last_value = 0;

            // mark board state as changed
            state_changed = true;
        }
    }

    state_changed
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
) -> bool {
    let mut state_changed = false;
    let mut empty_tiles = VecDeque::<_>::new();

    for t in tiles.rev() {
        let value = t.value().get();

        // enqueue empty tiles, rear-most first
        if value == 0 {
            empty_tiles.push_back(t);
            continue;
        }

        // current tile is non-zero and there are one or more empty tiles behind it
        if let Some(empty_tile) = empty_tiles.pop_front() {
            // "move" the current tile to the rear-most empty tile
            empty_tile.value().set(value);
            t.value().set(0);

            // the current tile is now empty
            empty_tiles.push_back(t);

            // mark board state as changed
            state_changed = true;
        }
    }

    state_changed
}
