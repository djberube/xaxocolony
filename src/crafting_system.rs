use bevy::prelude::*;
use crate::prelude::*;
use crate::inventory_system::*;

// ============================================================================
// CRAFTING SYSTEM - Raft/AoE2 style with tech tree requirements
// ============================================================================

#[derive(Clone)]
pub struct CraftingRecipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub output_item: String,
    pub output_quantity: u32,
    pub required_items: Vec<(String, u32)>, // (item_id, quantity)
    pub required_skill: Option<(String, i32)>, // (skill_name, level)
    pub required_tech: Option<String>, // Tech tree requirement
    pub crafting_time: f32, // In seconds
    pub crafting_station: Option<CraftingStation>,
    pub skill_xp_gained: i32,
}

#[derive(Clone, PartialEq)]
pub enum CraftingStation {
    Workbench,
    Forge,
    Anvil,
    CookingPot,
    Loom,
    TanningRack,
    Furnace,
    Laboratory,
    Sawmill,
    None, // Can craft anywhere
}

#[derive(Component)]
pub struct CraftingQueue {
    pub queue: Vec<CraftingTask>,
}

pub struct CraftingTask {
    pub recipe: CraftingRecipe,
    pub time_remaining: f32,
    pub started: bool,
}

// ============================================================================
// RECIPE DATABASE
// ============================================================================

pub struct RecipeDatabase;

