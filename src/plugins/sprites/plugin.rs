use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};
use bevy::sprite::{SpritePickingSettings, SpritePickingMode};

use crate::plugins::VARIANT_COUNT;
use crate::state::AppState;

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

// Warning: "assets/" is implied !
const SPRITE_DIRECTORY_NAME: &str = "sprites";

/// Load an image folder into the `AssetServer`
fn load_sprite_folder(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SpriteFolder(
        asset_server.load_folder(SPRITE_DIRECTORY_NAME),
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
pub enum SpriteType {
    Selector,

    Corn,
    Debug,
    Desert,
    Fish,
    Forest,
    Hill,
    Lumber,
    Mountain,
    Ocean,
    Ore,
    Plain,
    Snow,

    Settler,

    Village,
}

impl std::fmt::Display for SpriteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpriteType::Selector => "selector",

                SpriteType::Corn => "corn",
                SpriteType::Debug => "debug",
                SpriteType::Desert => "desert",
                SpriteType::Fish => "fish",
                SpriteType::Forest => "forest",
                SpriteType::Hill => "hill",
                SpriteType::Lumber => "lumber",
                SpriteType::Mountain => "mountain",
                SpriteType::Ocean => "ocean",
                SpriteType::Ore => "ore",
                SpriteType::Plain => "plain",
                SpriteType::Snow => "snow",

                SpriteType::Settler => "settler",

                SpriteType::Village => "village",
            }
        )?;
        Ok(())
    }
}

impl SpriteType {
    pub fn path(&self, variant: u8) -> String {
        let sprite_name = self.to_string();

        match self {
            SpriteType::Selector => {
                format!(
                    "{SPRITE_DIRECTORY_NAME}/{}/{}_0.png",
                    sprite_name, sprite_name
                )
            }

            SpriteType::Debug
            | SpriteType::Desert
            | SpriteType::Forest
            | SpriteType::Hill
            | SpriteType::Mountain
            | SpriteType::Ocean
            | SpriteType::Plain => {
                format!(
                    "{SPRITE_DIRECTORY_NAME}/{}/sprite_terrain_{}_{}_0.png",
                    sprite_name, sprite_name, variant
                )
            }

            SpriteType::Corn
            | SpriteType::Fish
            | SpriteType::Lumber
            | SpriteType::Ore
            | SpriteType::Snow => {
                format!(
                    "{SPRITE_DIRECTORY_NAME}/{}/sprite_terrain_{}_0_0.png",
                    sprite_name, sprite_name
                )
            }

            SpriteType::Settler => {
                format!(
                    "{SPRITE_DIRECTORY_NAME}/units/{}/{}_0_0.png",
                    sprite_name, sprite_name
                )
            }

            SpriteType::Village => {
                format!(
                    "{SPRITE_DIRECTORY_NAME}/structures/{}/{}_0_0.png",
                    sprite_name, sprite_name
                )
            }
        }
    }

    // Enumerate on all enum values (this is fine because those are empty variants)
    // WARN: This does not check exhaustivity at compile-time !
    // Use strum crate if you want to add that (but exhaustivity check is done in
    // path() method anyway)
    pub fn all() -> &'static [SpriteType] {
        use SpriteType::*;
        &[
            Selector, Corn, Debug, Desert, Fish, Forest, Hill, Lumber, Mountain, Ocean, Ore, Plain,
            Snow, Settler, Village,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct SpriteTypeVariant {
    sprite_type: SpriteType,
    variant: u8,
}

#[derive(Resource)]
pub struct SpriteAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    indices: HashMap<SpriteTypeVariant, usize>,
}

impl SpriteAtlas {
    pub fn get(&self, sprite_type: &SpriteType, variant: u8) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: *self
                .indices
                .get(&SpriteTypeVariant {
                    sprite_type: *sprite_type,
                    variant,
                })
                .unwrap_or_else(|| panic!("Unknow sprite type {:?}", sprite_type)),
        }
    }

    pub fn sprite(&self, sprite_type: &SpriteType, variant: u8, color: Option<Color>) -> Sprite {
        let mut sprite =
            Sprite::from_atlas_image(self.texture.clone(), self.get(sprite_type, variant));

        if let Some(c) = color {
            sprite.color = c;
        }
        sprite
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
    let indices = SpriteType::all()
        .iter()
        .flat_map(|&sprite_type| {
            info!("Loading {sprite_type}");
            // Cloning before move-ing into inner closure
            let asset_server = asset_server.clone();
            let texture_ids = texture_atlas_sources.texture_ids.clone();
            (0..VARIANT_COUNT).map(move |variant| {
                let index = texture_ids
                    .get(
                        &asset_server
                            .get_handle(sprite_type.path(variant))
                            .unwrap_or_else(|| {
                                panic!("Cannot find sprite type with path {}", sprite_type.path(0))
                            })
                            .id(),
                    )
                    .unwrap();
                (
                    SpriteTypeVariant {
                        sprite_type,
                        variant,
                    },
                    *index,
                )
            })
        })
        .collect::<HashMap<SpriteTypeVariant, usize>>();

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
            .add_systems(OnEnter(AppState::CreateSpriteAtlas), create_sprite_atlas)
            .insert_resource(SpritePickingSettings {
                picking_mode: SpritePickingMode::BoundingBox,
                require_markers: false
            });
    }
}
