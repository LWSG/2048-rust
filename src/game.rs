use rand::Rng;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Engine {
    //TODO Better Display
    pub game_data: Vec<Vec<u32>>,
    free_grid: Vec<usize>,
}

impl Engine {
    pub fn init() -> Self {
        let mut engine = Engine::new();
        engine.rand_insert();
        engine.rand_insert();
        engine
    }
    pub fn go(&mut self, d: Direction) {
        let change = match d {
            Direction::Up => self.go_updown(true),
            Direction::Down => self.go_updown(false),
            Direction::Left => self.go_leftright(true),
            Direction::Right => self.go_leftright(false),
        };
        if change {
            self.rand_insert();
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
    pub fn is_failed(&self) -> bool {
        for i in &self.game_data {
            for j in i {
                if *j == 0 {
                    return false;
                }
            }
        }
        for i in 0..4 {
            for j in 0..3 {
                if self.game_data[i][j] == self.game_data[i][j + 1] {
                    return false;
                }
            }
        }
        for j in 0..4 {
            for i in 0..3 {
                if self.game_data[i][j] == self.game_data[i + 1][j] {
                    return false;
                }
            }
        }
        true
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
    fn rand_insert(&mut self) {
        if self.free_grid.is_empty() {
            return;
        } else {
            let mut rng = rand::thread_rng();
            let new_element: u32 = if rng.gen_range(0..10) < 8 { 2 } else { 4 };
            let position = self
                .free_grid
                .remove(rng.gen_range(0..self.free_grid.len()));
            self.game_data[position / 4][position % 4] = new_element;
        }
    }
    //TODO go_updown and go_leftright may be merged into one function
    fn go_updown(&mut self, u: bool) -> bool {
        let mut free_grid = Vec::new();
        let mut change = false;
        for j in 0..4 {
            let mut slice = Vec::new();
            let mut compressed = false;
            for i in 0..4 {
                if self.game_data[uod(i, u)][j] != 0 {
                    if !compressed
                        && slice.len() != 0
                        && slice.ends_with(&[self.game_data[uod(i, u)][j]])
                    {
                        let x = slice.pop().unwrap();
                        slice.push(x * 2);
                        compressed = true;
                    } else {
                        compressed = false;
                        slice.push(self.game_data[uod(i, u)][j]);
                    }
                }
            }
            for i in 0..slice.len() {
                if !change && self.game_data[uod(i, u)][j] != slice[i] {
                    change = true;
                }
                self.game_data[uod(i, u)][j] = slice[i];
            }
            for i in slice.len()..4 {
                if !change && self.game_data[uod(i, u)][j] != 0 {
                    change = true;
                }
                self.game_data[uod(i, u)][j] = 0;
                free_grid.push(uod(i, u) * 4 + j);
            }
        }
        self.free_grid = free_grid;
        change
    }
    fn go_leftright(&mut self, u: bool) -> bool {
        let mut change = false;
        let mut free_grid = Vec::new();
        for i in 0..4 {
            let mut slice = Vec::new();
            let mut compressed = false;
            for j in 0..4 {
                if self.game_data[i][uod(j, u)] != 0 {
                    if !compressed
                        && slice.len() != 0
                        && slice.ends_with(&[self.game_data[i][uod(j, u)]])
                    {
                        let x = slice.pop().unwrap();
                        slice.push(x * 2);
                        compressed = true;
                    } else {
                        compressed = false;
                        slice.push(self.game_data[i][uod(j, u)]);
                    }
                }
            }
            for j in 0..slice.len() {
                if !change && self.game_data[i][uod(j, u)] != slice[j] {
                    change = true;
                }
                self.game_data[i][uod(j, u)] = slice[j];
            }
            for j in slice.len()..4 {
                if !change && self.game_data[i][uod(j, u)] != 0 {
                    change = true;
                }
                self.game_data[i][uod(j, u)] = 0;
                free_grid.push(i * 4 + uod(j, u));
            }
        }
        self.free_grid = free_grid;
        change
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
        eng.go_updown(false);
        assert_eq!(
            format!("{:?}", eng.game_data),
            format!(
                "{:?}",
                vec![vec![0; 4], vec![0; 4], vec![0, 0, 0, 1], vec![8, 1, 4, 8]]
            )
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
                    vec![0; 4],
                ]
            )
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
                    vec![0; 4],
                ]
            )
        );
    }
}
