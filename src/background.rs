use bevy::{
    prelude::*,
};

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let tile_size = Vec2::new(1280.0, 1025.0);
    let board_size = Vec2::new(10.0, 12.0);
    let mut pos = Vec2::new(
        -(tile_size.x * board_size.x),
        -(tile_size.y * board_size.y));
    while pos.x < tile_size.x * board_size.x {
        while pos.y < tile_size.y * board_size.y {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/background.png"),
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    ..Default::default()
                }
            ));

            pos.y += tile_size.y;
        }
        pos.x += tile_size.x;
        pos.y = -(tile_size.y * board_size.y);
    }

}
