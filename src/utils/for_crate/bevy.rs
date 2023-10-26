//! Utilities for [`bevy`]` crate.

use bevy::{ecs::system::EntityCommands, prelude::*};

/// Iterate over all children of the entity, their children, and so on.
///
/// Order in which entities are passed to callback is unspecified.
pub fn iterate_children_recursively(
    root: Entity,
    children: &Query<&Children>,
    mut callback: impl FnMut(Entity),
) {
    let mut entities = vec![root];
    while let Some(entity) = entities.pop() {
        callback(entity);
        if let Ok(children) = children.get(entity) {
            entities.extend(children.iter());
        }
    }
}

/// Adds methods to [`Commands`] which fail instead of panicking.
// TODO(later): extend this and replace most (all?) uses of Commands as is with
// fallible ones
pub trait FallibleCommands {
    /// Despawn entity and all its children.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_despawn_recursive(&mut self, entity: Entity);

    /// Add components to entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_insert(&mut self, entity: Entity, bundle: impl Bundle);

    /// Remove components from entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_remove<B: Bundle>(&mut self, entity: Entity);

    /// Spawn children to the entity.
    ///
    /// __Functor execution is delayed, so it can't reference local variables.__
    ///
    /// Silently fails if entity doesn't exist.
    fn try_with_children<F: FnOnce(&mut WorldChildBuilder) + Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        spawn_children: F,
    );
}

impl<'w, 's> FallibleCommands for Commands<'w, 's> {
    fn try_despawn_recursive(&mut self, entity: Entity) {
        self.add(move |world: &mut World| {
            if let Some(entity_mut) = world.get_entity_mut(entity) {
                entity_mut.despawn_recursive();
            }
        });
    }

    fn try_insert(&mut self, entity: Entity, bundle: impl Bundle) {
        if let Some(mut commands) = self.get_entity(entity) {
            commands.try_insert(bundle);
        }
    }

    fn try_remove<B: Bundle>(&mut self, entity: Entity) {
        if let Some(mut commands) = self.get_entity(entity) {
            commands.try_remove::<B>();
        }
    }

    fn try_with_children<F: FnOnce(&mut WorldChildBuilder) + Send + Sync + 'static>(
        &mut self,
        entity: Entity,
        spawn_children: F,
    ) {
        if let Some(mut commands) = self.get_entity(entity) {
            commands.try_with_children(spawn_children);
        }
    }
}

/// Adds methods to [`EntityCommands`] which fail instead of panicking.
// Source: https://github.com/Leafwing-Studios/Emergence/pull/765/files#diff-bea0bd142041553803e04ece803b9e2306f73b95d6bfce58893ecb20590bc0da
// MIT/Apache 2.0 license
// TODO: proper attirubution?
// TODO: should this be public or private?
trait FallibleEntityCommands {
    /// Add components to entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_insert(&mut self, bundle: impl Bundle) -> &mut Self;

    /// Remove components from entity.
    ///
    /// Silently fails if entity doesn't exist.
    fn try_remove<B: Bundle>(&mut self) -> &mut Self;

    /// Spawn children to the entity.
    ///
    /// __Functor execution is delayed, so it can't reference local variables.__
    ///
    /// Silently fails if entity doesn't exist.
    fn try_with_children<F: FnOnce(&mut WorldChildBuilder) + Send + Sync + 'static>(
        &mut self,
        spawn_children: F,
    ) -> &mut Self;
}

impl<'w, 's, 'a> FallibleEntityCommands for EntityCommands<'w, 's, 'a> {
    fn try_insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.add(|entity, world: &mut World| {
            if let Some(mut entity_mut) = world.get_entity_mut(entity) {
                entity_mut.insert(bundle);
            }
        });
        self
    }

    fn try_remove<B: Bundle>(&mut self) -> &mut Self {
        self.add(|entity, world: &mut World| {
            if let Some(mut entity_mut) = world.get_entity_mut(entity) {
                entity_mut.remove::<B>();
            }
        });
        self
    }

    fn try_with_children<F: FnOnce(&mut WorldChildBuilder) + Send + Sync + 'static>(
        &mut self,
        spawn_children: F,
    ) -> &mut Self {
        self.add(move |entity, world: &mut World| {
            if let Some(mut entity_mut) = world.get_entity_mut(entity) {
                entity_mut.with_children(spawn_children);
            }
        });
        self
    }
}
