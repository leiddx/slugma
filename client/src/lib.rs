mod core;
mod plugins;

pub mod event;

use bevy::app::{PluginGroup, PluginGroupBuilder};



pub struct Plugins;

impl PluginGroup for Plugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(core::Plugin)
			.add(plugins::character::Plugin)
			.add(plugins::console::Plugin)
			.add(plugins::input::Plugin)
			.add(plugins::fps::Plugin)
			.add(plugins::window::Plugin)
	}
}
