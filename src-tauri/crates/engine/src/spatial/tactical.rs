use rand::{Rng, RngExt};
use crate::types::{Side, Zone};
use super::formation::push_forward;
use super::state::{PlayerSpatialState, TacticalRole, TeamSpatialState};
use super::vec2::Vec2;

/// X coordinate of the center of a zone.
pub fn zone_center_x(zone: Zone) -> f64 {
    match zone {
        Zone::HomeBox      => 9.0,
        Zone::HomeDefense  => 27.0,
        Zone::Midfield     => 50.0,
        Zone::AwayDefense  => 73.0,
        Zone::AwayBox      => 91.0,
    }
}

/// X range (min, max) for a zone.
pub fn zone_x_range(zone: Zone) -> (f64, f64) {
    match zone {
        Zone::HomeBox      => (2.0,  18.0),
        Zone::HomeDefense  => (18.0, 38.0),
        Zone::Midfield     => (38.0, 62.0),
        Zone::AwayDefense  => (62.0, 82.0),
        Zone::AwayBox      => (82.0, 98.0),
    }
}

/// Target depth for a team based on possession and ball zone.
fn target_depth(is_attacking: bool, zone: Zone, side: Side) -> f64 {
    if is_attacking {
        // Ball zone in attack → push team forward more
        let zone_push = if side == Side::Home {
            match zone {
                Zone::AwayBox      => 1.0,
                Zone::AwayDefense  => 0.7,
                Zone::Midfield     => 0.3,
                Zone::HomeDefense  => 0.0,
                Zone::HomeBox      => -0.2,
            }
        } else {
            match zone {
                Zone::HomeBox      => 1.0,
                Zone::HomeDefense  => 0.7,
                Zone::Midfield     => 0.3,
                Zone::AwayDefense  => 0.0,
                Zone::AwayBox      => -0.2,
            }
        };
        zone_push
    } else {
        // Defending: compress backward
        let zone_pull = if side == Side::Home {
            match zone {
                Zone::HomeBox      => -0.85,
                Zone::HomeDefense  => -0.55,
                Zone::Midfield     => -0.2,
                Zone::AwayDefense  => 0.1,
                Zone::AwayBox      => 0.3,
            }
        } else {
            match zone {
                Zone::AwayBox      => -0.85,
                Zone::AwayDefense  => -0.55,
                Zone::Midfield     => -0.2,
                Zone::HomeDefense  => 0.1,
                Zone::HomeBox      => 0.3,
            }
        };
        zone_pull
    }
}

/// Is the player's formation position in the forward line?
fn is_in_forward_line(formation_pos: Vec2, side: Side) -> bool {
    if side == Side::Home {
        formation_pos.x > 62.0
    } else {
        formation_pos.x < 38.0
    }
}

/// Is the player's formation position in the midfield line?
fn is_in_midfield(formation_pos: Vec2, side: Side) -> bool {
    if side == Side::Home {
        formation_pos.x > 40.0 && formation_pos.x <= 62.0
    } else {
        formation_pos.x < 60.0 && formation_pos.x >= 38.0
    }
}

fn attacking_role<R: Rng>(
    player: &PlayerSpatialState,
    side: Side,
    zone: Zone,
    rng: &mut R,
) -> TacticalRole {
    let r: f64 = rng.random();
    if is_in_forward_line(player.formation_pos, side) {
        match zone {
            Zone::AwayBox | Zone::AwayDefense => {
                if r < 0.55 { TacticalRole::AttackRun } else { TacticalRole::SupportRun }
            }
            Zone::Midfield => {
                if r < 0.45 { TacticalRole::SupportRun } else { TacticalRole::ProgressBall }
            }
            _ => TacticalRole::ProgressBall,
        }
    } else if is_in_midfield(player.formation_pos, side) {
        match zone {
            Zone::AwayBox | Zone::AwayDefense => {
                if r < 0.5 { TacticalRole::SupportRun } else { TacticalRole::BuildUp }
            }
            Zone::Midfield => {
                if r < 0.6 { TacticalRole::ProgressBall } else { TacticalRole::SupportRun }
            }
            _ => TacticalRole::BuildUp,
        }
    } else {
        // Defender
        match zone {
            Zone::AwayBox | Zone::AwayDefense => {
                if r < 0.25 { TacticalRole::SupportRun } else { TacticalRole::BuildUp }
            }
            _ => TacticalRole::HoldShape,
        }
    }
}

fn defensive_role<R: Rng>(
    player: &PlayerSpatialState,
    side: Side,
    zone: Zone,
    rng: &mut R,
) -> TacticalRole {
    let r: f64 = rng.random();
    if is_in_forward_line(player.formation_pos, side) {
        // Forwards press or screen
        match zone {
            Zone::HomeBox | Zone::HomeDefense => {
                if r < 0.45 { TacticalRole::MidBlock } else { TacticalRole::HoldShape }
            }
            Zone::Midfield | Zone::AwayDefense => {
                if r < 0.6 { TacticalRole::HighPress } else { TacticalRole::HoldShape }
            }
            _ => TacticalRole::HoldShape,
        }
    } else if is_in_midfield(player.formation_pos, side) {
        match zone {
            Zone::HomeBox | Zone::HomeDefense => {
                if r < 0.7 { TacticalRole::LowBlock } else { TacticalRole::CloseDown }
            }
            Zone::Midfield => {
                if r < 0.55 { TacticalRole::MidBlock } else { TacticalRole::CloseDown }
            }
            _ => TacticalRole::MidBlock,
        }
    } else {
        // Defenders
        match zone {
            Zone::HomeBox | Zone::HomeDefense => {
                if r < 0.6 { TacticalRole::LowBlock } else { TacticalRole::CloseDown }
            }
            _ => TacticalRole::HoldShape,
        }
    }
}

