use bevy::prelude::*;
use bevy_test::*;

fn main() {
	let receive_thread = std::thread::spawn(|| {
		
	});

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

// -- upload part

#[derive(Resource)]
pub struct ServerSocket(pub udp_socket::UdpSocket);

#[derive(Component)]
pub struct TargetSockets(std::net::SocketAddr);

pub struct UploadPlugin;

impl Plugin for UploadPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.insert_resource(GameState::new())
    		.insert_resource(ServerSocket(udp_socket::UdpSocket::new("0.0.0.0:7878").unwrap()))
    		.add_systems(Update, (upload_players, send_gamestate).chain())
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

pub fn send_gamestate(server_socket: Res<ServerSocket>, targets: Query<&TargetSockets>, game_state: Res<GameState>) {
	for target in &targets {
		server_socket.0.send_to(&game_state.serialize(), &target.0);
	}
}

// downlaod part


pub struct ReceivePlugin;

impl Plugin for ReceivePlugin {
    fn build(&self, app: &mut App) {
    	app
    		.add_systems(Update, receive_messages)
    	;
    }
}

pub fn receive_messages(server_socket: ResMut<ServerSocket>) {
	
}
