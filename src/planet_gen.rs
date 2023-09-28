use bevy::prelude::*;
use bevy_egui::*;
use bracket_noise::prelude::*;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_planet);
    }
}
/*
// !! remove this soon and replace with planetcell
// !! only keeping this here so nothing breaks (im lazy)
struct Cell {
    position: Vec3,
    value: f32,
    color: Sprite,
}*/
enum CellType {
    Ocean,
    Land,
}

struct PlanetCell {
    height: f32,      // ? in the previous iteration this was value
    cell_t: CellType, // ? is it land or water
    temperature: f32,
    humidity: f32, //? because theres a massive difference between 40c when its humid vs dry ( like a rainforest vs a desert )
    rainfall: f32, //? ml per year
}
impl PlanetCell {}

struct Grid {
    planet_cells: Vec<Vec<PlanetCell>>,
}
struct Planet {
    average_temperature: f32,
    average_humidity: f32,
    average_rainfall: f32,
    sea_level: f32,
    wind_speed: f32,
    wind_direction: f32,
}

impl Planet {
    fn heightmap(w: u16, h: u16, seed: u64) {
        // let mut generated_cells = Vec::new();
        let space = 4.0;

        for x in 0..w {
            let mut rows = Vec::new();
            for y in 0..h {
                //calc position of cell
                let position = Vec3::new(x as f32 * space, y as f32 * space, 0.0);

                //get noise value
                let mut noise = FastNoise::seeded(seed);
                noise.set_noise_type(NoiseType::SimplexFractal);
                noise.set_fractal_octaves(10);
                noise.set_frequency(1.0);

                let value = noise.get_noise(position.x / w as f32, position.y / h as f32);

                // rows.push(cell);
            }
            // generated_cells.push(rows);
        }
    }
    fn create_planet(width: u16, height: u16, sea: f32, av_rain: f32, av_temp: f32) /*-> Planet*/
    {
        // * generate the heightmap
        let heightmap = Planet::heightmap(width, height, 38452);
        // * get distance from equator

        // * get sea level
        // * generate temperature
        // * generate wind speed & wind direction
        // * generate rainfall
        // * calcualte humidity
    }
}

/*
impl Cell {
    fn new(position: Vec3, value: f32, color: Sprite) -> Self {
        Self {
            position,
            value,
            color,
        }
    }
}
*/

/* ? old grid
struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn height_map(res: u32, space: f32, seed: u64) -> Grid {
        let mut cells = Vec::new();

        for x in 0..res {
            let mut row = Vec::new();
            for y in 0..res {
                let new_position = Vec3::new(x as f32 * space, y as f32 * space, 0.0);


                // ! noise generation
                let mut noise = FastNoise::seeded(seed);
                noise.set_noise_type(NoiseType::SimplexFractal);
                noise.set_fractal_octaves(10);
                noise.set_frequency(1.0);

                let value =
                    noise.get_noise(new_position.x / res as f32, new_position.y / res as f32);

                let cell = Cell::new(
                    new_position,
                    value,
                    Sprite {
                        color: Color::hsl(150.0, 0.0, value),
                        custom_size: Some(Vec2::new(space, space)),
                        ..Default::default()
                    },
                );
                row.push(cell);
            }
            cells.push(row);
        }
        Grid { cells }
    }
}
*/
fn generate_planet(mut commands: Commands) {
    let grid = Grid::height_map(200, 1.0, 14233);
    for row in grid.cells.iter() {
        for cell in row.iter() {
            commands.spawn(SpriteBundle {
                sprite: cell.color.clone(),
                transform: Transform::from_translation(cell.position),
                ..Default::default()
            });
        }
    }
}
