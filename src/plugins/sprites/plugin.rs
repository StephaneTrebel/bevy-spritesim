use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

use crate::state::AppState;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

/// Load an image folder into the `AssetServer`
fn load_sprite_folder(
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

/// Advance the `AppState` once all images handles have been loaded by the `AssetServer`
fn check_sprite_folder_load(
    mut next_state: ResMut<NextState<AppState>>,
    sprite_folder: Res<SpriteFolder>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            next_state.set(AppState::CreateSpriteAtlas);
        }
    }
}

/// Create a texture atlas with the given sampling setting
/// from the individual sprites of the given folder.
fn create_texture_atlas(
    folder: &LoadedFolder,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(image) = textures.get(id) else {
            warn!(
                "{} did not map to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        texture_atlas_builder.add_texture(Some(id), image);
    }

    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();

    let texture = textures.add(texture);

    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

/// Terrain are the base layers of all tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteTerrainType {
    Desert,
    Plain,
}

#[derive(Resource)]
pub struct SpriteAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    indices: HashMap<SpriteTerrainType, usize>,
}

impl SpriteAtlas {
    pub fn get(&self, sprite: SpriteTerrainType) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: *self
                .indices
                .get(&sprite)
                .unwrap_or_else(|| panic!("Unknow sprite type {:?}", sprite)),
        }
    }

    pub fn sprite(&self, sprite_type: SpriteTerrainType) -> Sprite {
        Sprite::from_atlas_image(self.texture.clone(), self.get(sprite_type))
    }
}

fn create_sprite_atlas(
    mut commands: Commands,

    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    sprite_handles: Res<SpriteFolder>,

    asset_server: Res<AssetServer>,

    mut texture_atlases_layout_assets: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_assets: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let loaded_folder = loaded_folder_assets.get(&sprite_handles.0).unwrap();

    // Build texture atlas that will contain all sprites from loaded folder
    let (texture_atlas_layout, texture_atlas_sources, texture_atlas_image) = create_texture_atlas(
        loaded_folder,
        Some(ImageSampler::nearest()),
        &mut texture_assets,
    );

    // Create indices from loaded sprites handles (images)
    let indices = HashMap::from([
        (
            SpriteTerrainType::Desert,
            *texture_atlas_sources
                .texture_ids
                .get(
                    &asset_server
                        .get_handle("sprites/terrain/desert.png")
                        .unwrap()
                        .id(),
                )
                .unwrap(),
        ),
        (
            SpriteTerrainType::Plain,
            *texture_atlas_sources
                .texture_ids
                .get(
                    &asset_server
                        .get_handle("sprites/terrain/plain.png")
                        .unwrap()
                        .id(),
                )
                .unwrap(),
        ),
    ]);

    commands.insert_resource(SpriteAtlas {
        texture: texture_atlas_image,
        layout: texture_atlases_layout_assets.add(texture_atlas_layout),
        indices,
    });

    commands.remove_resource::<SpriteFolder>();

    next_state.set(AppState::ReadyToDraw)
}

fn draw(mut commands: Commands, atlas: Res<SpriteAtlas>) {
    commands.spawn((
        atlas.sprite(SpriteTerrainType::Desert),
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::splat(1.),
            ..default()
        },
    ));
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SpriteLoadStart), load_sprite_folder)
            .add_systems(
                Update,
                check_sprite_folder_load.run_if(in_state(AppState::SpriteLoadInProgress)),
            )
            .add_systems(OnEnter(AppState::CreateSpriteAtlas), create_sprite_atlas)
            .add_systems(OnEnter(AppState::ReadyToDraw), draw);
    }
}
