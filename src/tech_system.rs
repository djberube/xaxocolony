use bevy::prelude::*;
use std::collections::HashMap;
use crate::prelude::*;

// ============================================================================
// TECHNOLOGY RESEARCH SYSTEM - AoE2 / Master of Orion 2 style
// ============================================================================

#[derive(Resource)]
pub struct TechnologyTree {
    pub unlocked_techs: Vec<String>,
    pub researching: Option<ResearchProgress>,
    pub available_techs: Vec<Technology>,
}

impl Default for TechnologyTree {
    fn default() -> Self {
        TechnologyTree {
            unlocked_techs: vec![],
            researching: None,
            available_techs: Self::initialize_tech_tree(),
        }
    }
}

impl TechnologyTree {
    pub fn initialize_tech_tree() -> Vec<Technology> {
        vec![
            // AGE 1: STONE AGE
            Technology {
                id: "stone_tools".to_string(),
                name: "Stone Tools".to_string(),
                description: "Basic stone implements for survival".to_string(),
                age: TechAge::StoneAge,
                category: TechCategory::Tools,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 50,
                    time: 30.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Recipe("craft_stone_axe".to_string()),
                    Unlock::Recipe("craft_stone_pickaxe".to_string()),
                ],
            },
            Technology {
                id: "fire".to_string(),
                name: "Fire Making".to_string(),
                description: "The ability to create and control fire".to_string(),
                age: TechAge::StoneAge,
                category: TechCategory::Survival,
                research_cost: ResearchCost {
                    resources: vec![("wood".to_string(), 10)],
                    research_points: 100,
                    time: 60.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("campfire".to_string()),
                    Unlock::Building("torch".to_string()),
                    Unlock::Recipe("cook_meat".to_string()),
                ],
            },
            Technology {
                id: "basic_farming".to_string(),
                name: "Basic Farming".to_string(),
                description: "Cultivate crops for sustainable food".to_string(),
                age: TechAge::StoneAge,
                category: TechCategory::Economy,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 150,
                    time: 90.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("farm".to_string()),
                    Unlock::Recipe("plant_wheat".to_string()),
                    Unlock::Recipe("plant_vegetables".to_string()),
                ],
            },
            Technology {
                id: "hunting".to_string(),
                name: "Hunting".to_string(),
                description: "Track and hunt wild animals".to_string(),
                age: TechAge::StoneAge,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 100,
                    time: 60.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Recipe("craft_spear".to_string()),
                    Unlock::Ability("tracking".to_string()),
                ],
            },

            // AGE 2: BRONZE AGE
            Technology {
                id: "mining".to_string(),
                name: "Mining".to_string(),
                description: "Extract valuable ores from the earth".to_string(),
                age: TechAge::BronzeAge,
                category: TechCategory::Economy,
                research_cost: ResearchCost {
                    resources: vec![("wood".to_string(), 20)],
                    research_points: 200,
                    time: 120.0,
                },
                prerequisites: vec!["stone_tools".to_string()],
                unlocks: vec![
                    Unlock::Building("mine".to_string()),
                    Unlock::Resource("copper_ore".to_string()),
                    Unlock::Resource("tin_ore".to_string()),
                ],
            },
            Technology {
                id: "smelting".to_string(),
                name: "Smelting".to_string(),
                description: "Refine ores into metal bars".to_string(),
                age: TechAge::BronzeAge,
                category: TechCategory::Economy,
                research_cost: ResearchCost {
                    resources: vec![("wood".to_string(), 30), ("stone".to_string(), 20)],
                    research_points: 250,
                    time: 150.0,
                },
                prerequisites: vec!["mining".to_string(), "fire".to_string()],
                unlocks: vec![
                    Unlock::Building("furnace".to_string()),
                    Unlock::Recipe("smelt_copper_bar".to_string()),
                    Unlock::Recipe("smelt_bronze_bar".to_string()),
                ],
            },
            Technology {
                id: "bronze_working".to_string(),
                name: "Bronze Working".to_string(),
                description: "Craft with bronze, a strong alloy".to_string(),
                age: TechAge::BronzeAge,
                category: TechCategory::Tools,
                research_cost: ResearchCost {
                    resources: vec![("bronze_bar".to_string(), 10)],
                    research_points: 300,
                    time: 180.0,
                },
                prerequisites: vec!["smelting".to_string()],
                unlocks: vec![
                    Unlock::Recipe("craft_bronze_sword".to_string()),
                    Unlock::Recipe("craft_bronze_armor".to_string()),
                    Unlock::Recipe("craft_bronze_tools".to_string()),
                ],
            },
            Technology {
                id: "marksmanship".to_string(),
                name: "Marksmanship".to_string(),
                description: "Master ranged ballistic weapons".to_string(),
                age: TechAge::BronzeAge,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![("wood".to_string(), 25)],
                    research_points: 200,
                    time: 120.0,
                },
                prerequisites: vec!["hunting".to_string()],
                unlocks: vec![
                    Unlock::Recipe("craft_railgun".to_string()),
                    Unlock::Recipe("craft_railgun_slugs".to_string()),
                    Unlock::Unit("rifleman".to_string()),
                ],
            },
            Technology {
                id: "masonry".to_string(),
                name: "Masonry".to_string(),
                description: "Build with stone structures".to_string(),
                age: TechAge::BronzeAge,
                category: TechCategory::Construction,
                research_cost: ResearchCost {
                    resources: vec![("stone".to_string(), 50)],
                    research_points: 250,
                    time: 150.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("stone_wall".to_string()),
                    Unlock::Building("stone_house".to_string()),
                    Unlock::Building("watchtower".to_string()),
                ],
            },

            // AGE 3: IRON AGE
            Technology {
                id: "ironworking".to_string(),
                name: "Iron Working".to_string(),
                description: "Smelt and forge iron, stronger than bronze".to_string(),
                age: TechAge::IronAge,
                category: TechCategory::Tools,
                research_cost: ResearchCost {
                    resources: vec![("iron_ore".to_string(), 30)],
                    research_points: 500,
                    time: 300.0,
                },
                prerequisites: vec!["smelting".to_string()],
                unlocks: vec![
                    Unlock::Recipe("smelt_iron_bar".to_string()),
                    Unlock::Recipe("craft_iron_sword".to_string()),
                    Unlock::Recipe("craft_iron_armor".to_string()),
                    Unlock::Recipe("craft_iron_axe".to_string()),
                    Unlock::Recipe("craft_iron_pickaxe".to_string()),
                ],
            },
            Technology {
                id: "advanced_weaponry".to_string(),
                name: "Advanced Weaponry".to_string(),
                description: "Craft superior weapons of war".to_string(),
                age: TechAge::IronAge,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![("iron_bar".to_string(), 50)],
                    research_points: 600,
                    time: 360.0,
                },
                prerequisites: vec!["ironworking".to_string()],
                unlocks: vec![
                    Unlock::Recipe("craft_war_axe".to_string()),
                    Unlock::Recipe("craft_longsword".to_string()),
                    Unlock::Recipe("craft_battle_hammer".to_string()),
                    Unlock::Unit("swordsman".to_string()),
                ],
            },
            Technology {
                id: "precision_weapons".to_string(),
                name: "Precision Weapons".to_string(),
                description: "Develop gauss rifles and sniper systems".to_string(),
                age: TechAge::IronAge,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![("iron_bar".to_string(), 40), ("wood".to_string(), 40)],
                    research_points: 550,
                    time: 330.0,
                },
                prerequisites: vec!["marksmanship".to_string(), "ironworking".to_string()],
                unlocks: vec![
                    Unlock::Recipe("craft_gauss_rifle".to_string()),
                    Unlock::Recipe("craft_gauss_charges".to_string()),
                    Unlock::Unit("sniper".to_string()),
                ],
            },
            Technology {
                id: "defense_systems".to_string(),
                name: "Defense Systems".to_string(),
                description: "Build fortified command centers and defense structures".to_string(),
                age: TechAge::IronAge,
                category: TechCategory::Construction,
                research_cost: ResearchCost {
                    resources: vec![("stone".to_string(), 100), ("iron_bar".to_string(), 30)],
                    research_points: 700,
                    time: 420.0,
                },
                prerequisites: vec!["masonry".to_string()],
                unlocks: vec![
                    Unlock::Building("command_center".to_string()),
                    Unlock::Building("gate".to_string()),
                    Unlock::Building("defense_turret".to_string()),
                ],
            },
            Technology {
                id: "mechanized_units".to_string(),
                name: "Mechanized Units".to_string(),
                description: "Deploy combat vehicles and armored units".to_string(),
                age: TechAge::IronAge,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![("iron_bar".to_string(), 40)],
                    research_points: 600,
                    time: 360.0,
                },
                prerequisites: vec!["ironworking".to_string()],
                unlocks: vec![
                    Unlock::Building("vehicle_bay".to_string()),
                    Unlock::Unit("armored_vehicle".to_string()),
                    Unlock::Unit("assault_trooper".to_string()),
                ],
            },

            // AGE 4: ELECTRONIC
            Technology {
                id: "steel_working".to_string(),
                name: "Steel Working".to_string(),
                description: "Master steel production for superior equipment".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Tools,
                research_cost: ResearchCost {
                    resources: vec![("iron_bar".to_string(), 100)],
                    research_points: 1000,
                    time: 600.0,
                },
                prerequisites: vec!["ironworking".to_string()],
                unlocks: vec![
                    Unlock::Recipe("smelt_steel_bar".to_string()),
                    Unlock::Recipe("craft_steel_sword".to_string()),
                    Unlock::Recipe("craft_steel_armor".to_string()),
                ],
            },
            Technology {
                id: "heavy_weapons".to_string(),
                name: "Heavy Weapons".to_string(),
                description: "Deploy artillery and siege mechs to break defenses".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Military,
                research_cost: ResearchCost {
                    resources: vec![("wood".to_string(), 100), ("iron_bar".to_string(), 80)],
                    research_points: 900,
                    time: 540.0,
                },
                prerequisites: vec!["defense_systems".to_string()],
                unlocks: vec![
                    Unlock::Building("heavy_weapons_facility".to_string()),
                    Unlock::Unit("artillery".to_string()),
                    Unlock::Unit("siege_mech".to_string()),
                    Unlock::Unit("railgun_turret".to_string()),
                ],
            },
            Technology {
                id: "biochemistry".to_string(),
                name: "Biochemistry".to_string(),
                description: "Synthesize medical compounds and performance enhancers".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Science,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 800,
                    time: 480.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("research_lab".to_string()),
                    Unlock::Recipe("craft_medkit".to_string()),
                    Unlock::Recipe("craft_energy_cell".to_string()),
                    Unlock::Recipe("craft_stimpack".to_string()),
                ],
            },
            Technology {
                id: "trade".to_string(),
                name: "Trade Networks".to_string(),
                description: "Establish trade routes and commerce".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Economy,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 600,
                    time: 360.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("market".to_string()),
                    Unlock::Building("trade_post".to_string()),
                    Unlock::Ability("trade_routes".to_string()),
                ],
            },

            // SPECIAL TECHNOLOGIES
            Technology {
                id: "agriculture_advanced".to_string(),
                name: "Advanced Agriculture".to_string(),
                description: "Improve farming efficiency dramatically".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Economy,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 500,
                    time: 300.0,
                },
                prerequisites: vec!["basic_farming".to_string()],
                unlocks: vec![
                    Unlock::Bonus("farm_yield_+50%".to_string()),
                    Unlock::Recipe("crop_rotation".to_string()),
                ],
            },
            Technology {
                id: "medicine".to_string(),
                name: "Medicine".to_string(),
                description: "Advanced healing and surgery".to_string(),
                age: TechAge::Electronic,
                category: TechCategory::Science,
                research_cost: ResearchCost {
                    resources: vec![],
                    research_points: 700,
                    time: 420.0,
                },
                prerequisites: vec![],
                unlocks: vec![
                    Unlock::Building("hospital".to_string()),
                    Unlock::Recipe("craft_medicine".to_string()),
                    Unlock::Ability("surgery".to_string()),
                ],
            },
        ]
    }

    pub fn can_research(&self, tech_id: &str) -> bool {
        // Check if already unlocked
        if self.unlocked_techs.contains(&tech_id.to_string()) {
            return false;
        }

        // Check if already researching
        if let Some(research) = &self.researching {
            if research.tech_id == tech_id {
                return false;
            }
        }

        // Find the tech
        if let Some(tech) = self.available_techs.iter().find(|t| t.id == tech_id) {
            // Check prerequisites
            for prereq in &tech.prerequisites {
                if !self.unlocked_techs.contains(prereq) {
                    return false;
                }
            }
            return true;
        }

        false
    }

    pub fn start_research(&mut self, tech_id: String) {
        if let Some(tech) = self.available_techs.iter().find(|t| t.id == tech_id).cloned() {
            self.researching = Some(ResearchProgress {
                tech_id: tech.id.clone(),
                time_remaining: tech.research_cost.time,
                research_points_spent: 0,
                research_points_needed: tech.research_cost.research_points,
            });
        }
    }

    pub fn complete_research(&mut self, tech_id: String) -> Vec<Unlock> {
        self.unlocked_techs.push(tech_id.clone());
        self.researching = None;

        // Return unlocks
        if let Some(tech) = self.available_techs.iter().find(|t| t.id == tech_id) {
            return tech.unlocks.clone();
        }
        vec![]
    }
}

