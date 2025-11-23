use bevy::prelude::*;
use crate::prelude::*;

// ============================================================================
// FORMATION & SQUAD SYSTEM - AoE2 military tactics
// ============================================================================

#[derive(Component)]
pub struct Squad {
    pub squad_id: u32,
    pub members: Vec<Entity>,
    pub formation: FormationType,
    pub stance: CombatStance,
    pub rally_point: Option<Position>,
    pub leader: Option<Entity>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FormationType {
    Line,         // ---- Single line
    Box,          // █ Box formation
    Column,       // |  Column
    Wedge,        // ▼ V-formation
    Circle,       // ○ Defensive circle
    Scattered,    // • • • No formation
    Flank,        // >) Flanking maneuver
    Phalanx,      // ▬ Tight defensive line
    Skirmish,     // ·· Loose formation for ranged
}

#[derive(Clone, PartialEq, Debug)]
pub enum CombatStance {
    Aggressive,   // Attack everything
    Defensive,    // Only defend when attacked
    StandGround,  // Don't move, only fight nearby
    NoAttack,     // Don't attack, flee if attacked
    Patrol,       // Patrol between points
    Guard,        // Guard a specific unit/building
}

#[derive(Component)]
pub struct FormationPosition {
    pub squad_id: u32,
    pub relative_position: Vec2, // Position relative to formation center
    pub rank: i32,               // Front line = 0, back line = higher
}

// ============================================================================
// SQUAD COMMANDS
// ============================================================================

#[derive(Clone)]
pub enum SquadCommand {
    MoveTo(Position),
    AttackMove(Position),
    AttackTarget(Entity),
    Patrol(Vec<Position>),
    Hold,
    Follow(Entity),
    SetFormation(FormationType),
    SetStance(CombatStance),
    Garrison(Entity),
    Split,
    Merge(u32), // Merge with another squad
}

// ============================================================================
// FORMATION CALCULATOR
// ============================================================================

pub struct FormationCalculator;

impl FormationCalculator {
    pub fn calculate_positions(
        formation: &FormationType,
        num_units: usize,
        center: &Position,
    ) -> Vec<Position> {
        match formation {
            FormationType::Line => Self::line_formation(num_units, center),
            FormationType::Box => Self::box_formation(num_units, center),
            FormationType::Column => Self::column_formation(num_units, center),
            FormationType::Wedge => Self::wedge_formation(num_units, center),
            FormationType::Circle => Self::circle_formation(num_units, center),
            FormationType::Phalanx => Self::phalanx_formation(num_units, center),
            FormationType::Flank => Self::flank_formation(num_units, center),
            FormationType::Skirmish => Self::skirmish_formation(num_units, center),
            FormationType::Scattered => Self::scattered_formation(num_units, center),
        }
    }

    fn line_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let width = num_units as i32;
        let start_x = center.x - width / 2;

        for i in 0..num_units {
            positions.push(Position {
                x: start_x + i as i32,
                y: center.y,
                z: center.z,
            });
        }

        positions
    }

    fn box_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let side = (num_units as f32).sqrt().ceil() as i32;

        for i in 0..num_units {
            let row = i as i32 / side;
            let col = i as i32 % side;

            positions.push(Position {
                x: center.x - side / 2 + col,
                y: center.y - side / 2 + row,
                z: center.z,
            });
        }

        positions
    }

    fn column_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let height = num_units as i32;
        let start_y = center.y - height / 2;

        for i in 0..num_units {
            positions.push(Position {
                x: center.x,
                y: start_y + i as i32,
                z: center.z,
            });
        }

        positions
    }

    fn wedge_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut row = 0;
        let mut units_placed = 0;

        while units_placed < num_units {
            let units_in_row = row + 1;
            let start_x = center.x - row / 2;

            for col in 0..units_in_row {
                if units_placed >= num_units {
                    break;
                }

                positions.push(Position {
                    x: start_x + col,
                    y: center.y - row,
                    z: center.z,
                });

                units_placed += 1;
            }

            row += 1;
        }

        positions
    }

    fn circle_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let radius = (num_units as f32 / 6.28).max(2.0);

        for i in 0..num_units {
            let angle = (i as f32 / num_units as f32) * 6.28318;
            let x = center.x + (radius * angle.cos()).round() as i32;
            let y = center.y + (radius * angle.sin()).round() as i32;

            positions.push(Position { x, y, z: center.z });
        }

        positions
    }

    fn phalanx_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let ranks = ((num_units as f32) / 4.0).ceil() as i32;
        let files = 4; // 4 units wide

        for i in 0..num_units {
            let rank = i as i32 / files;
            let file = i as i32 % files;

            positions.push(Position {
                x: center.x - files / 2 + file,
                y: center.y + rank - ranks / 2,
                z: center.z,
            });
        }

        positions
    }

    fn flank_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let wing_size = num_units / 3;

        // Left wing
        for i in 0..wing_size {
            positions.push(Position {
                x: center.x - 2,
                y: center.y + i as i32,
                z: center.z,
            });
        }

        // Center
        for i in 0..(num_units - wing_size * 2) {
            positions.push(Position {
                x: center.x,
                y: center.y + i as i32,
                z: center.z,
            });
        }

        // Right wing
        for i in 0..wing_size {
            positions.push(Position {
                x: center.x + 2,
                y: center.y + i as i32,
                z: center.z,
            });
        }

        positions
    }

    fn skirmish_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let spacing = 2; // Loose formation

        let side = (num_units as f32).sqrt().ceil() as i32;

        for i in 0..num_units {
            let row = i as i32 / side;
            let col = i as i32 % side;

            positions.push(Position {
                x: center.x - (side * spacing) / 2 + col * spacing,
                y: center.y - (side * spacing) / 2 + row * spacing,
                z: center.z,
            });
        }

        positions
    }

    fn scattered_formation(num_units: usize, center: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..num_units {
            positions.push(Position {
                x: center.x + rng.gen_range(-3..=3),
                y: center.y + rng.gen_range(-3..=3),
                z: center.z,
            });
        }

        positions
    }
}

