use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Person;

#[derive(Component)]
pub(crate) struct Name(pub(crate) String);