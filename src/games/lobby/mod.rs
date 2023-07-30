use crate::db::player::Player;
use valence::prelude::*;

use crate::games::lobby::systems::{open_chest, setup_chest_inventory, slot_click_handler, toggle_gamemode_on_sneak};
use crate::games::Game;

mod systems;

const SPAWN_Y: i32 = 64;
const CHEST_POS: [i32; 3] = [0, SPAWN_Y + 1, 3];

#[derive(Clone, Copy, Component)]
pub struct LobbyInstance;

#[derive(Clone, Copy, Component)]
pub struct LobbyClient;

pub struct LobbyGame;

impl LobbyGame {
    pub fn new() -> Self {
        Self
    }

    pub(crate) fn instance_marker() -> LobbyInstance {
        LobbyInstance
    }

    pub(crate) fn client_marker() -> LobbyClient {
        LobbyClient
    }
}

impl Game for LobbyGame {
    fn game_name() -> &'static str {
        "lobby"
    }

    fn game_description() -> &'static str {
        "A lobby where you can join games."
    }

    fn game_version() -> &'static str {
        "0.1.0"
    }

    fn display() -> ItemStack {
        ItemStack::new(ItemKind::Stone, 1, None)
    }

    fn dimension() -> Ident<String> {
        ident!("overworld").into()
    }

    fn setup_instance(&self, instance: &mut Instance) {
        for z in -5..5 {
            for x in -5..5 {
                instance.insert_chunk([x, z], UnloadedChunk::new());
            }
        }

        for z in -25..25 {
            for x in -25..25 {
                instance.set_block([x, SPAWN_Y, z], BlockState::STONE);
            }
        }

        instance.set_block(CHEST_POS, BlockState::CHEST);
    }

    fn spawn_player_location(player: Player) -> [f64; 3] {
        [0.0, SPAWN_Y as f64 + 1.0, 0.0]
    }
}

impl Plugin for LobbyGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chest_inventory)
            .add_systems(Update, (open_chest, toggle_gamemode_on_sneak, slot_click_handler));
    }
}
