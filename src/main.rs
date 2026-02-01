use macroquad::{prelude::*, rand::gen_range};

const GRID_W: usize = 500;
const GRID_H: usize = 260;
const CELL_SIZE: f32 = 4.0;

#[derive(Copy, Clone)]
struct Tile {
    id: i32,
    color: Color, // 0 = empty, 1 = powder
}

fn idx(x: usize, y: usize) -> usize {
    y * GRID_W + x
}

#[macroquad::main("Powder simulation")]
async fn main() {
    let brush_size = 1;
    let mut grid = vec![
        Tile {
            id: 0,
            color: color_u8!(0, 0, 0, 0),
        };
        GRID_W * GRID_H
    ];

    loop {
        clear_background(BLACK);

        // --- LEFT MOUSE = ADD POWDER ---
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let x = (mx / CELL_SIZE) as i32;
            let y = (my / CELL_SIZE) as i32;

            for rx in 0..brush_size {
                for ry in 0..brush_size {
                    let px = x + rx;
                    let py = y + ry;
                    if px >= 0 && py >= 0 && px < GRID_W as i32 && py < GRID_H as i32 {
                        let i = idx(px as usize, py as usize);
                        grid[i].id = 1;
                        grid[i].color =
                            color_u8!(253, gen_range(210, 255), gen_range(92, 190), 255);
                    }
                }
            }
        }

        // --- RIGHT MOUSE = REMOVE POWDER ---
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let x = (mx / CELL_SIZE) as i32;
            let y = (my / CELL_SIZE) as i32;

            for rx in 0..brush_size {
                for ry in 0..brush_size {
                    let px = x + rx;
                    let py = y + ry;
                    if px >= 0 && py >= 0 && px < GRID_W as i32 && py < GRID_H as i32 {
                        grid[idx(px as usize, py as usize)].id = 0;
                    }
                }
            }
        }

        // --- SIMULATION ---
        for y in (0..GRID_H - 1).rev() {
            for x in 0..GRID_W {
                let i = idx(x, y);

                if grid[i].id == 1 {
                    let below = idx(x, y + 1);
                    if grid[below].id == 0 {
                        grid.swap(i, below);
                    } else {
                        let dir = if rand::gen_range(0, 2) == 0 { -1 } else { 1 };
                        let nx = x as i32 + dir;

                        if nx >= 0 && nx < GRID_W as i32 {
                            let diag = idx(nx as usize, y + 1);
                            if grid[diag].id == 0 {
                                grid.swap(i, diag);
                            }
                        }
                    }
                }
            }
        }

        // --- RENDER ---
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                match grid[idx(x, y)].id {
                    1 => draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        grid[idx(x, y)].color,
                    ),
                    _ => {}
                }
            }
        }

        next_frame().await;
    }
}
