use bevy::prelude::*;
use bevy_test::*;
use crossbeam::channel::bounded;

fn main() {
	let (gamestate_sender, gamestate_receiver) = bounded::<Vec<u8>>(50);
	let (user_message_sender, user_message_receiver) = bounded::<Vec<u8>>(50);

	let networking_thread = std::thread::spawn(move || {
		let target_addresses = Vec::<std::net::SocketAddr>::new();
		let mut server_socket = udp_socket::UdpSocket::new("0.0.0.0:7878").unwrap();

		loop {
			// Receive
			match server_socket.receive_from() {
				Ok((data, addr)) => {
					println!("Received from {}: {:?}", addr, data);
					user_message_sender.send(data.to_vec()).ok();
				}
				Err(_) => continue,
			}

			// Send
			match gamestate_receiver.try_recv() {
				Ok(gamestate) => {
					for address in &target_addresses {
						if let Err(e) = server_socket.send_to(&gamestate, address) {
							eprintln!("send error to {}: {:?}", address, e);
						}
					}
				}
				Err(_) => continue,
			}

			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});

	App::new()
		// .add_plugins(DefaultPlugins)
		.insert_resource(GameStateSender(gamestate_sender))
		.insert_resource(UserMessageReceiver(user_message_receiver))
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
pub struct GameStateSender(crossbeam::channel::Sender<Vec<u8>>);

#[derive(Resource)]
pub struct UserMessageReceiver(crossbeam::channel::Receiver<Vec<u8>>);

pub struct UploadPlugin;

impl Plugin for UploadPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.insert_resource(GameState::new())
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

pub fn send_gamestate(gamestate_sender: Res<GameStateSender>, game_state: Res<GameState>) {
	gamestate_sender.0.send(game_state.serialize()).unwrap();
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

pub fn receive_messages(user_message_receiver: Res<UserMessageReceiver>) {
	while let Ok(m) = user_message_receiver.0.try_recv() {
		println!("received something {:?}", m);
	}
}
