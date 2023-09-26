use bevy::prelude::{Plugin, *};
use bevy_egui::*;
use bevy_mod_picking::{prelude::RaycastPickCamera, *};
use bevy_pancam::*;

mod cell_grid;
fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.14, 0.28, 0.42)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "World Map Generator".into(),
                        resolution: (1280.0, 720.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PanCamPlugin::default())
        .add_plugins(cell_grid::CellGridPlugin)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, start_world)
        .run()
}

fn start_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}
