// Game of life in Rust/Raylib-rs
// code free to use by anyone (it's not perfect)
// written by Kim -> @KimMakesGames on Twitter

use raylib::prelude::*;

fn main() {
    let window_size: (i32, i32) = (640, 480);
    let cell_size: (i32, i32) = (20,20);
    let grid_size: (i32, i32) = (window_size.0 / cell_size.0, window_size.1 / cell_size.1);

    let mut grid = Grid {
        width: grid_size.0 as usize,
        height: grid_size.1 as usize,
        cells: vec![false; (grid_size.0 * grid_size.1) as usize],
    };

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .build();    
    
    rl.set_target_fps(60);

    while !rl.window_should_close() {

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse_pos = rl.get_mouse_position();
            let grid_pos = (mouse_pos.x as i32 / cell_size.0, mouse_pos.y as i32 / cell_size.1);
            let cell = grid.get_cell(grid_pos.0, grid_pos.1);
            grid.set_cell(grid_pos.0, grid_pos.1, !cell);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            grid.iterate(); //do repeatedly
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);
        
        for i in 0..grid.cells.len() {
            
            let cell_pos = grid.index_to_pos(i);
            if grid.cells[i] {
                d.draw_rectangle(cell_pos.0 * cell_size.0, cell_pos.1 * cell_size.1, cell_size.0, cell_size.1, Color::WHITE);
            }
            d.draw_rectangle_lines(cell_pos.0 * cell_size.0, cell_pos.1 * cell_size.1, cell_size.0, cell_size.1, Color::LIGHTGRAY);
            
        }

    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn set_cell(&mut self, x: i32, y: i32, v: bool) {
        let i = self.pos_to_index(x, y);
        self.cells[i] = v;
    }
    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        self.cells[self.pos_to_index(x, y)]
    }
    pub fn iterate(&mut self) {
        let mut cells_next = vec![false; self.cells.len()];
        
        for i in 0..self.cells.len() {
            let cell_pos = self.index_to_pos(i);
            let mut neighbor_count = 0;
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 { continue; } //self

                    let neighbor_pos = (cell_pos.0 + x, cell_pos.1 + y);
                    if (neighbor_pos.0 < 0 || neighbor_pos.0 >= self.width as i32) || (neighbor_pos.1 < 0 || neighbor_pos.1 >= self.height as i32) { continue; } //bounds

                    if self.cells[self.pos_to_index(neighbor_pos.0, neighbor_pos.1)] { neighbor_count += 1 }
                }
            }

            if self.cells[i] { // is alive
                if neighbor_count < 2 || neighbor_count > 3 { // die from underpopulation or overpopulation
                    cells_next[i] = false; 
                } else if neighbor_count > 1 && neighbor_count < 4 { // survive
                    cells_next[i] = true;
                } else { // die naturally
                    cells_next[i] = false;
                }
            } else if neighbor_count == 3 { // is dead, should be born
                cells_next[i] = true;
            }
        }

        self.cells = cells_next;
    }

    fn index_to_pos(&self, i: usize) -> (i32, i32) {
       ((i  % self.width) as i32, (i / self.width) as i32)
    }
    fn pos_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width as i32 + x) as usize
    }

    pub fn draw() {

    }
}
