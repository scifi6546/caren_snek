static TILE_SIZE: u32 = 20;
#[derive(Serialize, Deserialize, Clone, std::cmp::PartialEq, Debug)]
pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    fn get_color(&self) -> u32 {
        match self {
            Self::Floor => 0x191919,
            Self::Wall => 0x033499,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Grid {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, tiles: Vec<Tile>) -> Grid {
        Grid {
            tiles: tiles,
            width: width,
            height: height,
        }
    }
    pub fn get_tile(&self, position: Vector2) -> Option<Tile> {
        let index = (position.x as u32 * self.width + position.y as u32) as usize;
        if index < self.tiles.len() {
            return Some(self.tiles[index].clone());
        }
        None
    }
    pub fn draw(&self) -> Vec<u32> {
        let mut out: Vec<u32> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                out.append(&mut vec![
                    self.tiles[(x * self.width + y) as usize].get_color(),
                    x * TILE_SIZE,
                    y * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                ])
            }
        }
        return out;
    }
}