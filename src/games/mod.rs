use valence::ecs::component::TableStorage;
use valence::prelude::*;
use crate::db::player::Player;


pub mod building;
pub mod lobby;

pub trait Game {
    fn game_name() -> &'static str;
    fn game_description() -> &'static str;
    fn game_version() -> &'static str;
    fn display() -> ItemStack;
    fn dimension() -> Ident<String>;
    fn setup_instance(&self, instance: &mut Instance);
    fn spawn_player_location(player: Player) -> [f64; 3];
}

pub enum Games {
    Building(building::BuildingGame),
    Lobby(lobby::LobbyGame),
}
