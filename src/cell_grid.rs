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

                let mut p_noise = FastNoise::seeded(seed + 3);
                p_noise.set_noise_type(NoiseType::PerlinFractal);
                p_noise.set_fractal_octaves(octaves + 1);
                p_noise.set_frequency(freq - 0.03);

                let s_value = noise.get_noise(new_x / width as f32, new_y / height as f32);
                let p_value = p_noise.get_noise(new_x / width as f32, new_y / height as f32);

                let value = (s_value + p_value) / 2.0;

                // adjust noise params
                let cell_sprite = Sprite {
                    color: Color::hsl(150.0, 0.0, value),
                    custom_size: Some(Vec2::new(space, space)),
                    ..Default::default()
                };
                let cell = Cell::new(Vec3::new(new_x, new_y, 0.0), value, cell_sprite);

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
    sprite: Sprite,
}
impl Cell {
    fn new(pos: Vec3, val: f32, spr: Sprite) -> Self {
        Self {
            posision: pos,
            value: val,
            sprite: spr,
        }
    }
}

fn height_map_grid(mut commands: Commands, asst: Res<AssetServer>) {
    let grid = Grid::create_noise_grid(400, 400, 16.0, 6356227, 20, 0.07);
    for row in grid.cells.iter() {
        for cell in row.iter() {
            commands.spawn(SpriteBundle {
                sprite: cell.sprite.clone(),
                transform: Transform::from_translation(cell.posision),
                ..Default::default()
            });
        }
    }
}

fn color_grid(mut commands: Commands, asst_serv: Res<AssetServer>) {
    let grass_texture = asst_serv.load("grass.png");
    let sand_texture = asst_serv.load("sand.png");
    let water_texture = asst_serv.load("water.png");

    let grid = Grid::create_noise_grid(400, 400, 16.0, 6356227, 20, 0.05);
    for row in grid.cells.iter() {
        for cell in row.iter() {
            if cell.value > 0.07 {
                commands.spawn(SpriteBundle {
                    texture: grass_texture.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            }
            if cell.value >= 0.05 {
                commands.spawn(SpriteBundle {
                    texture: sand_texture.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            } else {
                commands.spawn(SpriteBundle {
                    texture: water_texture.clone(),
                    transform: Transform::from_translation(cell.posision),
                    ..Default::default()
                });
            }
        }
    }
}

impl Plugin for CellGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, color_grid);
    }
}
