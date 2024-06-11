use std::process::Child;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::CursorGrabMode,
};
use crate::brain::Brain;
use crate::food::Food;
use rnglib::{RNG, Language};
use rand::{rngs::ThreadRng, Rng};
#[derive(Component)]
pub struct Creature {
    pub name: String,
    pub specie: String,
    pub age: u32,
    pub open_tab: bool,
    pub brain: Brain,
    pub show_brain: bool,
}
pub fn spawn_creature(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) -> Entity {

    let font = asset_server.load("fonts/Tiny5-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::BLACK,
    };
    let name = generate_name(&Language::Elven);
    let specie = generate_name(&Language::Roman);
    let text_transform = Transform {
        translation: Vec3::new(0.0, 15.0, 0.0),
        scale: Vec3::new(0.1, 0.1, 0.1),
        ..Default::default()
    };
    let mut rng = rand::thread_rng();
    let position = Transform::from_xyz(
        rng.gen::<f32>() * 5.0,
        rng.gen::<f32>() * 5.0,
        0.0
    ).with_scale(Vec3::new(7.0, 7.0, 7.0));
    commands.spawn((
        Creature {
            name: name.clone(),
            specie: specie.clone(),
            age: 0,
            open_tab: false,
            brain: Brain::new(),
            show_brain: false,
        },
        SpriteBundle {
            texture: asset_server.load("textures/creature.png"),
            transform: position,
            ..Default::default()
        },
    )).with_children(|parent|{
        parent.spawn((
                Text2dBundle {
                    text: Text::from_section(format!("{} ({})", name.clone(), specie.clone()), text_style.clone()),
                    transform: text_transform,
                    ..Default::default()
                }
        ));
    }).id()
}
pub fn move_creature(
    mut query: Query<(Entity, &Creature, &mut Transform, &Children)>,
    mut child_query: Query<&mut Transform, Without<Creature>>,
    mut time: Res<Time>,
) {
    for (entity, creature, mut transform, children) in query.iter_mut() {
        let angle = creature.brain.outputs[0]/50.0;
        if angle != 0.0 {
            transform.rotate_z(angle);
        }


        let speed = 100.0;
        let angle_degrees = f32::abs(transform.rotation.xyz().z) * 360.0;
        let velocity = creature.brain.outputs[1]*speed;
        let movement_direction = transform.rotation * -Vec3::Y;
        let movement_distance = velocity * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;

        let mut iter = child_query.iter_many_mut(children);
        while let Some(mut child_transform) = iter.fetch_next() {
            child_transform.rotate_z(-angle);
            child_transform.translation = Vec3::new(0.0, 15.0, 0.0);
        }

    }

}
fn watch_inputs(mut creature_query: Query<(&mut Creature, &Transform)>, mut food_query: Query<(&Food, &Transform)>) {
    for (creature, creature_transform) in creature_query.iter_mut() {
        let creature_vector: Vec2 = creature_transform.translation.xy();
        let mut shortest_distance = 100000.0;
        for (_, food_transform) in food_query.iter() {
            let food_vector = food_transform.translation.xy();
            let creature_food_vector = creature_vector - food_vector;
            let distance = vector_length(creature_food_vector);
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }

    }
}
fn vector_length(v: Vec2) -> f32 {
    f32::sqrt(f32::abs(v.x).powi(2) + f32::abs(v.y).powi(2))
}
fn generate_name(l: &Language) -> String {
    let rng = RNG::try_from(l).unwrap();
    rng.generate_name()

}
pub fn push_inputs(
    query: Query<(&mut Creature)>
){

}
