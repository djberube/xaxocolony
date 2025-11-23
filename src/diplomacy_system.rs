use bevy::prelude::*;
use crate::prelude::*;
use std::collections::HashMap;

// ============================================================================
// DIPLOMACY SYSTEM - MOO2/Civilization style
// ============================================================================

#[derive(Resource)]
pub struct DiplomacyState {
    pub factions: HashMap<u32, Faction>,
    pub relationships: HashMap<(u32, u32), Relationship>,
    pub treaties: Vec<Treaty>,
    pub trade_routes: Vec<TradeRoute>,
}

impl Default for DiplomacyState {
    fn default() -> Self {
        DiplomacyState {
            factions: HashMap::new(),
            relationships: HashMap::new(),
            treaties: vec![],
            trade_routes: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Faction {
    pub id: u32,
    pub name: String,
    pub civ_type: CivilizationType,
    pub leader: Option<Entity>,
    pub color: Color,
    pub ai_personality: AIPersonality,
    pub resources: HashMap<String, i32>,
    pub military_strength: i32,
    pub economic_strength: i32,
    pub territory_size: i32,
}

#[derive(Clone, PartialEq)]
pub enum AIPersonality {
    Aggressive,  // Likes war, dislikes peace
    Peaceful,    // Prefers diplomacy
    Economic,    // Focuses on trade
    Scientific,  // Focuses on technology
    Religious,   // Focuses on culture/religion
    Opportunist, // Adapts to situation
}

#[derive(Clone)]
pub struct Relationship {
    pub faction1: u32,
    pub faction2: u32,
    pub opinion: i32, // -100 to +100
    pub status: DiplomaticStatus,
    pub history: Vec<DiplomaticEvent>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DiplomaticStatus {
    War,
    Hostile,
    Neutral,
    Friendly,
    Allied,
    Vassal, // One faction is subordinate to another
}

#[derive(Clone)]
pub struct DiplomaticEvent {
    pub event_type: DiplomaticEventType,
    pub opinion_change: i32,
    pub turn: i32,
}

#[derive(Clone, PartialEq)]
pub enum DiplomaticEventType {
    DeclaredWar,
    MadePeace,
    BrokeTreaty,
    CompletedTrade,
    GaveGift,
    DemandRefused,
    BorderViolation,
    AttackedAlly,
    SharedEnemy,
    TechnologyTrade,
}

// ============================================================================
// TREATIES
// ============================================================================

#[derive(Clone)]
pub struct Treaty {
    pub treaty_id: u32,
    pub treaty_type: TreatyType,
    pub participants: Vec<u32>, // Faction IDs
    pub duration: Option<i32>,  // None = permanent
    pub terms: Vec<TreatyTerm>,
    pub active: bool,
}

#[derive(Clone, PartialEq)]
pub enum TreatyType {
    Peace,
    Alliance,
    TradeAgreement,
    NonAggression,
    MutualDefense,
    OpenBorders,
    ResearchAgreement,
    Vassalization,
}

#[derive(Clone)]
pub enum TreatyTerm {
    NoCombat,
    SharedVision,
    SharedResources(String, i32), // Resource type, amount per turn
    TechnologySharing,
    MilitaryAccess,
    TradeBonus(f32), // Multiplier
    Tribute(String, i32), // Resource type, amount per turn
}

// ============================================================================
// TRADE SYSTEM
// ============================================================================

#[derive(Clone)]
pub struct TradeRoute {
    pub route_id: u32,
    pub from_faction: u32,
    pub to_faction: u32,
    pub from_building: Entity,
    pub to_building: Entity,
    pub resource: String,
    pub amount_per_turn: i32,
    pub profit_per_turn: i32,
    pub caravans: Vec<Entity>,
    pub active: bool,
}

#[derive(Component)]
pub struct Trader {
    pub trade_route_id: u32,
    pub carrying: HashMap<String, i32>,
    pub destination: Position,
    pub return_destination: Position,
}

#[derive(Component)]
pub struct Market {
    pub faction_id: u32,
    pub trade_routes: Vec<u32>,
    pub buy_prices: HashMap<String, i32>,
    pub sell_prices: HashMap<String, i32>,
    pub stock: HashMap<String, i32>,
}

// ============================================================================
// DIPLOMACY ACTIONS
// ============================================================================

pub enum DiplomaticAction {
    DeclareWar(u32),
    OfferPeace(u32),
    ProposeAlliance(u32),
    BreakAlliance(u32),
    SendGift(u32, String, i32), // Faction, resource, amount
    MakeDemand(u32, Demand),
    ProposeTreaty(u32, TreatyType, Vec<TreatyTerm>),
    BreakTreaty(u32),
}

#[derive(Clone)]
pub enum Demand {
    PayTribute(String, i32),
    CedeTerritoryBuilding(Entity),
    BreakAllianceWith(u32),
    ShareTechnology(String),
}

// ============================================================================
// DIPLOMACY PLUGIN
// ============================================================================

pub struct DiplomacyPlugin;

impl Plugin for DiplomacyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DiplomacyState::default())
            .add_systems(
                Update,
                (
                    update_relationships_system,
                    process_trade_routes,
                    ai_diplomacy_decisions,
                )
                    .run_if(bevy::time::common_conditions::on_timer(
                        bevy::utils::Duration::from_secs(5),
                    )),
            );
    }
}

fn update_relationships_system(mut diplomacy: ResMut<DiplomacyState>) {
    // Opinions decay toward neutral over time
    for relationship in diplomacy.relationships.values_mut() {
        if relationship.opinion > 0 {
            relationship.opinion = (relationship.opinion - 1).max(0);
        } else if relationship.opinion < 0 {
            relationship.opinion = (relationship.opinion + 1).min(0);
        }

        // Update diplomatic status based on opinion
        relationship.status = match relationship.opinion {
            -100..=-50 => DiplomaticStatus::War,
            -49..=-10 => DiplomaticStatus::Hostile,
            -9..=9 => DiplomaticStatus::Neutral,
            10..=49 => DiplomaticStatus::Friendly,
            50..=100 => DiplomaticStatus::Allied,
            _ => DiplomaticStatus::Neutral,
        };
    }
}

fn process_trade_routes(
    mut diplomacy: ResMut<DiplomacyState>,
    mut market_query: Query<&mut Market>,
) {
    for route in &mut diplomacy.trade_routes {
        if !route.active {
            continue;
        }

        // Transfer resources
        if let Some(from_faction) = diplomacy.factions.get_mut(&route.from_faction) {
            if let Some(amount) = from_faction.resources.get_mut(&route.resource) {
                if *amount >= route.amount_per_turn {
                    *amount -= route.amount_per_turn;

                    // Add profit to from faction
                    let gold = from_faction.resources.entry("gold".to_string()).or_insert(0);
                    *gold += route.profit_per_turn;

                    // Give resources to to faction
                    if let Some(to_faction) = diplomacy.factions.get_mut(&route.to_faction) {
                        let resource = to_faction
                            .resources
                            .entry(route.resource.clone())
                            .or_insert(0);
                        *resource += route.amount_per_turn;
                    }
                }
            }
        }
    }
}

fn ai_diplomacy_decisions(
    diplomacy: Res<DiplomacyState>,
) {
    // AI makes diplomatic decisions based on personality and situation
    for (faction_id, faction) in &diplomacy.factions {
        match faction.ai_personality {
            AIPersonality::Aggressive => {
                // Look for weak neighbors to attack
                // Evaluate military strength vs neighbors
            }
            AIPersonality::Peaceful => {
                // Offer peace treaties
                // Propose alliances with neighbors
            }
            AIPersonality::Economic => {
                // Establish trade routes
                // Propose trade agreements
            }
            AIPersonality::Scientific => {
                // Propose research agreements
                // Trade technologies
            }
            AIPersonality::Religious => {
                // Spread religion/culture
                // Form coalitions
            }
            AIPersonality::Opportunist => {
                // Adapt to current situation
                // Backstab when advantageous
            }
        }
    }
}

// ============================================================================
// DIPLOMACY HELPERS
// ============================================================================

impl DiplomacyState {
    pub fn get_relationship(&self, faction1: u32, faction2: u32) -> Option<&Relationship> {
        self.relationships
            .get(&(faction1, faction2))
            .or_else(|| self.relationships.get(&(faction2, faction1)))
    }