// ============================================================================
// SQUAD BONUSES
// ============================================================================

pub struct FormationBonuses;

impl FormationBonuses {
    pub fn get_attack_bonus(formation: &FormationType) -> f32 {
        match formation {
            FormationType::Wedge => 1.2,       // 20% attack bonus
            FormationType::Phalanx => 0.9,     // Attack penalty but defense bonus
            FormationType::Flank => 1.15,      // 15% attack bonus
            _ => 1.0,
        }
    }

    pub fn get_defense_bonus(formation: &FormationType) -> f32 {
        match formation {
            FormationType::Phalanx => 1.3,     // 30% defense bonus
            FormationType::Circle => 1.2,      // 20% defense bonus
            FormationType::Box => 1.1,         // 10% defense bonus
            FormationType::Scattered => 0.9,   // Defense penalty
            _ => 1.0,
        }
    }

    pub fn get_movement_speed(formation: &FormationType) -> f32 {
        match formation {
            FormationType::Column => 1.2,      // 20% faster
            FormationType::Scattered => 1.15,  // 15% faster
            FormationType::Phalanx => 0.8,     // 20% slower
            FormationType::Box => 0.9,         // 10% slower
            _ => 1.0,
        }
    }
}

// ============================================================================
// FORMATION PLUGIN
// ============================================================================

pub struct FormationPlugin;

impl Plugin for FormationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                formation_movement_system,
                squad_cohesion_system,
                formation_combat_bonuses,
            )
                .run_if(bevy::time::common_conditions::on_timer(
                    bevy::utils::Duration::from_secs_f32(0.5),
                )),
        );
    }
}

fn formation_movement_system(
    mut commands: Commands,
    squad_query: Query<&Squad>,
    mut unit_query: Query<(Entity, &mut Position, &FormationPosition)>,
) {
    for squad in squad_query.iter() {
        if squad.members.is_empty() {
            continue;
        }

        // Calculate formation center (use first member for now)
        if let Some(&first) = squad.members.first() {
            if let Ok((_, center_pos, _)) = unit_query.get(first) {
                // Calculate formation positions
                let positions = FormationCalculator::calculate_positions(
                    &squad.formation,
                    squad.members.len(),
                    center_pos,
                );

                // Assign positions to units
                for (i, &member) in squad.members.iter().enumerate() {
                    if i < positions.len() {
                        if let Ok((entity, _, _)) = unit_query.get(member) {
                            // Set pathfinding target to formation position
                            commands.entity(entity).insert(Pathing {
                                destination: positions[i],
                                path: vec![],
                                unreachable: false,
                                moving_target: false,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn squad_cohesion_system(
    mut squad_query: Query<&mut Squad>,
    unit_query: Query<&Position>,
) {
    for mut squad in squad_query.iter_mut() {
        // Remove dead/despawned members
        squad.members.retain(|&member| unit_query.get(member).is_ok());

        // If squad is too spread out, regroup
        if squad.members.len() > 1 {
            let positions: Vec<&Position> = squad
                .members
                .iter()
                .filter_map(|&m| unit_query.get(m).ok())
                .collect();

            if positions.len() > 1 {
                // Calculate center
                let center_x = positions.iter().map(|p| p.x).sum::<i32>() / positions.len() as i32;
                let center_y = positions.iter().map(|p| p.y).sum::<i32>() / positions.len() as i32;

                // Check max distance from center
                let max_distance = positions
                    .iter()
                    .map(|p| {
                        ((p.x - center_x).pow(2) + (p.y - center_y).pow(2)) as f32
                    })
                    .fold(0.0, f32::max)
                    .sqrt();

                // If too spread out (> 10 tiles), trigger regroup
                if max_distance > 10.0 {
                    // Regroup logic would go here
                }
            }
        }
    }
}

fn formation_combat_bonuses(
    squad_query: Query<&Squad>,
    mut unit_query: Query<(&mut PhysicalBody, &FormationPosition)>,
) {
    for squad in squad_query.iter() {
        let attack_bonus = FormationBonuses::get_attack_bonus(&squad.formation);
        let defense_bonus = FormationBonuses::get_defense_bonus(&squad.formation);

        for &member in &squad.members {
            if let Ok((mut body, _)) = unit_query.get_mut(member) {
                // Apply formation bonuses (would need base stats to do this properly)
                // For now, this is a placeholder
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn create_squad(
    commands: &mut Commands,
    squad_id: u32,
    members: Vec<Entity>,
    formation: FormationType,
) {
    for (i, &member) in members.iter().enumerate() {
        commands.entity(member).insert(FormationPosition {
            squad_id,
            relative_position: Vec2::ZERO, // Will be calculated
            rank: i as i32 / 4,
        });
    }

    commands.spawn(Squad {
        squad_id,
        members: members.clone(),
        formation,
        stance: CombatStance::Aggressive,
        rally_point: None,
        leader: members.first().copied(),
    });
}
