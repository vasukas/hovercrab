//! GLTF scene manipulation

use crate::utils::for_crate::bevy::iterate_children_recursively;
use bevy::prelude::*;

/// **TL;DR: stick this with [`SceneBundle`] when loading GLTF files so all meshes will be turned into static colliders.**
///
/// Add children fixed rigid body and trimesh collider for each (grand)child of this entity with [`Mesh`] asset handle.
///
/// **Meshes must be already loaded; also this will wait until at least one entity has asset handle.**
///
/// TODO: this is too convoluted, but that's scene spawning for you
///
/// Component is removed after colliders are spawned.
#[derive(Component)]
pub struct SceneStaticCollider;

pub struct SceneUtilsPlugin;

impl Plugin for SceneUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_static_colliders);
    }
}

fn spawn_static_colliders(
    entities: Query<(Entity, &GlobalTransform), With<SceneStaticCollider>>,
    children: Query<&Children>,
    meshes: Query<(&Handle<Mesh>, &GlobalTransform)>,
    mesh_assets: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    use bevy_rapier3d::prelude::*;

    for (root, root_transform) in entities.iter() {
        let mut colliders = vec![];

        // collect all colliders and their transforms relative to root
        iterate_children_recursively(root, &children, |entity| {
            if let Ok((mesh, child_transform)) = meshes.get(entity) {
                let mesh = mesh_assets.get(mesh).unwrap();
                let collider =
                    Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap();
                let transform = child_transform.reparented_to(root_transform);
                colliders.push((transform, collider));
            }
        });

        if colliders.is_empty() {
            continue;
        }

        commands
            .entity(root)
            .remove::<SceneStaticCollider>()
            .with_children(|parent| {
                for (transform, collider) in colliders {
                    parent.spawn((
                        SpatialBundle::from_transform(transform),
                        RigidBody::Fixed,
                        collider,
                    ));
                }
            });
    }
}
