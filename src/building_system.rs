use bevy::prelude::*;
use crate::prelude::*;

// ============================================================================
// BUILDING SYSTEM - AoE2 style with upgrades
// ============================================================================

#[derive(Component, Clone)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: i32,
    pub max_level: i32,
    pub health: i32,
    pub max_health: i32,
    pub production_queue: Vec<ProductionItem>,
    pub garrison: Vec<Entity>,
    pub max_garrison: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub enum BuildingType {
    // Economy
    TownCenter,
    House,
    Farm,
    Mill,
    LumberCamp,
    MiningCamp,
    Market,
    Dock,

    // Military
    Barracks,
    FiringRange,
    VehicleBay,
    HeavyWeaponsFacility,
    CommandCenter,
    DefenseTurret,
    Gate,
    Wall,

    // Technology
    Blacksmith,
    University,
    ResearchStation,

    // Special
    Wonder,
    Workshop,
    ResearchLab,
    Furnace,
    CookingStation,
    TanningRack,
    Loom,
    StorageWarehouse,
}

impl BuildingType {
    pub fn base_health(&self) -> i32 {
        match self {
            BuildingType::TownCenter => 2400,
            BuildingType::House => 550,
            BuildingType::Farm => 480,
            BuildingType::Mill => 600,
            BuildingType::LumberCamp => 600,
            BuildingType::MiningCamp => 600,
            BuildingType::Market => 1800,
            BuildingType::Dock => 1800,
            BuildingType::Barracks => 1200,
            BuildingType::FiringRange => 1200,
            BuildingType::VehicleBay => 1200,
            BuildingType::HeavyWeaponsFacility => 1200,
            BuildingType::CommandCenter => 4800,
            BuildingType::DefenseTurret => 1500,
            BuildingType::Gate => 2750,
            BuildingType::Wall => 900,
            BuildingType::Blacksmith => 1200,
            BuildingType::University => 1200,
            BuildingType::ResearchStation => 1200,
            BuildingType::Wonder => 10000,
            BuildingType::Workshop => 800,
            BuildingType::ResearchLab => 1000,
            BuildingType::Furnace => 1200,
            BuildingType::CookingStation => 400,
            BuildingType::TanningRack => 600,
            BuildingType::Loom => 600,
            BuildingType::StorageWarehouse => 1500,
        }
    }

    pub fn build_time(&self) -> f32 {
        match self {
            BuildingType::TownCenter => 150.0,
            BuildingType::House => 25.0,
            BuildingType::Farm => 15.0,
            BuildingType::CommandCenter => 200.0,
            BuildingType::Wonder => 500.0,
            _ => 50.0,
        }
    }

    pub fn can_produce_units(&self) -> bool {
        matches!(
            self,
            BuildingType::TownCenter
                | BuildingType::Barracks
                | BuildingType::FiringRange
                | BuildingType::VehicleBay
                | BuildingType::HeavyWeaponsFacility
                | BuildingType::Dock
        )
    }

    pub fn can_research(&self) -> bool {
        matches!(
            self,
            BuildingType::TownCenter
                | BuildingType::Blacksmith
                | BuildingType::University
                | BuildingType::ResearchStation
                | BuildingType::ResearchLab
        )
    }

    pub fn garrison_capacity(&self) -> i32 {
        match self {
            BuildingType::TownCenter => 15,
            BuildingType::CommandCenter => 20,
            BuildingType::DefenseTurret => 5,
            _ => 0,
        }
    }

    pub fn provides_population(&self) -> i32 {
        match self {
            BuildingType::TownCenter => 5,
            BuildingType::House => 5,
            BuildingType::CommandCenter => 20,
            _ => 0,
        }
    }

    pub fn upgrade_cost(&self, current_level: i32) -> Vec<(String, u32)> {
        let multiplier = (current_level + 1) as u32;
        match self {
            BuildingType::TownCenter => vec![
                ("wood".to_string(), 200 * multiplier),
                ("stone".to_string(), 200 * multiplier),
                ("iron_bar".to_string(), 100 * multiplier),
            ],
            BuildingType::CommandCenter => vec![
                ("stone".to_string(), 300 * multiplier),
                ("iron_bar".to_string(), 150 * multiplier),
            ],
            _ => vec![
                ("wood".to_string(), 100 * multiplier),
                ("stone".to_string(), 50 * multiplier),
            ],
        }
    }
}

#[derive(Clone)]
pub struct ProductionItem {
    pub item_type: ProductionType,
    pub time_remaining: f32,
    pub progress: f32,
}

#[derive(Clone, PartialEq)]
pub enum ProductionType {
    Unit(String),
    Research(String),
    Upgrade,
}

// ============================================================================
// RESOURCE STORAGE SYSTEM
// ============================================================================

#[derive(Component)]
pub struct ResourceStorage {
    pub resources: HashMap<String, i32>,
    pub capacity: HashMap<String, i32>,
}

