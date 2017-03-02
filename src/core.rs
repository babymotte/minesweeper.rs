extern crate rand;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
    Custom(usize, usize, usize),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileState {
    Covered,
    Marked,
    Uncovered(u8),
    Detonated,
    NoOp,
}

#[derive(PartialEq, Debug)]
pub struct Tile {
    state: TileState,
    x: usize,
    y: usize,
    mine: bool,
    nearby_mines: u8,
}

pub struct MineField {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}


impl Tile {
    fn detonate(&mut self) {
        self.state = TileState::Detonated;
    }

    fn uncover(&mut self) {
        self.state = TileState::Uncovered(self.nearby_mines);
    }
}


impl MineField {
    pub fn new(level: Difficulty, blank_x: usize, blank_y: usize) -> MineField {

        let params = get_params_for_difficulty(level);

        let mut mf = MineField {
            tiles: Vec::new(),
            width: params.0,
            height: params.1,
        };

        mf.fill(params.2, blank_x, blank_y);

        mf
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> TileState {
        self.get_tile(x, y).state
    }

    pub fn uncover(&mut self, x: usize, y: usize) -> TileState {

        let mut tile = self.get_mut_tile(x, y);

        match tile.state {
            TileState::Covered => {
                if tile.mine {
                    tile.detonate();
                    TileState::Detonated
                } else {
                    tile.uncover();
                    TileState::Uncovered(tile.nearby_mines)
                }
            }
            _ => TileState::NoOp,
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> TileState {

        let tile = self.get_mut_tile(x, y);

        match tile.state {
            TileState::Covered => {
                tile.state = TileState::Marked;
                TileState::Marked
            }
            TileState::Marked => {
                tile.state = TileState::Covered;
                TileState::Covered
            }
            _ => TileState::NoOp,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        let i = self.to_index(x, y);
        &self.tiles[i]
    }

    fn get_mut_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        let i = self.to_index(x, y);
        &mut self.tiles[i]
    }

    fn fill(&mut self, mines: usize, blank_x: usize, blank_y: usize) {

        let mine_coordinates = {
            let all_indices = 0..self.width * self.height;
            let all_coordinates = all_indices.map(|i| (i % self.width, i / self.width));
            let filtered_coordinates = all_coordinates.filter(|c| c.0 != blank_x || c.1 != blank_y);
            rand::sample(&mut rand::thread_rng(), filtered_coordinates, mines)
        };

        for y in 0..self.height {
            for x in 0..self.width {
                let mine = mine_coordinates.contains(&(x, y));
                self.tiles.push(Tile {
                    state: TileState::Covered,
                    x: x,
                    y: y,
                    mine: mine,
                    nearby_mines: 0,
                });
            }
        }

        for i in 0..self.tiles.len() {
            self.tiles[i].nearby_mines = self.count_nearby_mines(&self.tiles[i]);
        }

    }

    fn count_nearby_mines(&self, tile: &Tile) -> u8 {

        let is = self.get_nearby_indices(tile.x as i8, tile.y as i8);
        let mines: Vec<usize> = is.into_iter().filter(|i| self.tiles[*i].mine).collect();

        mines.len() as u8
    }

    fn get_nearby_indices(&self, x: i8, y: i8) -> Vec<usize> {
        let os: [(i8, i8); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1),
                                 (1, 1)];
        let cs = os.iter().map(|o| ((x + o.0) as usize, (y + o.1) as usize));
        let filtered_cs = cs.filter(|c| c.0 < self.width && c.1 < self.height);
        let is = filtered_cs.map(|c| self.to_index(c.0, c.1));
        is.collect()
    }

    fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

pub fn get_params_for_difficulty(level: Difficulty) -> (usize, usize, usize) {

    match level {
        Difficulty::Beginner => (9, 9, 10),
        Difficulty::Intermediate => (16, 16, 40),
        Difficulty::Expert => (30, 16, 99),
        Difficulty::Custom(w, h, m) => (w, h, m),
    }
}