impl RecipeDatabase {
    pub fn get_all_recipes() -> Vec<CraftingRecipe> {
        vec![
            // BASIC TOOLS
            CraftingRecipe {
                id: "craft_wooden_club".to_string(),
                name: "Wooden Club".to_string(),
                description: "Craft a basic weapon".to_string(),
                output_item: "wooden_club".to_string(),
                output_quantity: 1,
                required_items: vec![("wood".to_string(), 5)],
                required_skill: None,
                required_tech: None,
                crafting_time: 10.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 20,
            },
            CraftingRecipe {
                id: "craft_iron_axe".to_string(),
                name: "Iron Axe".to_string(),
                description: "Craft an efficient chopping tool".to_string(),
                output_item: "iron_axe".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 3),
                    ("wood".to_string(), 2),
                ],
                required_skill: Some(("crafting".to_string(), 3)),
                required_tech: Some("ironworking".to_string()),
                crafting_time: 30.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 50,
            },
            CraftingRecipe {
                id: "craft_iron_pickaxe".to_string(),
                name: "Iron Pickaxe".to_string(),
                description: "Craft a mining tool".to_string(),
                output_item: "iron_pickaxe".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 4),
                    ("wood".to_string(), 2),
                ],
                required_skill: Some(("crafting".to_string(), 3)),
                required_tech: Some("ironworking".to_string()),
                crafting_time: 35.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 50,
            },
            CraftingRecipe {
                id: "craft_fishing_rod".to_string(),
                name: "Fishing Rod".to_string(),
                description: "Craft a fishing tool".to_string(),
                output_item: "fishing_rod".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("wood".to_string(), 3),
                    ("cloth".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 1)),
                required_tech: None,
                crafting_time: 15.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 30,
            },

            // WEAPONS
            CraftingRecipe {
                id: "craft_spear".to_string(),
                name: "Spear".to_string(),
                description: "Craft a long-reach weapon".to_string(),
                output_item: "spear".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("wood".to_string(), 2),
                    ("iron_bar".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 2)),
                required_tech: None,
                crafting_time: 20.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 40,
            },
            CraftingRecipe {
                id: "craft_iron_sword".to_string(),
                name: "Iron Sword".to_string(),
                description: "Craft a sturdy blade".to_string(),
                output_item: "iron_sword".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 5),
                    ("leather".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 5)),
                required_tech: Some("ironworking".to_string()),
                crafting_time: 60.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 100,
            },
            CraftingRecipe {
                id: "craft_war_axe".to_string(),
                name: "War Axe".to_string(),
                description: "Craft a powerful battle axe".to_string(),
                output_item: "war_axe".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 6),
                    ("wood".to_string(), 3),
                ],
                required_skill: Some(("crafting".to_string(), 7)),
                required_tech: Some("advanced_weaponry".to_string()),
                crafting_time: 80.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 150,
            },
            CraftingRecipe {
                id: "craft_bow".to_string(),
                name: "Hunting Bow".to_string(),
                description: "Craft a ranged weapon".to_string(),
                output_item: "bow".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("wood".to_string(), 4),
                    ("cloth".to_string(), 2),
                ],
                required_skill: Some(("crafting".to_string(), 4)),
                required_tech: Some("archery".to_string()),
                crafting_time: 45.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 80,
            },
            CraftingRecipe {
                id: "craft_crossbow".to_string(),
                name: "Crossbow".to_string(),
                description: "Craft a powerful crossbow".to_string(),
                output_item: "crossbow".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("wood".to_string(), 5),
                    ("iron_bar".to_string(), 4),
                    ("cloth".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 8)),
                required_tech: Some("advanced_archery".to_string()),
                crafting_time: 120.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 200,
            },

            // AMMO
            CraftingRecipe {
                id: "craft_arrows".to_string(),
                name: "Arrows".to_string(),
                description: "Craft arrows for bows".to_string(),
                output_item: "arrow".to_string(),
                output_quantity: 10,
                required_items: vec![
                    ("wood".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 2)),
                required_tech: Some("archery".to_string()),
                crafting_time: 10.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 10,
            },
            CraftingRecipe {
                id: "craft_bolts".to_string(),
                name: "Crossbow Bolts".to_string(),
                description: "Craft bolts for crossbows".to_string(),
                output_item: "bolt".to_string(),
                output_quantity: 10,
                required_items: vec![
                    ("wood".to_string(), 1),
                    ("iron_bar".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 4)),
                required_tech: Some("advanced_archery".to_string()),
                crafting_time: 15.0,
                crafting_station: Some(CraftingStation::Workbench),
                skill_xp_gained: 15,
            },

            // ARMOR
            CraftingRecipe {
                id: "craft_leather_helmet".to_string(),
                name: "Leather Helmet".to_string(),
                description: "Craft basic head protection".to_string(),
                output_item: "leather_helmet".to_string(),
                output_quantity: 1,
                required_items: vec![("leather".to_string(), 3)],
                required_skill: Some(("crafting".to_string(), 2)),
                required_tech: None,
                crafting_time: 25.0,
                crafting_station: Some(CraftingStation::TanningRack),
                skill_xp_gained: 40,
            },
            CraftingRecipe {
                id: "craft_leather_armor".to_string(),
                name: "Leather Armor".to_string(),
                description: "Craft light body armor".to_string(),
                output_item: "leather_armor".to_string(),
                output_quantity: 1,
                required_items: vec![("leather".to_string(), 8)],
                required_skill: Some(("crafting".to_string(), 3)),
                required_tech: None,
                crafting_time: 50.0,
                crafting_station: Some(CraftingStation::TanningRack),
                skill_xp_gained: 80,
            },
            CraftingRecipe {
                id: "craft_iron_helmet".to_string(),
                name: "Iron Helmet".to_string(),
                description: "Craft strong head protection".to_string(),
                output_item: "iron_helmet".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 5),
                    ("leather".to_string(), 1),
                ],
                required_skill: Some(("crafting".to_string(), 6)),
                required_tech: Some("ironworking".to_string()),
                crafting_time: 70.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 120,
            },
            CraftingRecipe {
                id: "craft_iron_armor".to_string(),
                name: "Iron Armor".to_string(),
                description: "Craft heavy body armor".to_string(),
                output_item: "iron_armor".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("iron_bar".to_string(), 15),
                    ("leather".to_string(), 3),
                ],
                required_skill: Some(("crafting".to_string(), 8)),
                required_tech: Some("ironworking".to_string()),
                crafting_time: 150.0,
                crafting_station: Some(CraftingStation::Forge),
                skill_xp_gained: 250,
            },

            // CLOTHING (The Long Dark style)
            CraftingRecipe {
                id: "craft_wool_hat".to_string(),
                name: "Wool Hat".to_string(),
                description: "Craft warm headwear".to_string(),
                output_item: "wool_hat".to_string(),
                output_quantity: 1,
                required_items: vec![("cloth".to_string(), 3)],
                required_skill: Some(("crafting".to_string(), 1)),
                required_tech: None,
                crafting_time: 30.0,
                crafting_station: Some(CraftingStation::Loom),
                skill_xp_gained: 30,
            },
            CraftingRecipe {
                id: "craft_winter_coat".to_string(),
                name: "Winter Coat".to_string(),
                description: "Craft heavy winter protection".to_string(),
                output_item: "winter_coat".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("cloth".to_string(), 10),
                    ("leather".to_string(), 2),
                ],
                required_skill: Some(("crafting".to_string(), 5)),
                required_tech: None,
                crafting_time: 120.0,
                crafting_station: Some(CraftingStation::Loom),
                skill_xp_gained: 150,
            },
            CraftingRecipe {
                id: "craft_leather_gloves".to_string(),
                name: "Leather Gloves".to_string(),
                description: "Craft hand protection".to_string(),
                output_item: "leather_gloves".to_string(),
                output_quantity: 1,
                required_items: vec![("leather".to_string(), 2)],
                required_skill: Some(("crafting".to_string(), 2)),
                required_tech: None,
                crafting_time: 20.0,
                crafting_station: Some(CraftingStation::TanningRack),
                skill_xp_gained: 30,
            },
            CraftingRecipe {
                id: "craft_boots".to_string(),
                name: "Leather Boots".to_string(),
                description: "Craft sturdy footwear".to_string(),
                output_item: "boots".to_string(),
                output_quantity: 1,
                required_items: vec![("leather".to_string(), 4)],
                required_skill: Some(("crafting".to_string(), 3)),
                required_tech: None,
                crafting_time: 40.0,
                crafting_station: Some(CraftingStation::TanningRack),
                skill_xp_gained: 50,
            },

            // RESOURCES
            CraftingRecipe {
                id: "smelt_iron_bar".to_string(),
                name: "Smelt Iron Bar".to_string(),
                description: "Smelt ore into iron bars".to_string(),
                output_item: "iron_bar".to_string(),
                output_quantity: 1,
                required_items: vec![("iron_ore".to_string(), 2)],
                required_skill: Some(("crafting".to_string(), 2)),
                required_tech: Some("smelting".to_string()),
                crafting_time: 20.0,
                crafting_station: Some(CraftingStation::Furnace),
                skill_xp_gained: 20,
            },
            CraftingRecipe {
                id: "craft_cloth".to_string(),
                name: "Weave Cloth".to_string(),
                description: "Weave plant fibers into cloth".to_string(),
                output_item: "cloth".to_string(),
                output_quantity: 1,
                required_items: vec![("wood".to_string(), 3)], // Placeholder - should be plant fibers
                required_skill: Some(("crafting".to_string(), 1)),
                required_tech: None,
                crafting_time: 15.0,
                crafting_station: Some(CraftingStation::Loom),
                skill_xp_gained: 15,
            },

            // FOOD/CONSUMABLES
            CraftingRecipe {
                id: "cook_meat".to_string(),
                name: "Cook Meat".to_string(),
                description: "Cook raw meat".to_string(),
                output_item: "cooked_meat".to_string(),
                output_quantity: 1,
                required_items: vec![("raw_meat".to_string(), 1)],
                required_skill: Some(("cooking".to_string(), 1)),
                required_tech: None,
                crafting_time: 10.0,
                crafting_station: Some(CraftingStation::CookingPot),
                skill_xp_gained: 10,
            },
            CraftingRecipe {
                id: "craft_bandage".to_string(),
                name: "Craft Bandage".to_string(),
                description: "Make basic medical supplies".to_string(),
                output_item: "bandage".to_string(),
                output_quantity: 3,
                required_items: vec![("cloth".to_string(), 1)],
                required_skill: Some(("crafting".to_string(), 1)),
                required_tech: None,
                crafting_time: 5.0,
                crafting_station: None,
                skill_xp_gained: 10,
            },
            CraftingRecipe {
                id: "craft_health_potion".to_string(),
                name: "Craft Health Potion".to_string(),
                description: "Brew a healing potion".to_string(),
                output_item: "health_potion".to_string(),
                output_quantity: 1,
                required_items: vec![
                    ("cloth".to_string(), 1),
                    ("wood".to_string(), 2), // Placeholder - should be herbs
                ],
                required_skill: Some(("crafting".to_string(), 5)),
                required_tech: Some("alchemy".to_string()),
                crafting_time: 30.0,
                crafting_station: Some(CraftingStation::Laboratory),
                skill_xp_gained: 50,
            },
            CraftingRecipe {
                id: "bake_bread".to_string(),
                name: "Bake Bread".to_string(),
                description: "Bake bread from grain".to_string(),
                output_item: "bread".to_string(),
                output_quantity: 2,
                required_items: vec![("wood".to_string(), 1)], // Placeholder - should be grain/flour
                required_skill: Some(("cooking".to_string(), 2)),
                required_tech: None,
                crafting_time: 15.0,
                crafting_station: Some(CraftingStation::CookingPot),
                skill_xp_gained: 15,
            },
        ]
    }

    pub fn get_recipe(recipe_id: &str) -> Option<CraftingRecipe> {
        Self::get_all_recipes()
            .into_iter()
            .find(|r| r.id == recipe_id)
    }

    pub fn can_craft(
        recipe: &CraftingRecipe,
        inventory: &Inventory,
        skillset: &Skillset,
        unlocked_techs: &Vec<String>,
    ) -> (bool, String) {
        // Check tech requirement
        if let Some(required_tech) = &recipe.required_tech {
            if !unlocked_techs.contains(required_tech) {
                return (false, format!("Requires technology: {}", required_tech));
            }
        }

        // Check skill requirement
        if let Some((skill_name, required_level)) = &recipe.required_skill {
            let skill_level = match skill_name.as_str() {
                "crafting" => skillset.crafting.level(),
                "cooking" => skillset.cooking.level(),
                _ => 0,
            };
            if skill_level < *required_level {
                return (
                    false,
                    format!("Requires {} level {}", skill_name, required_level),
                );
            }
        }

        // Check materials
        for (item_id, required_qty) in &recipe.required_items {
            if !inventory.has_item(item_id, *required_qty) {
                return (
                    false,
                    format!(
                        "Missing: {} x{}",
                        item_id,
                        required_qty - inventory.count_item(item_id)
                    ),
                );
            }
        }

        (true, "Can craft!".to_string())
    }
}