#[derive(Clone)]
pub struct Technology {
    pub id: String,
    pub name: String,
    pub description: String,
    pub age: TechAge,
    pub category: TechCategory,
    pub research_cost: ResearchCost,
    pub prerequisites: Vec<String>,
    pub unlocks: Vec<Unlock>,
}

#[derive(Clone, PartialEq)]
pub enum TechAge {
    StoneAge,
    BronzeAge,
    IronAge,
    Electronic,
    Renaissance,
    Industrial,
    Modern,
}

#[derive(Clone, PartialEq)]
pub enum TechCategory {
    Military,
    Economy,
    Tools,
    Construction,
    Science,
    Survival,
}

#[derive(Clone)]
pub struct ResearchCost {
    pub resources: Vec<(String, u32)>,
    pub research_points: i32,
    pub time: f32, // Seconds
}

#[derive(Clone)]
pub enum Unlock {
    Recipe(String),
    Building(String),
    Unit(String),
    Ability(String),
    Resource(String),
    Bonus(String),
}

pub struct ResearchProgress {
    pub tech_id: String,
    pub time_remaining: f32,
    pub research_points_spent: i32,
    pub research_points_needed: i32,
}

// ============================================================================
// CIVILIZATION SYSTEM (AoE2 Style)
// ============================================================================

