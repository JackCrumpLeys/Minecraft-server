#![allow(clippy::type_complexity)]

mod db;
mod games;
mod item_helpers;

use valence::client::message::SendMessage;

use surrealdb::error::Api::ConnectionUninitialised;

use valence::log::{error, info};
use valence::prelude::*;

use crate::db::player::Player;
use crate::db::DB;
use crate::games::building::BuildingGame;
use crate::games::{Game, Games};
use crate::games::lobby::{LobbyGame, LobbyInstance};

#[derive(Debug, Clone, Event)]
struct PlayerJoinEvent {
    client: Entity,
    instance: Entity,
    player: Player,
}

#[derive(Debug, Clone, Component)]
struct GameIdentifier(String);

#[derive(Resource)]
struct GamesRunning(Vec<Games>);

impl GamesRunning {
    fn get_game(&self, game_to_find: Games) -> Option<&Games> {
        self.0.iter().find(|game| matches!(game, game_to_find))
    }
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    App::new()
        .insert_resource(GamesRunning(vec![Games::Building(BuildingGame::new()), Games::Lobby(LobbyGame::new())]))
        .add_plugins((DefaultPlugins, BuildingGame, LobbyGame))
        .add_systems(Startup, setup)
        .add_systems(Update, (init_clients, despawn_disconnected_clients))
        .insert_resource(DB {
            runtime: runtime,
            db: None,
        })
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    mut db: ResMut<DB>,
    games: Res<GamesRunning>,
) {
    if let Err(error) = db.connect(false) {
        if let Err(err) = db.handle_error(error) {
            error!("Failed to connect to database: {:?}", err);
        }
    }

    for (i, game) in games.0.iter().enumerate() {
        match game {
            Games::Building(game) => {
                info!("Setting up game {}", BuildingGame::game_name());

                let mut instance = Instance::new(BuildingGame::dimension(), &dimensions, &biomes, &server);

                game.setup_instance(&mut instance);

                commands.spawn((
                    instance,
                    game.instance_marker(),
                    GameIdentifier(format!("{}#{i}", BuildingGame::game_name())),
                ));
            }
            Games::Lobby(game) => {
                info!("Setting up game {}", LobbyGame::game_name());

                let mut instance = Instance::new(LobbyGame::dimension(), &dimensions, &biomes, &server);

                game.setup_instance(&mut instance);

                commands.spawn((
                    instance,
                    LobbyGame::instance_marker(),
                    GameIdentifier(format!("{}#{i}", LobbyGame::game_name())),
                ));
            }
        }
    }
}

fn init_clients(
    mut clients: Query<(Entity, &mut Client, &mut Location, &mut Position, &UniqueId, &Username), Added<Client>>,
    mut db_res: ResMut<DB>,
    lobby_servers: Query<(Entity, &GameIdentifier), With<LobbyInstance>>,
    mut commands: Commands,
) {
    db_res
        .connect(false)
        .expect("Failed to connect to database");
    for (entity, mut client, mut location, mut pos, uuid, username) in &mut clients {
        match db_res.block_on(async {
            let _db = match db_res.db.as_ref() {
                Some(db) => db,
                None => return Err(surrealdb::Error::Api(ConnectionUninitialised)),
            };

            // Create a new player
            match Player::get_or_create(username.0.clone(), uuid.0.to_string(), db_res.as_ref())
                .await
            {
                Ok(mut player) => {
                    player.visited_count += 1;
                    player.save(db_res.as_ref()).await?;

                    client.send_chat_message(format!(
                        "Welcome back to Valence, {}! You have visited {} times.",
                        username.0, player.visited_count
                    ));

                    if !lobby_servers.is_empty() {
                        let lobby = lobby_servers.iter().next().unwrap();
                        info!("Found lobby server: {}", lobby.1.0);
                        location.0 = lobby.0;
                        client.send_chat_message(format!(
                            "connected to {}",
                            lobby.1.0
                        ));

                        pos.0 = LobbyGame::spawn_player_location(player).into();

                        commands.entity(entity).insert(LobbyGame::client_marker());
                    }

                    Ok(())
                }
                Err(err) => {
                    client.send_chat_message(format!(
                        "error: failed to load player {} from database",
                        username.0
                    ));

                    Err(err)
                }
            }
        }) {
            Err(err) => {
                db_res.handle_error(err).expect("failed to handle error");
            }
            _ => {}
        };
    }
}
