use valence::client::interact_block::InteractBlockEvent;
use valence::inventory::ClickSlotEvent;
use valence::prelude::*;
use crate::games::lobby::{CHEST_POS, LobbyClient};

#[derive(Clone, Copy, Component)]
pub struct Menu;

pub fn setup_chest_inventory(
    mut commands: Commands,
) {
    let mut inventory = Inventory::with_title(
        InventoryKind::Generic9x3,
        "Extra".italic() + " Chesty".not_italic().bold().color(Color::RED) + " Chest".not_italic(),
    );

    let slot_count = inventory.slots().len();

    for i in 0..slot_count {
        inventory.set_slot(i as u16, ItemStack::new(ItemKind::Stone, 1, None));
    }

    commands.spawn((inventory, Menu));
}

pub fn open_chest(
    mut commands: Commands,
    inventories: Query<Entity, (With<Inventory>, With<Menu>, Without<Client>)>,
    mut events: EventReader<InteractBlockEvent>,
    lobby_clients: Query<(), With<LobbyClient>>,
) {
    for event in events.iter() {
        if event.position != CHEST_POS.into() {
            continue;
        }
        if lobby_clients.get(event.client).is_err() {
            continue;
        }
        let open_inventory = OpenInventory::new(inventories.single());
        commands.entity(event.client).insert(open_inventory);
    }
}

pub fn toggle_gamemode_on_sneak(
    mut clients: Query<&mut GameMode, With<LobbyClient>>,
    mut events: EventReader<SneakEvent>,
) {
    for event in events.iter() {
        let Ok(mut mode) = clients.get_mut(event.client) else {
            continue;
        };

        if event.state == SneakState::Start {
            *mode = match *mode {
                GameMode::Survival => GameMode::Creative,
                GameMode::Creative => GameMode::Survival,
                _ => GameMode::Creative,
            };
        }
    }
}

pub fn slot_click_handler(
    mut events: EventReader<ClickSlotEvent>,
    _inventories: Query<&mut Inventory, With<Menu>>,
) {
    for event in events.iter() {
        println!("Slot click event: {:#?}", event);
    }
}
