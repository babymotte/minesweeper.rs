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

#[derive(Debug)]
pub struct MineField {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}


impl Tile {
    pub fn get_state(&self) -> TileState {
        self.state
    }

    pub fn get_nearby_mines(&self) -> u8 {
        self.nearby_mines
    }

    fn detonate(&mut self) {
        self.state = TileState::Detonated;
    }

    fn uncover(&mut self) {
        self.state = TileState::Uncovered(self.nearby_mines);
    }
}


impl MineField {
    pub fn new(level: Difficulty, blank_x: usize, blank_y: usize) -> MineField {

        let game_parameters = get_params_for_difficulty(level);

        let mut mf = MineField {
            tiles: Vec::new(),
            width: game_parameters.0,
            height: game_parameters.1,
        };

        fill(&mut mf, game_parameters.2, blank_x, blank_y);

        mf
    }

    pub fn is_clear(&self) -> bool {
        is_clear(&self.tiles)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_mine_count(&self) -> usize {
        self.tiles.iter().fold(0, |i, t| if t.mine { i + 1 } else { i })
    }

    pub fn get_unmarked_mine_count(&self) -> usize {
        let mine_count = self.get_mine_count();
        let flag_count = self.tiles.iter().fold(0, |i, t| if t.state == TileState::Marked {
            i + 1
        } else {
            i
        });
        mine_count - flag_count
    }

    pub fn get_tile_state(&self, x: usize, y: usize) -> TileState {
        self.get_tile(x, y).state
    }

    pub fn uncover(&mut self, x: usize, y: usize) -> TileState {

        let mut tile = self.get_mut_tile(x, y);

        match tile.state {
            TileState::Covered => uncover(tile),
            _ => TileState::NoOp,
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> TileState {

        let tile = self.get_mut_tile(x, y);

        match tile.state {
            TileState::Covered => set_flag(tile),
            TileState::Marked => remove_flag(tile),
            _ => TileState::NoOp,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        let i = self.to_index(x, y);
        &self.tiles[i]
    }

    pub fn get_nearby_coordinates(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let nearby_indices = get_nearby_indices(self, x as i8, y as i8);
        nearby_indices.iter()
            .map(|i| (i % self.get_width() as usize, i / self.get_width() as usize))
            .collect()
    }

    fn get_mut_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        let i = self.to_index(x, y);
        &mut self.tiles[i]
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

fn fill(mf: &mut MineField, mines: usize, blank_x: usize, blank_y: usize) {

    let width = mf.width;
    let height = mf.height;

    let mine_coordinates = generate_mine_coordinates(width, height, mines, blank_x, blank_y);

    create_tiles(&mut mf.tiles, width, height, mine_coordinates);

    update_nearby_mine_counts(mf);
}

fn update_nearby_mine_counts(mf: &mut MineField) {

    for i in 0..mf.tiles.len() {
        mf.tiles[i].nearby_mines = count_nearby_mines(mf, &mf.tiles[i]);
    }
}

fn count_nearby_mines(mf: &MineField, tile: &Tile) -> u8 {

    let is = get_nearby_indices(mf, tile.x as i8, tile.y as i8);
    let mines: Vec<usize> = is.into_iter().filter(|i| mf.tiles[*i].mine).collect();

    mines.len() as u8
}


fn get_nearby_indices(mf: &MineField, x: i8, y: i8) -> Vec<usize> {
    let os: [(i8, i8); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let cs = os.iter().map(|o| ((x + o.0) as usize, (y + o.1) as usize));
    let filtered_cs = cs.filter(|c| c.0 < mf.width && c.1 < mf.height);
    let is = filtered_cs.map(|c| mf.to_index(c.0, c.1));
    is.collect()
}

fn generate_mine_coordinates(width: usize,
                             height: usize,
                             mines: usize,
                             blank_x: usize,
                             blank_y: usize)
                             -> Vec<(usize, usize)> {
    let all_indices = 0..width * height;
    let all_coordinates = all_indices.map(|i| (i % width, i / width));
    let filtered_coordinates = all_coordinates.filter(|c| c.0 != blank_x || c.1 != blank_y);
    rand::sample(&mut rand::thread_rng(), filtered_coordinates, mines)
}

fn create_tiles(tiles: &mut Vec<Tile>,
                width: usize,
                height: usize,
                mine_coordinates: Vec<(usize, usize)>) {
    for y in 0..height {
        for x in 0..width {
            create_tile(tiles, x, y, &mine_coordinates);
        }
    }
}

fn create_tile(tiles: &mut Vec<Tile>, x: usize, y: usize, mine_coordinates: &Vec<(usize, usize)>) {
    let mine = mine_coordinates.contains(&(x, y));
    tiles.push(Tile {
                   state: TileState::Covered,
                   x: x,
                   y: y,
                   mine: mine,
                   nearby_mines: 0,
               });
}

fn is_clear(tiles: &Vec<Tile>) -> bool {
    for t in tiles {
        if !is_tile_clear(t) {
            return false;
        }
    }
    true
}

fn is_tile_clear(tile: &Tile) -> bool {
    match tile.state {
        TileState::Detonated => false,
        TileState::Covered if !tile.mine => false,
        TileState::Marked if !tile.mine => false,
        TileState::NoOp => panic!("Found tile with NoOp state. This doesn't make sense."),
        _ => true,
    }
}

fn uncover(tile: &mut Tile) -> TileState {
    if tile.mine {
        tile.detonate();
        TileState::Detonated
    } else {
        tile.uncover();
        TileState::Uncovered(tile.nearby_mines)
    }
}

fn set_flag(tile: &mut Tile) -> TileState {
    tile.state = TileState::Marked;
    TileState::Marked
}

fn remove_flag(tile: &mut Tile) -> TileState {
    tile.state = TileState::Covered;
    TileState::Covered
}