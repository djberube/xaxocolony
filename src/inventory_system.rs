use bevy::prelude::*;
use crate::prelude::*;

// ============================================================================
// INVENTORY SYSTEM - Grid-based inventory like Raft/The Long Dark
// ============================================================================

#[derive(Component, Clone)]
pub struct Inventory {
    pub slots: Vec<Option<InventoryItem>>,
    pub max_slots: usize,
    pub weight_capacity: f32,
    pub current_weight: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            slots: vec![None; 20], // 20 slots by default
            max_slots: 20,
            weight_capacity: 100.0,
            current_weight: 0.0,
        }
    }
}

impl Inventory {
    pub fn new(max_slots: usize, weight_capacity: f32) -> Self {
        Inventory {
            slots: vec![None; max_slots],
            max_slots,
            weight_capacity,
            current_weight: 0.0,
        }
    }

    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        // Try to stack first
        for slot in self.slots.iter_mut() {
            if let Some(existing) = slot {
                if existing.item_def.id == item.item_def.id && existing.item_def.stackable {
                    if existing.quantity + item.quantity <= item.item_def.max_stack {
                        existing.quantity += item.quantity;
                        self.current_weight += item.item_def.weight * item.quantity as f32;
                        return true;
                    }
                }
            }
        }

        // Find empty slot
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                if self.current_weight + item.item_def.weight * item.quantity as f32 <= self.weight_capacity {
                    self.current_weight += item.item_def.weight * item.quantity as f32;
                    *slot = Some(item);
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        for slot in self.slots.iter_mut() {
            if let Some(item) = slot {
                if item.item_def.id == item_id {
                    if item.quantity >= quantity {
                        item.quantity -= quantity;
                        self.current_weight -= item.item_def.weight * quantity as f32;
                        if item.quantity == 0 {
                            *slot = None;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        let mut total = 0;
        for slot in &self.slots {
            if let Some(item) = slot {
                if item.item_def.id == item_id {
                    total += item.quantity;
                }
            }
        }
        total >= quantity
    }

    pub fn count_item(&self, item_id: &str) -> u32 {
        let mut total = 0;
        for slot in &self.slots {
            if let Some(item) = slot {
                if item.item_def.id == item_id {
                    total += item.quantity;
                }
            }
        }
        total
    }
}

#[derive(Clone)]
pub struct InventoryItem {
    pub item_def: ItemDefinition,
    pub quantity: u32,
    pub condition: f32, // 0.0 to 1.0, for gear degradation (The Long Dark style)
    pub data: ItemData, // Additional data per item type
}

#[derive(Clone)]
pub struct ItemDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_class: ItemClass,
    pub weight: f32,
    pub value: i32,
    pub stackable: bool,
    pub max_stack: u32,
    pub sprite_index: usize,
}

#[derive(Clone, PartialEq)]
pub enum ItemClass {
    Weapon(WeaponStats),
    Armor(ArmorStats),
    Tool(ToolStats),
    Clothing(ClothingStats),
    Food(FoodStats),
    Resource,
    Consumable(ConsumableStats),
    Ammo(AmmoStats),
}

#[derive(Clone, PartialEq)]
pub struct WeaponStats {
    pub damage: i32,
    pub attack_speed: f32,
    pub range: f32,
    pub weapon_type: WeaponType,
    pub durability: f32,
    pub required_skill: i32,
    pub ammo_type: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum WeaponType {
    Melee,
    Railgun,
    GaussRifle,
    Gun,
    PlasmaLance,
    Blade,
    PowerAxe,
    Hammer,
}

#[derive(Clone, PartialEq)]
pub struct ArmorStats {
    pub defense: i32,
    pub armor_type: ArmorType,
    pub warmth: f32, // The Long Dark style temperature
    pub durability: f32,
}

#[derive(Clone, PartialEq)]
pub enum ArmorType {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    Shield,
}

#[derive(Clone, PartialEq)]
pub struct ToolStats {
    pub tool_type: ToolType,
    pub efficiency: f32,
    pub durability: f32,
}

#[derive(Clone, PartialEq)]
pub enum ToolType {
    Axe,
    Pickaxe,
    Hammer,
    Saw,
    Hoe,
    FishingRod,
    Knife,
}

#[derive(Clone, PartialEq)]
pub struct ClothingStats {
    pub warmth: f32,
    pub waterproof: f32,
    pub windproof: f32,
    pub slot: ClothingSlot,
}

#[derive(Clone, PartialEq)]
pub enum ClothingSlot {
    Head,
    Face,
    Chest,
    Hands,
    Legs,
    Feet,
    Accessory,
}

#[derive(Clone, PartialEq)]
pub struct FoodStats {
    pub nutrition: f32,
    pub calories: i32, // The Long Dark style
    pub hydration: f32,
    pub spoilage_rate: f32,
    pub cooking_required: bool,
}

#[derive(Clone, PartialEq)]
pub struct ConsumableStats {
    pub effect: ConsumableEffect,
    pub duration: f32,
}

#[derive(Clone, PartialEq)]
pub enum ConsumableEffect {
    Healing(i32),
    Warmth(f32),
    Energy(f32),
    Buff(String),
}

#[derive(Clone, PartialEq)]
pub struct AmmoStats {
    pub ammo_type: String,
    pub damage_modifier: f32,
}

#[derive(Clone)]
pub enum ItemData {
    None,
    Food { spoilage: f32 },
    Tool { uses_remaining: i32 },
    Weapon { durability_current: f32 },
    Armor { durability_current: f32 },
}

// ============================================================================
// EQUIPMENT SYSTEM
// ============================================================================

#[derive(Component, Clone)]
pub struct Equipment {
    pub weapon_main: Option<InventoryItem>,
    pub weapon_offhand: Option<InventoryItem>,
    pub armor_head: Option<InventoryItem>,
    pub armor_chest: Option<InventoryItem>,
    pub armor_legs: Option<InventoryItem>,
    pub armor_feet: Option<InventoryItem>,
    pub armor_hands: Option<InventoryItem>,
    pub clothing_head: Option<InventoryItem>,
    pub clothing_chest: Option<InventoryItem>,
    pub clothing_legs: Option<InventoryItem>,
    pub clothing_feet: Option<InventoryItem>,
    pub clothing_hands: Option<InventoryItem>,
    pub accessory1: Option<InventoryItem>,
    pub accessory2: Option<InventoryItem>,
}

impl Default for Equipment {
    fn default() -> Self {
        Equipment {
            weapon_main: None,
            weapon_offhand: None,
            armor_head: None,
            armor_chest: None,
            armor_legs: None,
            armor_feet: None,
            armor_hands: None,
            clothing_head: None,
            clothing_chest: None,
            clothing_legs: None,
            clothing_feet: None,
            clothing_hands: None,
            accessory1: None,
            accessory2: None,
        }
    }
}

impl Equipment {
    pub fn total_defense(&self) -> i32 {
        let mut total = 0;
        if let Some(item) = &self.armor_head {
            if let ItemClass::Armor(stats) = &item.item_def.item_class {
                total += stats.defense;
            }
        }
        if let Some(item) = &self.armor_chest {
            if let ItemClass::Armor(stats) = &item.item_def.item_class {
                total += stats.defense;
            }
        }
        if let Some(item) = &self.armor_legs {
            if let ItemClass::Armor(stats) = &item.item_def.item_class {
                total += stats.defense;
            }
        }
        if let Some(item) = &self.armor_feet {
            if let ItemClass::Armor(stats) = &item.item_def.item_class {
                total += stats.defense;
            }
        }
        if let Some(item) = &self.armor_hands {
            if let ItemClass::Armor(stats) = &item.item_def.item_class {
                total += stats.defense;
            }
        }
        total
    }

    pub fn total_warmth(&self) -> f32 {
        let mut total = 0.0;
        // From armor
        for slot in [&self.armor_head, &self.armor_chest, &self.armor_legs, &self.armor_feet, &self.armor_hands] {
            if let Some(item) = slot {
                if let ItemClass::Armor(stats) = &item.item_def.item_class {
                    total += stats.warmth;
                }
            }
        }
        // From clothing
        for slot in [&self.clothing_head, &self.clothing_chest, &self.clothing_legs, &self.clothing_feet, &self.clothing_hands] {
            if let Some(item) = slot {
                if let ItemClass::Clothing(stats) = &item.item_def.item_class {
                    total += stats.warmth;
                }
            }
        }
        total
    }

    pub fn weapon_damage(&self) -> i32 {
        if let Some(item) = &self.weapon_main {
            if let ItemClass::Weapon(stats) = &item.item_def.item_class {
                return stats.damage;
            }
        }
        0
    }
}

// ============================================================================
// ITEM DATABASE - Pre-defined items
// ============================================================================

pub struct ItemDatabase;

impl ItemDatabase {
    pub fn create_item(id: &str, quantity: u32) -> Option<InventoryItem> {
        let def = Self::get_definition(id)?;
        Some(InventoryItem {
            item_def: def.clone(),
            quantity,
            condition: 1.0,
            data: match def.item_class {
                ItemClass::Food(_) => ItemData::Food { spoilage: 1.0 },
                ItemClass::Tool(_) => ItemData::Tool { uses_remaining: 100 },
                ItemClass::Weapon(_) => ItemData::Weapon { durability_current: 1.0 },
                ItemClass::Armor(_) => ItemData::Armor { durability_current: 1.0 },
                _ => ItemData::None,
            },
        })
    }

    pub fn get_definition(id: &str) -> Option<ItemDefinition> {
        match id {
            // RESOURCES
            "wood" => Some(ItemDefinition {
                id: "wood".to_string(),
                name: "Wood".to_string(),
                description: "Basic building material".to_string(),
                item_class: ItemClass::Resource,
                weight: 2.0,
                value: 1,
                stackable: true,
                max_stack: 100,
                sprite_index: 94 * 64 + 30,
            }),
            "stone" => Some(ItemDefinition {
                id: "stone".to_string(),
                name: "Stone".to_string(),
                description: "Heavy building material".to_string(),
                item_class: ItemClass::Resource,
                weight: 5.0,
                value: 2,
                stackable: true,
                max_stack: 50,
                sprite_index: 51 * 64 + 8,
            }),
            "iron_ore" => Some(ItemDefinition {
                id: "iron_ore".to_string(),
                name: "Iron Ore".to_string(),
                description: "Raw iron ore for smelting".to_string(),
                item_class: ItemClass::Resource,
                weight: 3.0,
                value: 5,
                stackable: true,
                max_stack: 50,
                sprite_index: 51 * 64 + 8,
            }),
            "iron_bar" => Some(ItemDefinition {
                id: "iron_bar".to_string(),
                name: "Iron Bar".to_string(),
                description: "Refined iron for crafting".to_string(),
                item_class: ItemClass::Resource,
                weight: 2.5,
                value: 10,
                stackable: true,
                max_stack: 50,
                sprite_index: 51 * 64 + 8,
            }),
            "cloth" => Some(ItemDefinition {
                id: "cloth".to_string(),
                name: "Cloth".to_string(),
                description: "Fabric for clothing".to_string(),
                item_class: ItemClass::Resource,
                weight: 0.5,
                value: 3,
                stackable: true,
                max_stack: 100,
                sprite_index: 51 * 64 + 8,
            }),
            "leather" => Some(ItemDefinition {
                id: "leather".to_string(),
                name: "Leather".to_string(),
                description: "Tanned animal hide".to_string(),
                item_class: ItemClass::Resource,
                weight: 1.0,
                value: 8,
                stackable: true,
                max_stack: 50,
                sprite_index: 51 * 64 + 8,
            }),

            // WEAPONS - Melee
            "wooden_club" => Some(ItemDefinition {
                id: "wooden_club".to_string(),
                name: "Wooden Club".to_string(),
                description: "Basic melee weapon".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 5,
                    attack_speed: 1.0,
                    range: 1.0,
                    weapon_type: WeaponType::Hammer,
                    durability: 50.0,
                    required_skill: 0,
                    ammo_type: None,
                }),
                weight: 3.0,
                value: 10,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),
            "combat_blade" => Some(ItemDefinition {
                id: "combat_blade".to_string(),
                name: "Combat Blade".to_string(),
                description: "Sturdy tactical blade".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 15,
                    attack_speed: 1.2,
                    range: 1.5,
                    weapon_type: WeaponType::Blade,
                    durability: 100.0,
                    required_skill: 3,
                    ammo_type: None,
                }),
                weight: 4.0,
                value: 100,
                stackable: false,
                max_stack: 1,
                sprite_index: 19 * 64 + 45,
            }),
            "power_axe" => Some(ItemDefinition {
                id: "power_axe".to_string(),
                name: "Power Axe".to_string(),
                description: "Powered combat axe".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 20,
                    attack_speed: 0.8,
                    range: 1.5,
                    weapon_type: WeaponType::PowerAxe,
                    durability: 120.0,
                    required_skill: 5,
                    ammo_type: None,
                }),
                weight: 6.0,
                value: 150,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),
            "plasma_lance" => Some(ItemDefinition {
                id: "plasma_lance".to_string(),
                name: "Plasma Lance".to_string(),
                description: "Long reach energy weapon".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 12,
                    attack_speed: 1.0,
                    range: 2.5,
                    weapon_type: WeaponType::PlasmaLance,
                    durability: 80.0,
                    required_skill: 2,
                    ammo_type: None,
                }),
                weight: 3.5,
                value: 80,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),

            // RANGED WEAPONS
            "railgun" => Some(ItemDefinition {
                id: "railgun".to_string(),
                name: "Railgun".to_string(),
                description: "Basic ballistic ranged weapon".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 10,
                    attack_speed: 0.7,
                    range: 8.0,
                    weapon_type: WeaponType::Railgun,
                    durability: 100.0,
                    required_skill: 2,
                    ammo_type: Some("railgun_slug".to_string()),
                }),
                weight: 2.0,
                value: 120,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),
            "gauss_rifle" => Some(ItemDefinition {
                id: "gauss_rifle".to_string(),
                name: "Gauss Rifle".to_string(),
                description: "Powerful gauss rifle".to_string(),
                item_class: ItemClass::Weapon(WeaponStats {
                    damage: 18,
                    attack_speed: 0.5,
                    range: 10.0,
                    weapon_type: WeaponType::GaussRifle,
                    durability: 150.0,
                    required_skill: 4,
                    ammo_type: Some("gauss_charge".to_string()),
                }),
                weight: 4.5,
                value: 200,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),

            // AMMO
            "railgun_slug" => Some(ItemDefinition {
                id: "railgun_slug".to_string(),
                name: "Railgun Slug".to_string(),
                description: "Ammunition for railguns".to_string(),
                item_class: ItemClass::Ammo(AmmoStats {
                    ammo_type: "railgun_slug".to_string(),
                    damage_modifier: 1.0,
                }),
                weight: 0.1,
                value: 1,
                stackable: true,
                max_stack: 100,
                sprite_index: 94 * 64 + 30,
            }),
            "gauss_charge" => Some(ItemDefinition {
                id: "gauss_charge".to_string(),
                name: "Gauss Charge".to_string(),
                description: "Charge for gauss rifles".to_string(),
                item_class: ItemClass::Ammo(AmmoStats {
                    ammo_type: "gauss_charge".to_string(),
                    damage_modifier: 1.2,
                }),
                weight: 0.2,
                value: 2,
                stackable: true,
                max_stack: 100,
                sprite_index: 94 * 64 + 30,
            }),

            // TOOLS
            "iron_axe" => Some(ItemDefinition {
                id: "iron_axe".to_string(),
                name: "Iron Axe".to_string(),
                description: "Efficient tree chopping tool".to_string(),
                item_class: ItemClass::Tool(ToolStats {
                    tool_type: ToolType::Axe,
                    efficiency: 1.5,
                    durability: 200.0,
                }),
                weight: 3.0,
                value: 50,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),
            "iron_pickaxe" => Some(ItemDefinition {
                id: "iron_pickaxe".to_string(),
                name: "Iron Pickaxe".to_string(),
                description: "For mining stone and ore".to_string(),
                item_class: ItemClass::Tool(ToolStats {
                    tool_type: ToolType::Pickaxe,
                    efficiency: 1.5,
                    durability: 200.0,
                }),
                weight: 4.0,
                value: 60,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),
            "fishing_rod" => Some(ItemDefinition {
                id: "fishing_rod".to_string(),
                name: "Fishing Rod".to_string(),
                description: "For catching fish".to_string(),
                item_class: ItemClass::Tool(ToolStats {
                    tool_type: ToolType::FishingRod,
                    efficiency: 1.0,
                    durability: 100.0,
                }),
                weight: 1.5,
                value: 40,
                stackable: false,
                max_stack: 1,
                sprite_index: 94 * 64 + 30,
            }),

            // ARMOR
            "leather_helmet" => Some(ItemDefinition {
                id: "leather_helmet".to_string(),
                name: "Leather Helmet".to_string(),
                description: "Basic head protection".to_string(),
                item_class: ItemClass::Armor(ArmorStats {
                    defense: 3,
                    armor_type: ArmorType::Head,
                    warmth: 1.0,
                    durability: 100.0,
                }),
                weight: 1.5,
                value: 30,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "iron_helmet" => Some(ItemDefinition {
                id: "iron_helmet".to_string(),
                name: "Iron Helmet".to_string(),
                description: "Strong head protection".to_string(),
                item_class: ItemClass::Armor(ArmorStats {
                    defense: 8,
                    armor_type: ArmorType::Head,
                    warmth: 0.5,
                    durability: 200.0,
                }),
                weight: 3.0,
                value: 100,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "leather_armor" => Some(ItemDefinition {
                id: "leather_armor".to_string(),
                name: "Leather Armor".to_string(),
                description: "Light body protection".to_string(),
                item_class: ItemClass::Armor(ArmorStats {
                    defense: 5,
                    armor_type: ArmorType::Chest,
                    warmth: 2.0,
                    durability: 150.0,
                }),
                weight: 4.0,
                value: 80,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "iron_armor" => Some(ItemDefinition {
                id: "iron_armor".to_string(),
                name: "Iron Armor".to_string(),
                description: "Heavy body protection".to_string(),
                item_class: ItemClass::Armor(ArmorStats {
                    defense: 15,
                    armor_type: ArmorType::Chest,
                    warmth: 1.0,
                    durability: 250.0,
                }),
                weight: 10.0,
                value: 250,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),

            // CLOTHING - The Long Dark style
            "wool_hat" => Some(ItemDefinition {
                id: "wool_hat".to_string(),
                name: "Wool Hat".to_string(),
                description: "Keeps your head warm".to_string(),
                item_class: ItemClass::Clothing(ClothingStats {
                    warmth: 3.0,
                    waterproof: 0.2,
                    windproof: 0.5,
                    slot: ClothingSlot::Head,
                }),
                weight: 0.3,
                value: 20,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "winter_coat" => Some(ItemDefinition {
                id: "winter_coat".to_string(),
                name: "Winter Coat".to_string(),
                description: "Heavy winter protection".to_string(),
                item_class: ItemClass::Clothing(ClothingStats {
                    warmth: 10.0,
                    waterproof: 0.6,
                    windproof: 0.8,
                    slot: ClothingSlot::Chest,
                }),
                weight: 2.0,
                value: 150,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "leather_gloves" => Some(ItemDefinition {
                id: "leather_gloves".to_string(),
                name: "Leather Gloves".to_string(),
                description: "Protects hands from cold".to_string(),
                item_class: ItemClass::Clothing(ClothingStats {
                    warmth: 2.0,
                    waterproof: 0.4,
                    windproof: 0.6,
                    slot: ClothingSlot::Hands,
                }),
                weight: 0.4,
                value: 30,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),
            "boots" => Some(ItemDefinition {
                id: "boots".to_string(),
                name: "Leather Boots".to_string(),
                description: "Sturdy footwear".to_string(),
                item_class: ItemClass::Clothing(ClothingStats {
                    warmth: 3.0,
                    waterproof: 0.7,
                    windproof: 0.5,
                    slot: ClothingSlot::Feet,
                }),
                weight: 1.2,
                value: 50,
                stackable: false,
                max_stack: 1,
                sprite_index: 51 * 64 + 8,
            }),

            // FOOD
            "cooked_meat" => Some(ItemDefinition {
                id: "cooked_meat".to_string(),
                name: "Cooked Meat".to_string(),
                description: "Nutritious cooked meat".to_string(),
                item_class: ItemClass::Food(FoodStats {
                    nutrition: 30.0,
                    calories: 400,
                    hydration: 0.0,
                    spoilage_rate: 0.05,
                    cooking_required: false,
                }),
                weight: 0.5,
                value: 15,
                stackable: true,
                max_stack: 20,
                sprite_index: 94 * 64 + 32,
            }),
            "raw_meat" => Some(ItemDefinition {
                id: "raw_meat".to_string(),
                name: "Raw Meat".to_string(),
                description: "Needs cooking".to_string(),
                item_class: ItemClass::Food(FoodStats {
                    nutrition: 15.0,
                    calories: 200,
                    hydration: 0.0,
                    spoilage_rate: 0.2,
                    cooking_required: true,
                }),
                weight: 0.5,
                value: 8,
                stackable: true,
                max_stack: 20,
                sprite_index: 94 * 64 + 32,
            }),
            "fish" => Some(ItemDefinition {
                id: "fish".to_string(),
                name: "Fish".to_string(),
                description: "Fresh caught fish".to_string(),
                item_class: ItemClass::Food(FoodStats {
                    nutrition: 20.0,
                    calories: 250,
                    hydration: 5.0,
                    spoilage_rate: 0.3,
                    cooking_required: true,
                }),
                weight: 0.4,
                value: 10,
                stackable: true,
                max_stack: 20,
                sprite_index: 94 * 64 + 32,
            }),
            "bread" => Some(ItemDefinition {
                id: "bread".to_string(),
                name: "Bread".to_string(),
                description: "Basic food staple".to_string(),
                item_class: ItemClass::Food(FoodStats {
                    nutrition: 15.0,
                    calories: 300,
                    hydration: 0.0,
                    spoilage_rate: 0.08,
                    cooking_required: false,
                }),
                weight: 0.3,
                value: 5,
                stackable: true,
                max_stack: 30,
                sprite_index: 94 * 64 + 32,
            }),

            // CONSUMABLES
            "bandage" => Some(ItemDefinition {
                id: "bandage".to_string(),
                name: "Bandage".to_string(),
                description: "Stops bleeding".to_string(),
                item_class: ItemClass::Consumable(ConsumableStats {
                    effect: ConsumableEffect::Healing(20),
                    duration: 0.0,
                }),
                weight: 0.1,
                value: 10,
                stackable: true,
                max_stack: 50,
                sprite_index: 51 * 64 + 8,
            }),
            "medkit" => Some(ItemDefinition {
                id: "medkit".to_string(),
                name: "Medkit".to_string(),
                description: "Nanite injector that restores health".to_string(),
                item_class: ItemClass::Consumable(ConsumableStats {
                    effect: ConsumableEffect::Healing(50),
                    duration: 0.0,
                }),
                weight: 0.5,
                value: 50,
                stackable: true,
                max_stack: 10,
                sprite_index: 51 * 64 + 8,
            }),
            _ => None,
        }
    }

    pub fn all_item_ids() -> Vec<&'static str> {
        vec![
            "wood", "stone", "iron_ore", "iron_bar", "cloth", "leather",
            "wooden_club", "combat_blade", "power_axe", "plasma_lance", "railgun", "gauss_rifle",
            "railgun_slug", "gauss_charge",
            "iron_axe", "iron_pickaxe", "fishing_rod",
            "leather_helmet", "iron_helmet", "leather_armor", "iron_armor",
            "wool_hat", "winter_coat", "leather_gloves", "boots",
            "cooked_meat", "raw_meat", "fish", "bread",
            "bandage", "medkit",
        ]
    }
}

// ============================================================================
// INVENTORY PLUGIN
// ============================================================================

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            inventory_degradation_system,
            equipment_bonus_system,
        ).run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs(5))));
    }
}

// System to degrade equipment over time (The Long Dark style)
fn inventory_degradation_system(
    mut query: Query<(&mut Inventory, &mut Equipment)>,
) {
    for (mut inventory, mut equipment) in query.iter_mut() {
        // Degrade equipped items
        let mut degrade_item = |item: &mut Option<InventoryItem>| {
            if let Some(inv_item) = item {
                inv_item.condition -= 0.001; // Very slow degradation
                if inv_item.condition <= 0.0 {
                    *item = None; // Item breaks
                }
            }
        };

        degrade_item(&mut equipment.weapon_main);
        degrade_item(&mut equipment.armor_chest);
        degrade_item(&mut equipment.armor_head);
        degrade_item(&mut equipment.armor_legs);
        degrade_item(&mut equipment.armor_feet);
        degrade_item(&mut equipment.armor_hands);
        degrade_item(&mut equipment.clothing_chest);
        degrade_item(&mut equipment.clothing_head);

        // Degrade food items in inventory
        for slot in inventory.slots.iter_mut() {
            if let Some(item) = slot {
                if let ItemClass::Food(food_stats) = &item.item_def.item_class {
                    if let ItemData::Food { spoilage } = &mut item.data {
                        *spoilage -= food_stats.spoilage_rate * 0.01;
                        if *spoilage <= 0.0 {
                            *slot = None; // Food spoiled
                        }
                    }
                }
            }
        }
    }
}

// System to apply equipment bonuses to attributes
fn equipment_bonus_system(
    mut query: Query<(&Equipment, &mut PhysicalBody)>,
) {
    for (equipment, mut body) in query.iter_mut() {
        // Equipment bonuses are already calculated via Equipment::total_defense() etc
        // This could add temporary stat bonuses from equipment
    }
}