#[derive(Component, Clone)]
pub struct Civilization {
    pub civ_type: CivilizationType,
    pub bonuses: Vec<CivBonus>,
    pub unique_units: Vec<String>,
    pub unique_techs: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum CivilizationType {
    Humans,
    Cyborgs,
    Miners,
    Mutants,
    Synthetics,
}

impl CivilizationType {
    pub fn bonuses(&self) -> Vec<CivBonus> {
        match self {
            CivilizationType::Humans => vec![
                CivBonus::GatherRate("all".to_string(), 1.1),
                CivBonus::BuildSpeed(1.15),
                CivBonus::StartingResources("food".to_string(), 100),
            ],
            CivilizationType::Cyborgs => vec![
                CivBonus::ArcheryDamage(1.25),
                CivBonus::MovementSpeed(1.2),
                CivBonus::VisionRange(1.5),
            ],
            CivilizationType::Miners => vec![
                CivBonus::MiningSpeed(1.5),
                CivBonus::BuildingHealth(1.3),
                CivBonus::MeleeDefense(1.2),
            ],
            CivilizationType::Mutants => vec![
                CivBonus::MeleeDamage(1.3),
                CivBonus::UnitHealth(1.25),
                CivBonus::TrainingSpeed(1.2),
            ],
            CivilizationType::Synthetics => vec![
                CivBonus::NoFoodCost,
                CivBonus::Regeneration(0.5),
                CivBonus::PoisonResistance(1.0),
            ],
        }
    }