impl Default for ResourceStorage {
    fn default() -> Self {
        let mut resources = HashMap::new();
        let mut capacity = HashMap::new();

        // Initialize resources
        resources.insert("wood".to_string(), 200);
        resources.insert("stone".to_string(), 100);
        resources.insert("iron_ore".to_string(), 0);
        resources.insert("iron_bar".to_string(), 0);
        resources.insert("food".to_string(), 100);
        resources.insert("gold".to_string(), 0);

        // Initialize capacity
        capacity.insert("wood".to_string(), 1000);
        capacity.insert("stone".to_string(), 1000);
        capacity.insert("iron_ore".to_string(), 500);
        capacity.insert("iron_bar".to_string(), 500);
        capacity.insert("food".to_string(), 500);
        capacity.insert("gold".to_string(), 1000);

        ResourceStorage {
            resources,
            capacity,
        }
    }
}

impl ResourceStorage {
    pub fn add_resource(&mut self, resource: &str, amount: i32) -> i32 {
        let current = *self.resources.get(resource).unwrap_or(&0);
        let cap = *self.capacity.get(resource).unwrap_or(&1000);
        let can_add = (cap - current).min(amount);

        self.resources.insert(resource.to_string(), current + can_add);
        can_add
    }

    pub fn remove_resource(&mut self, resource: &str, amount: i32) -> bool {
        let current = *self.resources.get(resource).unwrap_or(&0);
        if current >= amount {
            self.resources.insert(resource.to_string(), current - amount);
            true
        } else {
            false
        }
    }

    pub fn has_resources(&self, requirements: &Vec<(String, u32)>) -> bool {
        for (resource, amount) in requirements {
            let current = *self.resources.get(resource).unwrap_or(&0);
            if current < *amount as i32 {
                return false;
            }
        }
        true
    }

    pub fn consume_resources(&mut self, requirements: &Vec<(String, u32)>) -> bool {
        if !self.has_resources(requirements) {
            return false;
        }

        for (resource, amount) in requirements {
            self.remove_resource(resource, *amount as i32);
        }
        true
    }
}

// ============================================================================
// POPULATION SYSTEM
// ============================================================================

#[derive(Resource)]
pub struct Population {
    pub current: i32,
    pub capacity: i32,
}

impl Default for Population {
    fn default() -> Self {
        Population {
            current: 0,
            capacity: 10,
        }
    }
}

// ============================================================================
// BUILDING PLUGIN
// ============================================================================

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Population::default())
            .add_systems(
                Update,
                (
                    building_production_system,
                    building_upgrade_system,
                    building_garrison_system,
                    calculate_population_capacity,
                )
                    .run_if(bevy::time::common_conditions::on_timer(
                        bevy::utils::Duration::from_secs(1),
                    )),
            );
    }
}

fn building_production_system(
    mut building_query: Query<&mut Building>,
    mut commands: Commands,
) {
    for mut building in building_query.iter_mut() {
        if building.production_queue.is_empty() {
            continue;
        }

        let item = &mut building.production_queue[0];
        item.time_remaining -= 1.0;
        item.progress = 1.0 - (item.time_remaining / 100.0);

        if item.time_remaining <= 0.0 {
            // Production complete
            match &item.item_type {
                ProductionType::Unit(unit_type) => {
                    println!("Unit produced: {}", unit_type);
                    // TODO: Spawn unit near building
                }
                ProductionType::Research(tech) => {
                    println!("Research complete: {}", tech);
                    // TODO: Unlock tech
                }
                ProductionType::Upgrade => {
                    building.level += 1;
                    building.max_health = (building.max_health as f32 * 1.5) as i32;
                    building.health = building.max_health;
                    println!("Building upgraded to level {}", building.level);
                }
            }

            building.production_queue.remove(0);
        }
    }
}

fn building_upgrade_system(
    mut building_query: Query<&mut Building>,
    mut resource_storage: Query<&mut ResourceStorage>,
) {
    for mut building in building_query.iter_mut() {
        // Auto-upgrade logic could go here
        // For now, upgrades are triggered manually via production queue
    }
}

fn building_garrison_system(
    building_query: Query<&Building>,
) {
    for building in building_query.iter() {
        // Handle garrisoned units healing, protection, etc.
        if !building.garrison.is_empty() {
            // Units inside are protected from attacks
            // Units slowly heal while garrisoned
        }
    }
}

fn calculate_population_capacity(
    building_query: Query<&Building>,
    mut population: ResMut<Population>,
) {
    let mut capacity = 0;

    for building in building_query.iter() {
        capacity += building.building_type.provides_population();
    }

    population.capacity = capacity.max(10); // Minimum 10 capacity
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn spawn_building(
    commands: &mut Commands,
    building_type: BuildingType,
    position: Position,
    texture: &SpriteSheetHandle,
    atlas_layout: &SpriteSheetAtlasLayout,
) -> Entity {
    let max_health = building_type.base_health();
    let max_garrison = building_type.garrison_capacity();

    commands.spawn((
        SpriteSheetBundle {
            texture: texture.0.clone(),
            atlas: TextureAtlas {
                layout: atlas_layout.0.clone(),
                index: 3 * 64 + 10, // Placeholder sprite
            },
            transform: position.to_transform_layer(1.0),
            ..default()
        },
        Building {
            building_type: building_type.clone(),
            level: 1,
            max_level: 5,
            health: max_health,
            max_health,
            production_queue: vec![],
            garrison: vec![],
            max_garrison,
        },
        position,
    )).id()
}
