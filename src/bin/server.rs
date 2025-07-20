use bevy::prelude::*;
use bevy_test::*;

fn main() {
	App::new()
		// .add_plugins(DefaultPlugins)
		.add_plugins(UploadPlugin)
		.add_systems(Startup, add_player)
		.run();
}

pub fn add_player(mut commands: Commands) {
	commands.spawn((Player, Name("Test".to_string())));
	commands.spawn((Player, Name("Jonas".to_string())));
}

// -- test upload part

pub struct UploadPlugin;

impl Plugin for UploadPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.insert_resource(GameState::new())
    		.add_systems(Update, (upload_players).chain())
    	;
    }
}

pub fn upload_players(query: Query<&Player>, mut game_state: ResMut<GameState>) {
	let mut players = Vec::new();
	for player in &query {
		players.push(player.clone());
	}
	game_state.players = players;
}

