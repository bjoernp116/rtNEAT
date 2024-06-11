use bevy::{
    prelude::*,
    ecs::component::Component

};
use rand::{Rng, rngs::ThreadRng};

#[derive(Component)]
pub struct Food {}

pub fn spawn_food(mut commands: Commands, asset_server: Res<AssetServer>, mut query: Query<&Food>){
    let mut n_food = 0;
    for (_) in query.iter() {
        n_food += 1;
    }

    let bounds = Vec2::new(-10000.0, 10000.0);
    if n_food < 100 {
        let mut rng = rand::thread_rng();
        let textures = ["fries", "tomato", "can"];
        let position = Vec2::new(rng.gen_range(bounds.x..bounds.y), rng.gen_range(bounds.x..bounds.y));
        commands.spawn((
            Food {},
            SpriteBundle {
                texture: asset_server.load(format!("textures/{}.png", textures[rng.gen_range(0..3)])),
                transform: Transform::from_xyz(position.x, position.y, 0.0).with_scale(Vec3::new(5.0,5.0,5.0)),
                ..Default::default()
            },

        ));

    }

}
