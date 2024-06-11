#![allow(unused)]
//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    prelude::*,
    render::render_resource::{AddressMode, SamplerDescriptor},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::CursorGrabMode
};
use bevy_egui::EguiPlugin;
use crate::creature::*;
use crate::ui::*;
use crate::brain::*;
use crate::background::*;
use crate::food::*;
mod food;
mod creature;
mod ui;
mod brain;
mod background;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_pancam::PanCamPlugin::default())
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (setup, background::spawn_background))
        .add_systems(FixedUpdate, (move_creature, food::spawn_food))
        .add_systems(Update, (ui::workspace, ui::entity_ui, ui::entity_brain))
        .run();
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default()).insert(
        bevy_pancam::PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
            enabled: true, // when false, controls are disabled. See toggle example.
            zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
            min_scale: 1., // prevent the camera from zooming too far in
            max_scale: Some(40.), // prevent the camera from zooming too far out
            ..Default::default()
        }
    );
    //let entity1 = spawn_creature(commands, asset_server);
}
fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        println!("Mouse Down");
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
