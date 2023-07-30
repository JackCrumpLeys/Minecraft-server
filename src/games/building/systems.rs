use valence::aabb::Aabb;
use valence::client::interact_block::InteractBlockEvent;
use valence::entity::living::LivingEntity;

use valence::inventory::HeldItem;
use valence::prelude::*;
use crate::games::building::{BuildingClient, BuildingInstance};
use crate::item_helpers::is_spawn_egg;

pub(crate) fn place_blocks(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem), With<BuildingClient>>,
    entity_hitboxes: Query<&Hitbox, With<LivingEntity>>,
    mut instances: Query<&mut Instance, With<BuildingInstance>>,
    mut events: EventReader<InteractBlockEvent>,
) {
    for mut instance in instances.iter_mut() {
        'events: for event in events.iter() {
            let Ok((mut inventory, game_mode, held)) = clients.get_mut(event.client) else {
                continue;
            };

            // If the entity is in the way of the block, don't place it.
            let block_pos = event.position.get_in_direction(event.face);

            let bx = block_pos.x as f64;
            let by = block_pos.y as f64;
            let bz = block_pos.z as f64;

            let block_aabb = Aabb::new([bx, by, bz], [bx + 1.0, by + 1.0, bz + 1.0]);

            for hitbox in entity_hitboxes.iter() {
                let mut entity_aabb = hitbox.get();

                entity_aabb.min.y += 0.1; // add a little bit of leeway because otherwise you cannot
                // place blocks below you

                // if the entity is in the way of the block, don't place it.
                if entity_aabb.intersects(block_aabb) {
                    continue 'events;
                }
            }

            if event.hand != Hand::Main {
                if let Some(item) = inventory.slot(held.slot()) {
                    // if the player is holding a placeable item in their hand, don't place from offhand
                    if is_spawn_egg(&item.item) // spawn eggs
                        || BlockKind::from_item_kind(item.item).is_some()
                    {
                        continue;
                    }
                }
            }

            // get the held item
            let slot_id = if event.hand != Hand::Main {
                45
            } else {
                held.slot()
            };
            let Some(stack) = inventory.slot(slot_id) else {
                // no item in the slot
                continue;
            };

            let Some(block_kind) = BlockKind::from_item_kind(stack.item) else {
                // can't place this item as a block
                continue;
            };

            if *game_mode == GameMode::Survival {
                // check if the player has the item in their inventory and remove
                // it.
                if stack.count() > 1 {
                    let count = stack.count();
                    inventory.set_slot_amount(slot_id, count - 1);
                } else {
                    inventory.set_slot(slot_id, None);
                }
            }
            instance.set_block(block_pos, block_kind.to_state());
        }
    }
}

pub(crate) fn toggle_gamemode_on_sneak(
    mut clients: Query<&mut GameMode, With<BuildingClient>>,
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

pub(crate) fn digging(
    clients: Query<&GameMode, With<BuildingClient>>,
    mut instances: Query<&mut Instance, With<BuildingInstance>>,
    mut events: EventReader<DiggingEvent>,
) {
    for mut instance in instances.iter_mut() {
        for event in events.iter() {
            let Ok(game_mode) = clients.get(event.client) else {
                continue;
            };
            if (*game_mode == GameMode::Creative && event.state == DiggingState::Start)
                || (*game_mode == GameMode::Survival && event.state == DiggingState::Stop)
            {
                instance.set_block(event.position, BlockState::AIR);
            }
        }
    }
}

