use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

use crate::state::AppState;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

macro_rules! sprite_path {
    ($n1:expr) => {
        concat!("sprites", $n1)
    };
}

/// Load an image folder into the `AssetServer`
fn load_sprite_folder(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SpriteFolder(
        // Warning: "assets/" is implied !
        asset_server.load_folder(sprite_path!("")),
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

    let (texture_atlas_layout, texture_atlas_sources, texture_atlas) =
        texture_atlas_builder.build().unwrap();

    let texture_atlas_handle = textures.add(texture_atlas);

    let texture_atlas_image = textures.get_mut(&texture_atlas_handle).unwrap();
    texture_atlas_image.sampler = sampling.unwrap_or_default();

    (
        texture_atlas_layout,
        texture_atlas_sources,
        texture_atlas_handle,
    )
}

/// Terrains are the base layers of all tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteTerrainType {
    Debug,
    Desert,
    Ocean,
    Plain,
}

impl SpriteTerrainType {
    pub fn path(&self) -> &'static str {
        match self {
            // TODO: Move non-terrain sprites to their own directory and rename them
            SpriteTerrainType::Debug => sprite_path!("/debug/sprite_terrain_debug_0_0.png"),
            SpriteTerrainType::Desert => sprite_path!("/desert/sprite_terrain_desert_0_0.png"),
            SpriteTerrainType::Ocean => sprite_path!("/ocean/sprite_terrain_ocean_0_0.png"),
            SpriteTerrainType::Plain => sprite_path!("/plain/sprite_terrain_plain_0_0.png"),
        }
    }

    // Enumerate on all enum values (this is fine because those are empty variants)
    // WARN: This does not check exhaustivity at compile-time !
    // Use strum crate if you want to add that (but exhaustivity check is done in
    // path() method anyway)
    pub fn all() -> &'static [SpriteTerrainType] {
        use SpriteTerrainType::*;
        &[Desert, Debug, Ocean, Plain]
    }
}

/// Biomes are above terrain and characterize a place
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteBiomeType {
    Forest,
    Hill,
    Mountain,
}

impl SpriteBiomeType {
    pub fn path(&self) -> &'static str {
        match self {
            // TODO: Move non-terrain sprites to their own directory and rename them
            SpriteBiomeType::Forest => sprite_path!("/forest/sprite_terrain_forest_0_0.png"),
            SpriteBiomeType::Hill => sprite_path!("/hill/sprite_terrain_hill_0_0.png"),
            SpriteBiomeType::Mountain => {
                sprite_path!("/mountain/sprite_terrain_mountain_0_0.png")
            }
        }
    }

    // Enumerate on all enum values (this is fine because those are empty variants)
    // WARN: This does not check exhaustivity at compile-time !
    // Use strum crate if you want to add that (but exhaustivity check is done in
    // path() method anyway)
    pub fn all() -> &'static [SpriteBiomeType] {
        use SpriteBiomeType::*;
        &[Forest, Hill, Mountain]
    }
}

/// Terrain are the base layers of all tiles
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpriteSpecialType {
    Corn,
    Fish,
    Lumber,
    Ore,
    Snow,
}

impl SpriteSpecialType {
    pub fn path(&self) -> &'static str {
        match self {
            SpriteSpecialType::Corn => sprite_path!("/corn/sprite_terrain_corn_0_0.png"),
            SpriteSpecialType::Fish => sprite_path!("/fish/sprite_terrain_fish_0_0.png"),
            SpriteSpecialType::Lumber => sprite_path!("/lumber/sprite_terrain_lumber_0_0.png"),
            SpriteSpecialType::Ore => sprite_path!("/lumber/sprite_terrain_ore_0_0.png"),
            SpriteSpecialType::Snow => sprite_path!("/lumber/sprite_terrain_snow_0_0.png"),
        }
    }

    // Enumerate on all enum values (this is fine because those are empty variants)
    // WARN: This does not check exhaustivity at compile-time !
    // Use strum crate if you want to add that (but exhaustivity check is done in
    // path() method anyway)
    pub fn all() -> &'static [SpriteSpecialType] {
        use SpriteSpecialType::*;
        &[Corn, Fish, Lumber]
    }
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
    // Build texture atlas that will contain all sprites from loaded folder
    let (texture_atlas_layout, texture_atlas_sources, texture_atlas_image) = create_texture_atlas(
        loaded_folder_assets.get(&sprite_handles.0).unwrap(),
        Some(ImageSampler::nearest()),
        &mut texture_assets,
    );

    // Create indices from loaded sprites handles (images)
    let indices = SpriteTerrainType::all()
        .iter()
        .map(|&sprite_type| {
            let index = *texture_atlas_sources
                .texture_ids
                .get(&asset_server.get_handle(sprite_type.path()).unwrap().id())
                .unwrap();
            (sprite_type, index)
        })
        .collect::<HashMap<SpriteTerrainType, usize>>();

    commands.insert_resource(SpriteAtlas {
        texture: texture_atlas_image,
        layout: texture_atlases_layout_assets.add(texture_atlas_layout),
        indices,
    });

    commands.remove_resource::<SpriteFolder>();

    next_state.set(AppState::MapGenerationStart)
}

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SpriteLoadStart), load_sprite_folder)
            .add_systems(
                Update,
                check_sprite_folder_load.run_if(in_state(AppState::SpriteLoadInProgress)),
            )
            .add_systems(OnEnter(AppState::CreateSpriteAtlas), create_sprite_atlas);
    }
}
