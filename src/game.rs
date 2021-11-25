use rand::Rng;
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub enum EngineStatus {
    Ok,
    Failed,
}
#[derive(Debug)]
pub enum EngineError {
    NoFreeGrid,
}
pub struct Engine {
    game_data: Vec<Vec<u32>>,
    free_grid: Vec<usize>,
}
impl Engine {
    pub fn init() -> Self {
        let mut engine = Engine::new();
        engine.rand_insert().expect("Initialize Failed");
        engine.rand_insert().expect("Initialize Failed");
        engine
    }
    pub fn go(&mut self, d: Direction) -> EngineStatus {
        match d {
            Direction::Up => {
                self.go_updown(true);
            }
            Direction::Down => {
                self.go_updown(false);
            }
            Direction::Left => {
                self.go_leftright(true);
            }
            Direction::Right => {
                self.go_leftright(false);
            }
        }
        match self.rand_insert() {
            Ok(_) => EngineStatus::Ok,
            Err(_) => EngineStatus::Failed,
        }
    }
    pub fn score(&self) -> u32 {
        let mut sum = 0;
        for i in &self.game_data {
            for j in i {
                sum += j;
            }
        }
        sum
    }

    fn new() -> Self {
        Engine {
            game_data: vec![vec![0; 4]; 4],
            free_grid: (0..15).collect(),
        }
    }
    fn from(d: &Vec<u32>) -> Self {
        let mut game_data = vec![vec![0; 4]; 4];
        let mut free_grid = Vec::new();
        let size = if 16 > game_data.len() { d.len() } else { 16 };
        for i in 0..size {
            game_data[i / 4][i % 4] = if d[i] == 0 {
                free_grid.push(i);
                0
            } else {
                d[i]
            }
        }
        Engine {
            game_data,
            free_grid,
        }
    }
    fn rand_insert(&mut self) -> Result<(), EngineError> {
        if self.free_grid.is_empty() {
            Err(EngineError::NoFreeGrid)
        } else {
            let mut rng = rand::thread_rng();
            let new_element: u32 = if rng.gen() { 2 } else { 4 };
            let position = self
                .free_grid
                .remove(rng.gen_range(0..self.free_grid.len()));
            self.game_data[position / 4][position % 4] = new_element;
            Ok(())
        }
    }
    fn go_updown(&mut self, u: bool) {
        for j in 0..4 {
            for i in 0..3 {
                if self.game_data[uod(i, u)][j] == self.game_data[uod(i + 1, u)][j] {
                    self.game_data[uod(i, u)][j] *= 2;
                    self.game_data[uod(i + 1, u)][j] = 0;
                }
            }
        }

        let mut free_grid = Vec::new();
        for j in 0..4 {
            let mut ufree = 0;
            for i in 0..4 {
                if self.game_data[uod(i, u)][j] != 0 {
                    self.game_data[uod(ufree, u)][j] = self.game_data[uod(i, u)][j];
                    ufree += 1;
                }
            }
            for i in ufree..4 {
                self.game_data[uod(i, u)][j] = 0;
                free_grid.push(i * 4 + j);
            }
        }
        self.free_grid = free_grid;
    }
    fn go_leftright(&mut self, u: bool) {
        for i in 0..4 {
            for j in 0..3 {
                if self.game_data[i][uod(j, u)] == self.game_data[i][uod(j + 1, u)] {
                    self.game_data[i][uod(j, u)] *= 2;
                    self.game_data[i][uod(j + 1, u)] = 0;
                }
            }
        }

        let mut free_grid = Vec::new();
        for i in 0..4 {
            let mut ufree = 0;
            for j in 0..4 {
                if self.game_data[i][uod(j, u)] != 0 {
                    self.game_data[i][uod(ufree, u)] = self.game_data[i][uod(j, u)];
                    ufree += 1;
                }
            }
            for j in ufree..4 {
                self.game_data[i][uod(j, u)] = 0;
                free_grid.push(i * 4 + j);
            }
        }
        self.free_grid = free_grid;
    }
    fn update_free_grid(&mut self) {
        let mut free_grid = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.game_data[i][j] == 0 {
                    free_grid.push(i * 4 + j);
                }
            }
        }
        self.free_grid = free_grid;
    }
}
fn uod(i: usize, u: bool) -> usize {
    if u {
        i
    } else {
        3 - i
    }
}

#[cfg(test)]
mod test {
    use super::Engine;
    #[test]
    fn go() {
        let mut eng = Engine::from(&vec![2, 0, 0, 1, 2, 1, 4, 8, 4]);
        eng.go_updown(true);
        assert_eq!(
            format!("{:?}", eng.game_data),
            format!(
                "{:?}",
                vec![vec![4, 1, 4, 1], vec![4, 0, 0, 8], vec![0; 4], vec![0; 4]]
            )
        );
        eng.update_free_grid();
        assert_eq!(
            format!("{:?}", eng.free_grid),
            format!("{:?}", vec![5, 6, 8, 9, 10, 11, 12, 13, 14, 15])
        );
        eng.go_updown(false);
        assert_eq!(
            format!("{:?}", eng.game_data),
            format!(
                "{:?}",
                vec![vec![0; 4], vec![0; 4], vec![0, 0, 0, 1], vec![8, 1, 4, 8]]
            )
        );
        eng.update_free_grid();
        assert_eq!(
            format!("{:?}", eng.free_grid),
            format!("{:?}", vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
        let mut eng = Engine::from(&vec![2, 2, 0, 4, 2, 0, 0, 8, 4]);
        eng.go_leftright(true);
        assert_eq!(
            format!("{:?}", eng.game_data),
            format!(
                "{:?}",
                vec![
                    vec![4, 4, 0, 0],
                    vec![2, 8, 0, 0],
                    vec![4, 0, 0, 0],
                    vec![0; 4]
                ]
            )
        );
        eng.update_free_grid();
        assert_eq!(
            format!("{:?}", eng.free_grid),
            format!("{:?}", vec![2, 3, 6, 7, 9, 10, 11, 12, 13, 14, 15])
        );
        eng.go_leftright(false);
        assert_eq!(
            format!("{:?}", eng.game_data),
            format!(
                "{:?}",
                vec![
                    vec![0, 0, 0, 8],
                    vec![0, 0, 2, 8],
                    vec![0, 0, 0, 4],
                    vec![0; 4]
                ]
            )
        );
        eng.update_free_grid();
        assert_eq!(
            format!("{:?}", eng.free_grid),
            format!("{:?}", vec![0, 1, 2, 4, 5, 8, 9, 10, 12, 13, 14, 15])
        );
    }
}
