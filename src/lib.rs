pub mod udp_socket;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Player;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Resource)]
pub struct GameState {
	pub players: Vec<Player>,
}

impl GameState {
	pub fn new() -> Self {
		Self {
		    players: Vec::new(),
		}
	}
}
