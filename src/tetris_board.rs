use rand::{seq::IteratorRandom, thread_rng, Rng};

pub struct BoardState {
    pub width: u8,
    pub height: u8,
    pub falling_state: TetrominoState,
    pub locked_state: Vec<Vec<BoardBlock>>,
    pub next_tetromino: TetrominoState,
    pub blinking_rows_queue: Vec<u8>,
    pub row_shift_queue: Vec<u8>
}

impl BoardState {
    pub fn new() -> Self {
        let falling_tetromino = BoardState::get_next_tetromino();
        let next_tetromino = BoardState::get_next_tetromino();
        BoardState {
            width: 10,
            height: 20,
            falling_state: falling_tetromino,
            locked_state: vec![vec![BoardBlock {
                color: [0.0; 4],
                filled: false
            }; 10]; 20],
            next_tetromino: next_tetromino,
            blinking_rows_queue: vec![],
            row_shift_queue: vec![]
        }
    }

    pub fn gravity(&mut self) {
        for row in &self.row_shift_queue.clone() {
            self.shift_board_down(row);
        }
        self.row_shift_queue = vec![];

        let mut locked = false;
        for y in 0..4 {
            for x in 0..4 {
                if (self.falling_state.shape[self.falling_state.rotation as usize] & (0x8000 >> (y * 4 + x))) != 0 {
                    if self.falling_state.y + y + 2  > self.height as i8 || self.locked_state[(self.falling_state.y + y + 1) as usize][(self.falling_state.x + x) as usize].filled {
                        locked = true;
                        self.lock_current();
                    }
                }
            }
        }
        if !locked {
            self.falling_state.y += 1;
        }
        // print_board_state(&self);

        self.check_completed_lines();
    }

