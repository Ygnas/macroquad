#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum TileType {
    EMPTY,
}

impl Tile {
    /// Returns a tile with given position.
    /// # Examples
    ///
    /// ```
    /// let tile = Tile::new(0.0, 0.0)
    /// ```
    pub fn new(x: i32, y: i32) -> Tile {
        Tile {
            x,
            y,
            tile_type: TileType::EMPTY,
        }
    }
}
