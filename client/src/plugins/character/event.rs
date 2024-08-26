use bevy::ecs::event::Event;


#[derive(Event, Debug)]
pub struct Update(pub String);
