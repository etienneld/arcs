use specs::prelude::*;
use std::{borrow::Borrow, collections::HashMap};

/// A name that can be looked up later in the [`NameTable`].
///
/// Each [`Name`] should be unique within a [`World`]. Conflicts may mess up the
/// [`NameTable`] bookkeeping and lead to bad lookups.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Name(String);

impl Name {
    pub fn new<S: Into<String>>(name: S) -> Self { Name(name.into()) }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str { self.0.as_ref() }
}

impl Borrow<String> for Name {
    fn borrow(&self) -> &String { &self.0 }
}

impl Borrow<str> for Name {
    fn borrow(&self) -> &str { self.0.as_str() }
}

impl Component for Name {
    type Storage = FlaggedStorage<Name, HashMapStorage<Name>>;
}

/// A global [`Resource`] for looking up an [`Entity`] using its [`Name`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NameTable {
    names: HashMap<Name, Entity>,
}

impl NameTable {
    pub fn get(&self, name: &str) -> Option<Entity> {
        self.names.get(name).copied()
    }

    pub fn iter<'this>(
        &'this self,
    ) -> impl Iterator<Item = (&str, Entity)> + 'this {
        self.names.iter().map(|(name, ent)| (name.as_ref(), *ent))
    }
}
