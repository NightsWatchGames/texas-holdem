use bevy::prelude::*;

pub fn setup_table(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("textures/table.png"),
            // transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        }
    );
}

pub fn setup_one_card(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    let texture_handle = asset_server.load("textures/poker-cards.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 89.0), 13, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..default()
        },
    );
}