use crate::components::ComponentVtable;
use specs::{Builder as _, Component, Entity, World, WorldExt};
use std::any::Any;

/// A wrapper around a [`World`] which records any changes that are made.
pub struct ChangeRecorder<'world> {
    world: &'world World,
    changeset: Vec<Change>,
}

impl<'world> ChangeRecorder<'world> {
    pub fn new(world: &'world World) -> Self {
        ChangeRecorder {
            world,
            changeset: Vec::new(),
        }
    }

    /// Create a new [`Entity`].
    pub fn create_entity(&mut self) -> Builder<'_> {
        Builder {
            _recorder: self,
            _components: Vec::new(),
        }
    }

    /// Delete an [`Entity`] and any components associated with it.
    pub fn delete_entity(&mut self, entity: Entity) {
        // we need to use our "reflection" mechanism to figure out which
        // components are associated with this entity and get a copy so our
        // backwards closure is able to restore them again.
        let current_components: Vec<_> =
            crate::components::attached_to_entity(self.world, entity).collect();

        self.push_change(
            move |world| {
                world.delete_entity(entity).unwrap();
            },
            move |world| {
                let ent = world.create_entity().build();

                for (vtable, component) in &current_components {
                    vtable.set(world, ent, component);
                }
            },
        )
    }

    /// Associate a new [`Component`] with a particular [`Entity`].
    pub fn set_component<C: Component + Clone>(
        &mut self,
        entity: Entity,
        component: C,
    ) {
        // The forward operation just overwrites the component with the new
        // copy
        let forwards = move |world: &mut World| {
            world
                .write_storage::<C>()
                .insert(entity, component.clone())
                .unwrap();
        };

        // The backward component is a little trickier. We need to retrieve the
        // previous value and (if there was one) revert the entity to that one,
        // otherwise delete the component altogether
        let previous_value = self.get_component::<C>(entity);
        let backwards = move |world: &mut World| match previous_value {
            Some(ref value) => {
                world
                    .write_storage::<C>()
                    .insert(entity, value.clone())
                    .unwrap();
            },
            None => {
                world.write_storage::<C>().remove(entity);
            },
        };

        self.push_change(forwards, backwards);
    }

    /// Look up the [`Component`] associated with an [`Entity`].
    ///
    /// This performs a copy because 9 times out of 10 you'll mutate the value
    /// and pass it to [`ChangeRecorder::set_component()`] anyway... plus
    /// the borrow checker complains because we save where the component is
    /// stored as a local and returning `&C` would lead to dangling pointers.
    pub fn get_component<C: Component + Clone>(
        &self,
        entity: Entity,
    ) -> Option<C> {
        self.world.read_storage().get(entity).cloned()
    }

    /// Removes a [`Component`] from this [`Entity`].
    pub fn delete_component<C: Component + Clone>(&mut self, entity: Entity) {
        let previous_value = self.get_component::<C>(entity);

        self.push_change(
            move |world| {
                world.write_storage::<C>().remove(entity);
            },
            move |world| {
                if let Some(ref previous_value) = previous_value {
                    world
                        .write_storage::<C>()
                        .insert(entity, previous_value.clone())
                        .unwrap();
                }
            },
        )
    }

    /// Add a new [`Change`] to the `changeset` which will invoke `forwards` to
    /// apply a diff and `backwards` to revert it.
    fn push_change<F, B>(&mut self, forwards: F, backwards: B)
    where
        F: Fn(&mut World) + 'static,
        B: Fn(&mut World) + 'static,
    {
        self.changeset.push(Change {
            forward: Box::new(forwards),
            backward: Box::new(backwards),
        });
    }

    /// Extract the list of [`Change`]s.
    pub fn into_changes(self) -> ChangeSet {
        ChangeSet {
            changes: self.changeset,
        }
    }
}

pub struct ChangeSet {
    changes: Vec<Change>,
}

impl ChangeSet {
    pub fn apply(&self, world: &mut World) {
        for change in &self.changes {
            change.apply(world);
        }
    }

    pub fn revert(&self, world: &mut World) {
        for change in self.changes.iter().rev() {
            change.revert(world);
        }
    }
}

pub struct Builder<'recorder> {
    _recorder: &'recorder ChangeRecorder<'recorder>,
    _components: Vec<(&'static ComponentVtable, Box<dyn Any>)>,
}

/// A single change.
pub(crate) struct Change {
    forward: Box<dyn Fn(&mut World)>,
    backward: Box<dyn Fn(&mut World)>,
}

impl Change {
    pub fn apply(&self, world: &mut World) { (self.forward)(world); }

    pub fn revert(&self, world: &mut World) { (self.backward)(world); }
}