    pub fn get_relationship_mut(&mut self, faction1: u32, faction2: u32) -> Option<&mut Relationship> {
        if self.relationships.contains_key(&(faction1, faction2)) {
            self.relationships.get_mut(&(faction1, faction2))
        } else {
            self.relationships.get_mut(&(faction2, faction1))
        }
    }

    pub fn modify_opinion(&mut self, faction1: u32, faction2: u32, change: i32) {
        if let Some(relationship) = self.get_relationship_mut(faction1, faction2) {
            relationship.opinion = (relationship.opinion + change).clamp(-100, 100);
        }
    }

    pub fn declare_war(&mut self, faction1: u32, faction2: u32) {
        if let Some(relationship) = self.get_relationship_mut(faction1, faction2) {
            relationship.status = DiplomaticStatus::War;
            relationship.opinion = -100;
            relationship.history.push(DiplomaticEvent {
                event_type: DiplomaticEventType::DeclaredWar,
                opinion_change: -100,
                turn: 0, // Would use actual turn counter
            });
        }

        // Break all treaties
        self.treaties.retain(|treaty| {
            !treaty.participants.contains(&faction1) || !treaty.participants.contains(&faction2)
        });
    }

    pub fn make_peace(&mut self, faction1: u32, faction2: u32) {
        if let Some(relationship) = self.get_relationship_mut(faction1, faction2) {
            relationship.status = DiplomaticStatus::Neutral;
            relationship.opinion = 0;
            relationship.history.push(DiplomaticEvent {
                event_type: DiplomaticEventType::MadePeace,
                opinion_change: 50,
                turn: 0,
            });
        }
    }

    pub fn establish_trade_route(
        &mut self,
        from_faction: u32,
        to_faction: u32,
        from_building: Entity,
        to_building: Entity,
        resource: String,
        amount: i32,
    ) -> u32 {
        let route_id = self.trade_routes.len() as u32;

        self.trade_routes.push(TradeRoute {
            route_id,
            from_faction,
            to_faction,
            from_building,
            to_building,
            resource: resource.clone(),
            amount_per_turn: amount,
            profit_per_turn: amount * 2, // 2 gold per resource unit
            caravans: vec![],
            active: true,
        });

        // Improve relations
        self.modify_opinion(from_faction, to_faction, 10);

        route_id
    }
}
