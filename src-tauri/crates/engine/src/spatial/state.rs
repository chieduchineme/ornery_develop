use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::{PlayerData, Side, Zone};
use super::vec2::Vec2;

// ── Ball ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BallState {
    pub pos: Vec2,
    pub vel: Vec2,
    /// Player id currently carrying/dribbling the ball, or None if loose.
    pub carrier_id: Option<String>,
}

impl BallState {
    pub fn at(x: f64, y: f64) -> Self {
        Self { pos: Vec2::new(x, y), vel: Vec2::default(), carrier_id: None }
    }

    pub fn speed(&self) -> f64 {
        self.vel.magnitude()
    }
}

// ── Player ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalRole {
    HoldShape,
    BuildUp,
    ProgressBall,
    AttackRun,
    SupportRun,
    HighPress,
    MidBlock,
    LowBlock,
    CloseDown,
    TrackRunner,
}

#[derive(Debug, Clone)]
pub struct PlayerSpatialState {
    pub id: String,
    pub pos: Vec2,
    pub vel: Vec2,
    /// Where the player is trying to reach this tick.
    pub target: Vec2,
    /// Base position from the team formation.
    pub formation_pos: Vec2,
    /// Max speed in units/tick (derived from pace attribute).
    pub max_speed: f64,
    pub role: TacticalRole,
}

impl PlayerSpatialState {
    pub fn new(player: &PlayerData, formation_pos: Vec2) -> Self {
        // pace 100 → 1.1 units/tick; pace 50 → 0.65 units/tick
        let max_speed = 0.50 + (player.pace as f64 / 100.0) * 0.65;
        Self {
            id: player.id.clone(),
            pos: formation_pos,
            vel: Vec2::default(),
            target: formation_pos,
            formation_pos,
            max_speed,
            role: TacticalRole::HoldShape,
        }
    }
}

// ── Team ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TeamSpatialState {
    pub side: Side,
    pub players: Vec<PlayerSpatialState>,
    /// Team depth: -1 = fully defensive, 0 = neutral, +1 = fully attacking
    pub depth: f64,
}

impl TeamSpatialState {
    pub fn centroid(&self) -> Vec2 {
        let n = self.players.len() as f64;
        if n == 0.0 { return Vec2::new(50.0, 50.0); }
        let (sx, sy) = self.players.iter().fold((0.0, 0.0), |(ax, ay), p| {
            (ax + p.pos.x, ay + p.pos.y)
        });
        Vec2::new(sx / n, sy / n)
    }
}

// ── Spatial match state ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SpatialMatchState {
    pub home: TeamSpatialState,
    pub away: TeamSpatialState,
    pub ball: BallState,
    pub possession: Side,
    pub ball_zone: Zone,
}

// ── Output frame ─────────────────────────────────────────────────────────────

/// One snapshot of spatial state, keyed by match minute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialFrame {
    pub minute: u8,
    pub ball_x: f64,
    pub ball_y: f64,
    pub possession: Side,
    pub ball_zone: Zone,
    /// player_id → (x, y)
    pub players: HashMap<String, (f64, f64)>,
}