fn target_for_role<R: Rng>(
    player: &PlayerSpatialState,
    role: TacticalRole,
    side: Side,
    zone: Zone,
    ball_pos: Vec2,
    team_depth: f64,
    rng: &mut R,
) -> Vec2 {
    let jx: f64 = (rng.random::<f64>() - 0.5) * 3.0;
    let jy: f64 = (rng.random::<f64>() - 0.5) * 4.0;

    // Base = formation pos shifted by team depth
    let depth_shift = team_depth * 14.0;
    let base_x = push_forward(player.formation_pos.x, side, depth_shift);
    let base = Vec2::new(base_x, player.formation_pos.y);

    match role {
        TacticalRole::HoldShape => Vec2::new(
            (base.x + jx * 0.5).clamp(3.0, 97.0),
            (base.y + jy * 0.5).clamp(5.0, 95.0),
        ),

        TacticalRole::BuildUp => Vec2::new(
            (base.x + jx).clamp(3.0, 97.0),
            (base.y + jy).clamp(5.0, 95.0),
        ),

        TacticalRole::ProgressBall => {
            // Move toward zone center
            let zx = zone_center_x(zone.advance_towards(side));
            Vec2::new(
                (zx + jx).clamp(3.0, 97.0),
                (base.y + jy).clamp(5.0, 95.0),
            )
        }

        TacticalRole::AttackRun => {
            let goal_x = if side == Side::Home { 89.0 } else { 11.0 };
            Vec2::new(
                (goal_x + (rng.random::<f64>() - 0.5) * 12.0).clamp(3.0, 97.0),
                (25.0 + rng.random::<f64>() * 50.0).clamp(5.0, 95.0),
            )
        }

        TacticalRole::SupportRun => Vec2::new(
            (base.x + jx * 1.5).clamp(3.0, 97.0),
            (base.y + (rng.random::<f64>() - 0.5) * 14.0).clamp(5.0, 95.0),
        ),

        TacticalRole::HighPress => {
            // Press toward ball with some spread
            Vec2::new(
                (ball_pos.x + (rng.random::<f64>() - 0.5) * 10.0).clamp(3.0, 97.0),
                (ball_pos.y + (rng.random::<f64>() - 0.5) * 12.0).clamp(5.0, 95.0),
            )
        }

        TacticalRole::CloseDown => {
            // Move close to ball
            Vec2::new(
                (ball_pos.x + (rng.random::<f64>() - 0.5) * 7.0).clamp(3.0, 97.0),
                (ball_pos.y + (rng.random::<f64>() - 0.5) * 7.0).clamp(5.0, 95.0),
            )
        }

        TacticalRole::MidBlock => {
            // Hold mid shape close to ball's zone
            let block_x = if side == Side::Home {
                (zone_center_x(zone) - 8.0).clamp(15.0, 60.0)
            } else {
                (zone_center_x(zone) + 8.0).clamp(40.0, 85.0)
            };
            Vec2::new(
                (block_x + jx).clamp(3.0, 97.0),
                (base.y + jy).clamp(5.0, 95.0),
            )
        }

        TacticalRole::LowBlock => {
            // Compress deep
            let deep_x = if side == Side::Home {
                (base.x - 8.0).clamp(6.0, 40.0)
            } else {
                (base.x + 8.0).clamp(60.0, 94.0)
            };
            Vec2::new(
                (deep_x + jx * 0.5).clamp(3.0, 97.0),
                (base.y + jy * 0.5).clamp(5.0, 95.0),
            )
        }

        TacticalRole::TrackRunner => Vec2::new(
            (base.x + jx).clamp(3.0, 97.0),
            (base.y + jy).clamp(5.0, 95.0),
        ),
    }
}

/// Assign roles and targets to all players on a team.
pub fn assign_roles_and_targets<R: Rng>(
    team: &mut TeamSpatialState,
    possession: Side,
    zone: Zone,
    ball_pos: Vec2,
    rng: &mut R,
) {
    let is_attacking = team.side == possession;
    let td = target_depth(is_attacking, zone, team.side);
    // Smoothly interpolate team depth
    team.depth += (td - team.depth) * 0.28;

    let depth = team.depth;
    let side = team.side;

    for player in &mut team.players {
        let role = if is_attacking {
            attacking_role(player, side, zone, rng)
        } else {
            defensive_role(player, side, zone, rng)
        };
        player.role = role;
        player.target = target_for_role(player, role, side, zone, ball_pos, depth, rng);
    }
}
