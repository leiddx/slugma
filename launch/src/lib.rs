pub mod camera;
pub mod event;
pub mod state;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct Plugins;

impl PluginGroup for Plugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>().add(camera::Plugin)
	}
}
