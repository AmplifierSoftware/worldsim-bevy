use bevy::prelude::*;
use bevy_egui::*;
use bracket_noise::prelude::*;
use std::ops::RangeInclusive;

pub struct CellGridPlugin;
/*
    explination time,
    assign a value (we'll call this height) to each cell between 0.0 and 1.0,
    0 = black,
    1 = white

    ---------------------------
    we ignore basically everything above this line because i implimented it differently

    use noise to get this value
    assign it to the cell
    profit

    ---------------------------
*/

enum CellState {
    Dead,
    Alive,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn create_noise_grid(
        width: u32,
        height: u32,
        space: f32,
        seed: u64,
        octaves: i32,
        freq: f32,
    ) -> Grid {
        let mut cells = Vec::new();

        //noise let here

        for x in 0..width {
            let mut rows = Vec::new();
            for y in 0..height {
                let new_x = x as f32 * space + 1.0;
                let new_y = y as f32 * space + 1.0;

                let mut noise = FastNoise::seeded(seed);
                noise.set_noise_type(NoiseType::SimplexFractal);
                noise.set_fractal_octaves(octaves);
                noise.set_frequency(freq);

                let value = noise.get_noise(new_x / width as f32, new_y / height as f32);

                // adjust noise params

                let cell = Cell::new(Vec3::new(new_x as f32, new_y as f32, 0.0), value);

                rows.push(cell);
            }
            cells.push(rows);
        }
        Grid { cells }
    }
}
struct Cell {
    posision: Vec3,
    value: f32,
}
impl Cell {
    fn new(pos: Vec3, val: f32) -> Self {
        Self {
            posision: pos,
            value: val,
        }
    }
}

fn noise_grid(mut commands: Commands, asst: Res<AssetServer>) {
    let grass = asst.load("grass.png");
    let water = asst.load("water.png");
    let sand = asst.load("sand.png");

    let grid = Grid::create_noise_grid(500, 500, 16.0, 72452, 30, 0.07);
    for row in grid.cells.iter() {
        for cell in row.iter() {
            if cell.value > 0.25 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    texture: grass.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            }
            if cell.value >= 0.23 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    texture: sand.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            } else {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    texture: water.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            }
        }
    }
}

impl Plugin for CellGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, noise_grid);
    }
}
