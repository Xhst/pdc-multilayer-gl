use rand::Rng;

pub struct Gol {
    size: u32,
    pub current: Vec<bool>,
    pub next: Vec<bool>,
}

impl Gol {
    pub fn new(grid_size: u32, density: f32) -> Gol {
        let size = grid_size + 2;
        let mut current = vec![false; (size * size) as usize];
        let next = vec![false; (size * size) as usize];

        Gol::init_grid(&mut current, size, density);

        Gol {
            size,
            current,
            next,
        }
    }

    fn init_grid(current: &mut [bool], size: u32, density: f32) {
        let mut rng = rand::thread_rng();
        for i in 1..size - 1 {
            for j in 1..size - 1 {
                current[Gol::idx(size, i, j)] = rng.gen::<f32>() < density;
            }
        }
    }

    fn idx(size: u32, i: u32, j: u32) -> usize {
        (i * size + j) as usize
    }

    fn count_alive_neighbors(&self, i: u32, j: u32) -> u8 {
        let size = self.size;
        self.current[Gol::idx(size, i - 1, j - 1)] as u8
            + self.current[Gol::idx(size, i - 1, j)] as u8
            + self.current[Gol::idx(size, i - 1, j + 1)] as u8
            + self.current[Gol::idx(size, i, j - 1)] as u8
            + self.current[Gol::idx(size, i, j + 1)] as u8
            + self.current[Gol::idx(size, i + 1, j - 1)] as u8
            + self.current[Gol::idx(size, i + 1, j)] as u8
            + self.current[Gol::idx(size, i + 1, j + 1)] as u8
    }

    pub fn swap_grids(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn step(&mut self) {
        let size = self.size;
        for i in 1..size - 1 {
            for j in 1..size - 1 {
                let alive_neighbors = self.count_alive_neighbors(i, j);

                let is_alive = self.current[Gol::idx(size, i, j)];

                let next_state = (is_alive && !(alive_neighbors < 2 || alive_neighbors > 3))
                    || (!is_alive && alive_neighbors == 3);

                self.next[Gol::idx(size, i, j)] = next_state;
            }
        }

        self.swap_grids();
    }

    pub fn fill_ghost_cells(&mut self) {
        let size = self.size;

        // Left and right borders
        for i in 1..size - 1 {
            self.current[Gol::idx(size, i, 0)] = self.current[Gol::idx(size, i, size - 2)];
            self.current[Gol::idx(size, i, size - 1)] = self.current[Gol::idx(size, i, 1)];
        }

        // Top and bottom borders
        for j in 0..size {
            self.current[Gol::idx(size, 0, j)] = self.current[Gol::idx(size, size - 2, j)];
            self.current[Gol::idx(size, size - 1, j)] = self.current[Gol::idx(size, 1, j)];
        }

        // Corners
        self.current[Gol::idx(size, 0, 0)] = self.current[Gol::idx(size, size - 2, size - 2)];
        self.current[Gol::idx(size, 0, size - 1)] = self.current[Gol::idx(size, size - 2, 1)];
        self.current[Gol::idx(size, size - 1, 0)] = self.current[Gol::idx(size, 1, size - 2)];
        self.current[Gol::idx(size, size - 1, size - 1)] = self.current[Gol::idx(size, 1, 1)];
    }
}