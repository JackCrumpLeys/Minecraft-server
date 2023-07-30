use valence::prelude::*;
use crate::db::player::Player;

use crate::games::Game;

mod systems;

const SPAWN_Y: i32 = 64;

#[derive(Clone, Copy, Component)]
pub struct BuildingInstance;

#[derive(Clone, Copy, Component)]
pub struct BuildingClient;

pub struct BuildingGame;

impl BuildingGame {
    pub fn new() -> Self {
        Self
    }

    pub(crate) fn instance_marker(&self) -> BuildingClient {
        BuildingClient
    }

    fn client_marker(&self) -> BuildingInstance {
        BuildingInstance
    }
}

impl Game for BuildingGame {
    fn game_name() -> &'static str {
        "building"
    }

    fn game_description() -> &'static str {
        "A game where you can build things."
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
                instance.set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
            }
        }
    }

    fn spawn_player_location(player: Player) -> [f64; 3] {
        [0.0, SPAWN_Y as f64 + 1.0, 0.0]
    }
}

impl Plugin for BuildingGame {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (systems::toggle_gamemode_on_sneak, systems::digging, systems::place_blocks));
    }
}