    pub fn unique_units(&self) -> Vec<String> {
        match self {
            CivilizationType::Humans => vec!["assault_trooper".to_string(), "elite_guard".to_string()],
            CivilizationType::Cyborgs => vec!["scout".to_string(), "sentinel_bot".to_string()],
            CivilizationType::Miners => vec!["shock_trooper".to_string(), "heavy_weapons_specialist".to_string()],
            CivilizationType::Mutants => vec!["warboss".to_string(), "marauder".to_string()],
            CivilizationType::Synthetics => vec!["ai_controller".to_string(), "immortal_cyborg".to_string()],
        }
    }
}

#[derive(Clone)]
pub enum CivBonus {
    GatherRate(String, f32),      // Resource type, multiplier
    BuildSpeed(f32),                // Multiplier
    StartingResources(String, i32), // Resource type, amount
    ArcheryDamage(f32),             // Multiplier
    MeleeDamage(f32),               // Multiplier
    MeleeDefense(f32),              // Multiplier
    MovementSpeed(f32),             // Multiplier
    VisionRange(f32),               // Multiplier
    MiningSpeed(f32),               // Multiplier
    BuildingHealth(f32),            // Multiplier
    UnitHealth(f32),                // Multiplier
    TrainingSpeed(f32),             // Multiplier
    NoFoodCost,                     // Units don't consume food
    Regeneration(f32),              // HP per second
    PoisonResistance(f32),          // Damage reduction
}

// ============================================================================
// TECH PLUGIN
// ============================================================================

pub struct TechPlugin;

impl Plugin for TechPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TechnologyTree::default())
            .add_systems(
                Update,
                research_progress_system.run_if(bevy::time::common_conditions::on_timer(
                    bevy::utils::Duration::from_secs(1),
                )),
            );
    }
}

fn research_progress_system(mut tech_tree: ResMut<TechnologyTree>) {
    if let Some(research) = &mut tech_tree.researching {
        research.time_remaining -= 1.0;

        if research.time_remaining <= 0.0 {
            let tech_id = research.tech_id.clone();
            let unlocks = tech_tree.complete_research(tech_id);
            println!("Research complete! Unlocked: {:?}", unlocks);
        }
    }
}
