use bevy::app::{App, Plugin};
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

use crate::state::AppState;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

/// Load all sprites of a folder
fn load_textures(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SpriteFolder(
        // Warning: "assets/" is implied !
        asset_server.load_folder("sprites"),
    ));
    next_state.set(AppState::SpriteLoadInProgress)
}

/// Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    sprite_folder: Res<SpriteFolder>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            next_state.set(AppState::SpriteLoadFinished);
        }
    }
}

/// Create a texture atlas with the given sampling setting
/// from the individual sprites in the given folder.
fn create_texture_atlas(
    folder: &LoadedFolder,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{} did not map to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

fn setup(
    mut commands: Commands,
    sprite_handles: Res<SpriteFolder>,
    _asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folders.get(&sprite_handles.0).unwrap();

    let (texture_atlas, _sources, texture) =
        create_texture_atlas(loaded_folder, Some(ImageSampler::nearest()), &mut textures);
    let _atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Sprite::from_image(texture.clone()),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::splat(0.5),
            ..default()
        },
    ));
}

pub struct SpriteDisplayPlugin;

impl Plugin for SpriteDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SpriteLoadStart), load_textures)
            .add_systems(
                Update,
                check_textures.run_if(in_state(AppState::SpriteLoadInProgress)),
            )
            .add_systems(OnEnter(AppState::SpriteLoadFinished), setup);
    }
}
