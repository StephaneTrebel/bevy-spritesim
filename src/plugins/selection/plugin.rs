use bevy::prelude::*;

use crate::plugins::map::MapCoordinates;

/// Component added to an entity that has been clicked on
/// (like a unit that can be moved, or a target destination for a unit movement)
#[derive(Component)]
pub struct ClickedEntity;

fn handle_click_on_entity(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    currently_clicked_entity: Option<Single<Entity, With<ClickedEntity>>>,
    entities: Query<
        (Entity, &Transform, NameOrEntity),
        (Without<ClickedEntity>, Without<Camera2d>),
    >,
) {
    if buttons.just_pressed(MouseButton::Left) {
        info!("Button pressed !");
        let window = windows.single().expect("No windows ? :(");
        if let Some(cursor_position) = window.cursor_position() {
            let (camera, camera_transform) = camera_q.single().expect("No camera ? :(");

            // Conversion écran -> monde
            if let Ok(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            {
                info!("Click at {:?}", world_position);

                // Snap world_position to map_coordinates by converting through them
                let map_coordinates: MapCoordinates = world_position.into();
                let snapped_world_position: Vec2 = map_coordinates.into();
                info!("snapped_world_position {snapped_world_position}");

                let entity = entities
                    .iter()
                    .sort_by::<(Entity, &Transform)>(|e1, e2| {
                        e2.1.translation.z.total_cmp(&e1.1.translation.z)
                    })
                    .find(|(_, t, _)| t.translation.xy() == snapped_world_position);
                if let Some((entity, _, name)) = entity {
                    info!("Entity clicked on {name}");
                    if let Some(entity) = currently_clicked_entity {
                        commands.entity(entity.entity()).remove::<ClickedEntity>();
                    }
                    commands.entity(entity).insert(ClickedEntity);
                } else {
                    info!("No entity there.");
                }
            }
        }
    }
}

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, handle_click_on_entity);
    }
}
