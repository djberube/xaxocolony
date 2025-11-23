use bevy::prelude::*;
use crate::prelude::*;
use crate::inventory_system::*;

// ============================================================================
// RANGED COMBAT SYSTEM - Bows, Crossbows, Guns
// ============================================================================

#[derive(Component)]
pub struct RangedWeapon {
    pub weapon_type: RangedWeaponType,
    pub damage: i32,
    pub range: f32,
    pub accuracy: f32, // 0.0 to 1.0
    pub reload_time: f32,
    pub projectile_speed: f32,
    pub ammo_type: String,
    pub last_shot_time: f32,
}

#[derive(Clone, PartialEq)]
pub enum RangedWeaponType {
    Bow,
    Crossbow,
    Sling,
    Javelin,
    Gun,
    Cannon,
}

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub owner: Entity,
    pub target: Option<Entity>,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub piercing: bool,
    pub explosive: bool,
    pub explosion_radius: f32,
}

#[derive(Component)]
pub struct Ammo {
    pub ammo_type: String,
    pub count: u32,
}

// System to shoot ranged weapons
pub fn ranged_attack_system(
    mut commands: Commands,
    mut shooter_query: Query<(
        Entity,
        &Position,
        &mut Equipment,
        &mut Inventory,
        &Targeting,
        &PhysicalBody,
    )>,
    target_query: Query<&Position, With<Attackable>>,
    time: Res<Time>,
    texture: Res<SpriteSheetHandle>,
    atlas_layout: Res<SpriteSheetAtlasLayout>,
) {
    for (shooter_entity, shooter_pos, mut equipment, mut inventory, targeting, body) in
        shooter_query.iter_mut()
    {
        // Check if has ranged weapon equipped
        if let Some(weapon_item) = &equipment.weapon_main {
            if let ItemClass::Weapon(weapon_stats) = &weapon_item.item_def.item_class {
                // Only handle ranged weapons
                match weapon_stats.weapon_type {
                    WeaponType::Bow | WeaponType::Crossbow | WeaponType::Gun => {
                        // Check ammo
                        if let Some(ammo_type) = &weapon_stats.ammo_type {
                            if !inventory.has_item(ammo_type, 1) {
                                continue; // No ammo
                            }

                            // Get target position
                            if let Ok(target_pos) = target_query.get(targeting.target) {
                                let distance = shooter_pos.distance(target_pos);

                                // Check if in range
                                if distance as f32 <= weapon_stats.range {
                                    // Consume ammo
                                    inventory.remove_item(ammo_type, 1);

                                    // Calculate projectile direction
                                    let dx = (target_pos.x - shooter_pos.x) as f32;
                                    let dy = (target_pos.y - shooter_pos.y) as f32;
                                    let length = (dx * dx + dy * dy).sqrt();
                                    let velocity = Vec2::new(dx / length, dy / length) * 10.0;

                                    // Spawn projectile
                                    let projectile_sprite_index = match weapon_stats.weapon_type {
                                        WeaponType::Bow | WeaponType::Crossbow => 94 * 64 + 30,
                                        WeaponType::Gun => 94 * 64 + 31,
                                        _ => 94 * 64 + 30,
                                    };

                                    commands.spawn((
                                        SpriteSheetBundle {
                                            texture: texture.0.clone(),
                                            atlas: TextureAtlas {
                                                layout: atlas_layout.0.clone(),
                                                index: projectile_sprite_index,
                                            },
                                            transform: shooter_pos.to_transform_layer(5.0),
                                            ..default()
                                        },
                                        Projectile {
                                            damage: weapon_stats.damage
                                                + body.attributes.strength / 2,
                                            owner: shooter_entity,
                                            target: Some(targeting.target),
                                            velocity,
                                            lifetime: 5.0,
                                            piercing: false,
                                            explosive: false,
                                            explosion_radius: 0.0,
                                        },
                                        Position {
                                            x: shooter_pos.x,
                                            y: shooter_pos.y,
                                            z: 5,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                    _ => {} // Not a ranged weapon
                }
            }
        }
    }
}

// System to move projectiles
pub fn projectile_movement_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Position, &mut Transform, &mut Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut position, mut transform, mut projectile) in projectile_query.iter_mut() {
        // Move projectile
        let movement = projectile.velocity * time.delta_seconds() * 20.0;

        // Update position
        let new_x = position.x as f32 + movement.x;
        let new_y = position.y as f32 + movement.y;

        position.x = new_x.round() as i32;
        position.y = new_y.round() as i32;

        // Update transform smoothly
        transform.translation.x += movement.x * TILE_SIZE;
        transform.translation.y += movement.y * TILE_SIZE;

        // Decrease lifetime
        projectile.lifetime -= time.delta_seconds();

        // Remove if lifetime expired
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// System to handle projectile collisions
pub fn projectile_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Position, &Projectile)>,
    mut target_query: Query<(Entity, &Position, &mut PhysicalBody), With<Attackable>>,
    tile_hash: Res<TileHash>,
) {
    for (proj_entity, proj_pos, projectile) in projectile_query.iter() {
        // Check collision with walls
        if let Some(tile) = tile_hash.hash.get(proj_pos) {
            if tile.is_wall() {
                commands.entity(proj_entity).despawn();
                continue;
            }
        }

        // Check collision with targets
        for (target_entity, target_pos, mut target_body) in target_query.iter_mut() {
            // Don't hit the shooter
            if target_entity == projectile.owner {
                continue;
            }

            // Check if projectile hit target
            if proj_pos.x == target_pos.x && proj_pos.y == target_pos.y {
                // Apply damage
                target_body.attributes.health -= projectile.damage;

                // Apply knockback
                let dx = (target_pos.x - proj_pos.x) as f32;
                let dy = (target_pos.y - proj_pos.y) as f32;

                // Spawn hit effect
                // TODO: Add visual effect

                // Handle explosive projectiles
                if projectile.explosive {
                    apply_explosion(
                        &mut commands,
                        target_pos,
                        projectile.explosion_radius,
                        projectile.damage / 2,
                        &mut target_query,
                    );
                }

                // Remove projectile unless it's piercing
                if !projectile.piercing {
                    commands.entity(proj_entity).despawn();
                }
            }
        }
    }
}

// Helper function for explosive damage
fn apply_explosion(
    commands: &mut Commands,
    center: &Position,
    radius: f32,
    damage: i32,
    target_query: &mut Query<(Entity, &Position, &mut PhysicalBody), With<Attackable>>,
) {
    for (_, target_pos, mut target_body) in target_query.iter_mut() {
        let distance = center.distance(target_pos) as f32;

        if distance <= radius {
            // Damage falls off with distance
            let damage_multiplier = 1.0 - (distance / radius);
            let final_damage = (damage as f32 * damage_multiplier) as i32;
            target_body.attributes.health -= final_damage;
        }
    }

    // TODO: Spawn explosion visual effect
}

// ============================================================================
// PROJECTILE TYPES
// ============================================================================

pub struct ProjectileBuilder;

impl ProjectileBuilder {
    pub fn arrow(owner: Entity, position: Position, velocity: Vec2, damage: i32) -> (Projectile, Position) {
        (
            Projectile {
                damage,
                owner,
                target: None,
                velocity,
                lifetime: 3.0,
                piercing: false,
                explosive: false,
                explosion_radius: 0.0,
            },
            position,
        )
    }

    pub fn bolt(owner: Entity, position: Position, velocity: Vec2, damage: i32) -> (Projectile, Position) {
        (
            Projectile {
                damage: damage + 2, // Bolts do more damage
                owner,
                target: None,
                velocity,
                lifetime: 4.0,
                piercing: true, // Bolts can pierce
                explosive: false,
                explosion_radius: 0.0,
            },
            position,
        )
    }

    pub fn bullet(owner: Entity, position: Position, velocity: Vec2, damage: i32) -> (Projectile, Position) {
        (
            Projectile {
                damage,
                owner,
                target: None,
                velocity: velocity * 3.0, // Much faster
                lifetime: 2.0,
                piercing: true,
                explosive: false,
                explosion_radius: 0.0,
            },
            position,
        )
    }

    pub fn explosive_arrow(
        owner: Entity,
        position: Position,
        velocity: Vec2,
        damage: i32,
    ) -> (Projectile, Position) {
        (
            Projectile {
                damage,
                owner,
                target: None,
                velocity,
                lifetime: 3.0,
                piercing: false,
                explosive: true,
                explosion_radius: 2.0,
            },
            position,
        )
    }

    pub fn cannonball(
        owner: Entity,
        position: Position,
        velocity: Vec2,
        damage: i32,
    ) -> (Projectile, Position) {
        (
            Projectile {
                damage: damage * 3,
                owner,
                target: None,
                velocity: velocity * 0.5, // Slower
                lifetime: 5.0,
                piercing: false,
                explosive: true,
                explosion_radius: 4.0,
            },
            position,
        )
    }
}

// ============================================================================
// TARGETING SYSTEM FOR RANGED UNITS
// ============================================================================

pub fn ranged_targeting_system(
    mut commands: Commands,
    mut ranged_query: Query<(Entity, &Position, &Equipment, &mut Brain), Without<Targeting>>,
    target_query: Query<(Entity, &Position), With<Attackable>>,
) {
    for (entity, pos, equipment, mut brain) in ranged_query.iter_mut() {
        // Check if has ranged weapon
        if let Some(weapon_item) = &equipment.weapon_main {
            if let ItemClass::Weapon(weapon_stats) = &weapon_item.item_def.item_class {
                match weapon_stats.weapon_type {
                    WeaponType::Bow | WeaponType::Crossbow | WeaponType::Gun => {
                        // Find nearest enemy in range
                        let mut nearest_target = None;
                        let mut nearest_distance = weapon_stats.range;

                        for (target_entity, target_pos) in target_query.iter() {
                            if target_entity == entity {
                                continue;
                            }

                            let distance = pos.distance(target_pos) as f32;

                            if distance <= nearest_distance {
                                nearest_distance = distance;
                                nearest_target = Some(target_entity);
                            }
                        }

                        // Set target
                        if let Some(target) = nearest_target {
                            commands.entity(entity).insert(Targeting { target });
                            brain.task = Some(Task::Fight);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// ============================================================================
// ACCURACY SYSTEM
// ============================================================================

pub fn apply_accuracy(
    base_accuracy: f32,
    distance: f32,
    max_range: f32,
    skill_level: i32,
    weather_modifier: f32,
) -> bool {
    let mut rng = rand::thread_rng();

    // Distance penalty
    let distance_modifier = 1.0 - (distance / max_range) * 0.5;

    // Skill bonus
    let skill_modifier = 1.0 + (skill_level as f32 * 0.05);

    // Final accuracy
    let final_accuracy = base_accuracy * distance_modifier * skill_modifier * weather_modifier;

    // Roll for hit
    rng.gen::<f32>() < final_accuracy
}

pub fn combat_system_ranged(
    _commands: Commands,
) {
    // Legacy function kept for compatibility
}
