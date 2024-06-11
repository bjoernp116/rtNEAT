use bevy::{
    asset::AssetServer, ecs::{
        component::Component, system::{
            Commands, Query, Res
        }
    }, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform::commands
};
use crate::creature::{self, Creature};
use bevy_egui::{egui::{self, RichText, epaint}, EguiContexts};

pub fn workspace(
    mut contexts: EguiContexts,
    mut query: Query<(&mut Creature)>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    ){
    egui::Window::new("Entities").show(contexts.ctx_mut(), |ui| {
        if ui.button("+").clicked() {
            creature::spawn_creature(commands, asset_server);
        }
        for (mut creature) in query.iter_mut() {
            if ui.button(format!("{}: {}", creature.specie, creature.name)).clicked() {
                creature.open_tab = !creature.open_tab;
                break;
            }
        }
    });
}
pub fn entity_ui(mut contexts: EguiContexts, mut query: Query<(&mut Creature, &Transform)>){
    for (mut creature, transform) in query.iter_mut() {
        if creature.open_tab {
            egui::Window::new(format!("{} {}", creature.name, creature.specie)).show(contexts.ctx_mut(), |ui| {
                ui.label(format!("Name: {}", creature.name));
                ui.label(format!("Specie: {}", creature.specie));
                ui.label(format!("Age: {}", creature.age));
                ui.label(RichText::new("Brain:").heading());
                if ui.button("Show Brain").clicked() {
                    creature.show_brain = true;
                }
                ui.label(format!("Inputs: {:?}", creature.brain.inputs));
                ui.label(format!("Outputs: {:?}", creature.brain.outputs));
                ui.label("Manual:");
                ui.add(egui::Slider::new(&mut creature.brain.outputs[0], 0.0..=1.0));
                ui.add(egui::Slider::new(&mut creature.brain.outputs[1], 0.0..=1.0));
                ui.label(RichText::new("Transform").heading());
                let v = transform.translation;
                ui.label(format!("Position: ({}, {})", v.x, v.y));
                ui.label(format!("Rotation: {}", transform.rotation.xyz()));

            });
        }
    }

}
pub fn entity_brain(
    mut commands: Commands,
    mut creature_query: Query<&mut Creature>,
    mut camera_query: Query<(&mut Camera, &mut bevy_pancam::PanCam)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for creature in creature_query.iter_mut() {
        if creature.show_brain {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
                    material: materials.add(Color::GREEN),
                    ..default()
                }
            ));
        }
    }
}












