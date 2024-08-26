use bevy::ecs::event::Event;

#[derive(Event)]
pub(crate) struct PromptRefresh(pub String);

#[derive(Event)]
pub(crate) struct PanelRefresh;
