use macroquad::prelude::*;

const GRID_W: usize = 500;
const GRID_H: usize = 260;

const CELL_SIZE: f32 = 4.0;
#[derive(Copy, Clone, PartialEq)]
struct Tile {
    id: i32,
    tick: i32,
}

fn idx(x: usize, y: usize) -> usize {
    y * GRID_W + x
}

#[macroquad::main("Powder simulation")]
async fn main() {
    let mut grid = vec![Tile { id: 0, tick: 0 }; GRID_W * GRID_H];
    loop {
        clear_background(BLACK);
        let size = 2;

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();

            let x = (mx / CELL_SIZE) as usize;
            let y = (my / CELL_SIZE) as usize;
            for rx in 0..size {
                for ry in 0..size {
                    grid[idx(x + rx, y + ry)].id = 1;
                }
            }
            if x < GRID_W && y < GRID_H {
                grid[idx(x, y)].id = 1;
            }
        }
        for y in (0..GRID_H - 1).rev() {
            for x in 0..GRID_W {
                let i = idx(x, y);

                if grid[i].id == 1 {
                    let below = idx(x, y + 1);
                    if grid[below].id == 0 {
                        grid.swap(i, below);
                    } else {
                        if x > 0 && grid[idx(x - 1, y + 1)].id == 0 {
                            grid.swap(i, idx(x - 1, y + 1));
                        } else if x > 0 && x + 1 < GRID_W && grid[idx(x + 1, y - 1)].id == 0 {
                            grid.swap(i, idx(x + 1, y + 1));
                        }
                    }
                }
            }
        }

        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if grid[idx(x, y)].id == 1 {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        YELLOW,
                    );
                }
            }
        }

        next_frame().await;
    }
}