    fn lock_current(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if (self.falling_state.shape[self.falling_state.rotation as usize] & (0x8000 >> (y * 4 + x))) != 0 {
                    let locked_state_block = &mut self.locked_state[(self.falling_state.y + y) as usize][(self.falling_state.x + x) as usize];
                    locked_state_block.filled = true;
                    locked_state_block.color = self.falling_state.color;
                }
            }
        }
        self.falling_state = self.next_tetromino.clone();
        self.next_tetromino = BoardState::get_next_tetromino();
    }

    fn get_next_tetromino() -> TetrominoState {
        // todo!()
        let mut rng = thread_rng();
        TetrominoState {
            color: [rng.gen_range(0.2..1.0), rng.gen_range(0.2..1.0), rng.gen_range(0.2..1.0), 1.0],
            rotation: 0,
            shape: SHAPES.iter().choose(&mut rng).unwrap().to_owned(),
            x: 4,
            y: 0
        }
    }

    pub fn move_right(&mut self) {
        if self.can_move(1) {
            self.falling_state.x += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.can_move(-1) {
            self.falling_state.x -= 1;
        }
    }

    pub fn rotate_left(&mut self) {
        if self.can_rotate(-1) {
            let mut new_rotation = (self.falling_state.rotation as i8 - 1) % 4;
            if new_rotation < 0 {
                new_rotation = 3;
            }
            self.falling_state.rotation = new_rotation;
        }
    }

    pub fn rotate_right(&mut self) {
        if self.can_rotate(1) {
            let mut new_rotation = (self.falling_state.rotation as i8 + 1) % 4;
            if new_rotation < 0 {
                new_rotation = 3;
            }
            self.falling_state.rotation = new_rotation;
        }
    }

    fn check_completed_lines(&mut self) {
        let mut y = 0;
        for row in &self.locked_state {
            let mut x = 0;
            let mut completed = true;
            for block in row {
                if !block.filled {
                    completed = false;
                }
                x += 1;
            }
            if completed {
                self.row_shift_queue.push(y);
                self.blinking_rows_queue.push(y);
            }
            y += 1;
        }
    }

    pub fn shift_board_down(&mut self, from_row: &u8) {
        // println!("fr{}", from_row);

        for y in (0..*from_row).rev() {
            // println!("y{}", y);
            for x in 0..self.width {
                // println!("x{}", x);
                self.locked_state[y as usize + 1][x as usize] = self.locked_state[y as usize][x as usize];
            }
        }
    }

    fn can_move(&self, x_delta: i16) -> bool {
        // Now check where would all not locked tetrominoes be after this move
        for y in 0..4 {
            for x in 0..4 {
                // Some bitwise fuckery to check if current xy has block or not
                if (self.falling_state.shape[self.falling_state.rotation as usize] & (0x8000 >> (y * 4 + x))) != 0 {
                    // If this move moves tetromino outside of board or inside of other tetromino, return false -> collision detected
                    if self.falling_state.x as i16 + x + x_delta + 1  > self.width as i16 || self.falling_state.x as i16 + x + x_delta < 0 || self.locked_state[(self.falling_state.y + y as i8) as usize][(self.falling_state.x as i16 + x + x_delta) as usize].filled {
                        return false
                    }
                }
            }
        }

        true
    }

    fn can_rotate(&self, rotation_delta: i8) -> bool {
        // Now check where would all not locked tetrominoes be after rotation
        let mut new_rotation = (self.falling_state.rotation as i8 + rotation_delta) % 4;
        if new_rotation < 0 {
            new_rotation = 3;
        }
        println!("{}", new_rotation);
        for y in 0..4 {
            for x in 0..4 {
                // Some bitwise fuckery to check if current xy has block or not
                if (self.falling_state.shape[new_rotation as usize] & (0x8000 >> (y * 4 + x))) != 0 {
                    // If this move moves tetromino outside of board or inside of other tetromino, return false -> collision detected
                    if self.falling_state.x as i16 + x + 1  > self.width as i16 || self.falling_state.x as i16 + x < 0 || self.falling_state.y as i16 + y + 1 > self.height as i16 ||  self.locked_state[(self.falling_state.y + y as i8) as usize][(self.falling_state.x as i16 + x) as usize].filled {
                        return false
                    }
                }
            }
        }

        true
    }

    fn can_gravity(&self) -> bool {
        // Now check where would all not locked tetrominoes be after next gravity tick
        for y in 0..4 {
            for x in 0..4 {
                // Some bitwise fuckery to check if current xy has block or not
                if (self.falling_state.shape[self.falling_state.rotation as usize] & (0x8000 >> (y * 4 + x))) != 0 {
                    // If next gravity tick moves tetromino outside of board or inside of other tetromino, return false -> collision detected
                    if self.falling_state.y + y + 2  > self.height as i8 || self.locked_state[(self.falling_state.y + y + 1) as usize][(self.falling_state.x + x) as usize].filled {
                        return false
                    }
                }
            }
        }

        true
    }
}

// https://stackoverflow.com/questions/38594574/tetris-2d-array-logic
pub static SHAPES: [[u16; 4]; 7] = [
    [ 0x4640, 0x0E40, 0x4C40, 0x4E00 ], // 'T'
    [ 0x8C40, 0x6C00, 0x8C40, 0x6C00 ], // 'S'
    [ 0x4C80, 0xC600, 0x4C80, 0xC600 ], // 'Z'
    [ 0x4444, 0x0F00, 0x4444, 0x0F00 ], // 'I'
    [ 0x44C0, 0x8E00, 0xC880, 0xE200 ], // 'J'
    [ 0x88C0, 0xE800, 0xC440, 0x2E00 ], // 'L'
    [ 0xCC00, 0xCC00, 0xCC00, 0xCC00 ]  // 'O'
];

#[derive(Clone)]
pub struct TetrominoState {
    pub shape: [u16; 4],
    pub rotation: i8,
    pub color: [f32; 4],
    pub x: i8,
    pub y: i8
}

#[derive(Clone, Copy)]
pub struct BoardBlock {
    pub color: [f32; 4],
    pub filled: bool
}