// ============================================================================
// CRAFTING STATION COMPONENT
// ============================================================================

#[derive(Component)]
pub struct CraftingStationEntity {
    pub station_type: CraftingStation,
    pub in_use: bool,
    pub current_crafter: Option<Entity>,
}

// ============================================================================
// CRAFTING PLUGIN
// ============================================================================

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            crafting_progress_system.run_if(bevy::time::common_conditions::on_timer(
                bevy::utils::Duration::from_secs_f32(1.0),
            )),
        );
    }
}

fn crafting_progress_system(
    mut query: Query<(&mut CraftingQueue, &mut Inventory, &mut PhysicalBody)>,
    time: Res<Time>,
) {
    for (mut queue, mut inventory, mut body) in query.iter_mut() {
        if queue.queue.is_empty() {
            continue;
        }

        let task = &mut queue.queue[0];

        // Start task if not started
        if !task.started {
            // Consume materials
            let mut can_start = true;
            for (item_id, qty) in &task.recipe.required_items {
                if !inventory.has_item(item_id, *qty) {
                    can_start = false;
                    break;
                }
            }

            if can_start {
                // Remove materials
                for (item_id, qty) in &task.recipe.required_items {
                    inventory.remove_item(item_id, *qty);
                }
                task.started = true;
            } else {
                // Can't start, remove from queue
                queue.queue.remove(0);
                continue;
            }
        }

        // Progress the task
        task.time_remaining -= 1.0; // 1 second per tick

        // Complete the task
        if task.time_remaining <= 0.0 {
            // Add output item
            if let Some(item) = ItemDatabase::create_item(
                &task.recipe.output_item,
                task.recipe.output_quantity,
            ) {
                inventory.add_item(item);
            }

            // Grant skill XP
            body.skillset.crafting.experience += task.recipe.skill_xp_gained;

            // Remove task from queue
            queue.queue.remove(0);
        }
    }
}
