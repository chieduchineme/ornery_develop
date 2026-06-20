mod events;
mod formation;
mod physics;
mod state;
mod tactical;
mod vec2;

pub use state::{SpatialFrame, SpatialMatchState};

use std::collections::HashMap;
use rand::Rng;

use crate::event::MatchEvent;
use crate::types::{PlayerData, Side, Zone};
use state::{BallState, PlayerSpatialState, TeamSpatialState};

/// Number of physics ticks to run per logical match minute.
const TICKS_PER_MINUTE: u32 = 20;

// ---------------------------------------------------------------------------
// SpatialSimulator — maintains spatial state and produces per-minute frames
// ---------------------------------------------------------------------------

pub struct SpatialSimulator {
    state: SpatialMatchState,
    frames: Vec<SpatialFrame>,
}

impl SpatialSimulator {
    /// Initialise with starting 11 for each team and their formations.
    pub fn new(
        home_players: &[PlayerData],
        home_formation: &str,
        away_players: &[PlayerData],
        away_formation: &str,
    ) -> Self {
        let home_positions = formation::base_positions(home_formation, Side::Home);
        let away_positions = formation::base_positions(away_formation, Side::Away);

        let home_spatial: Vec<PlayerSpatialState> = home_players
            .iter()
            .zip(home_positions.iter().copied())
            .map(|(p, pos)| PlayerSpatialState::new(p, pos))
            .collect();

        let away_spatial: Vec<PlayerSpatialState> = away_players
            .iter()
            .zip(away_positions.iter().copied())
            .map(|(p, pos)| PlayerSpatialState::new(p, pos))
            .collect();

        let state = SpatialMatchState {
            home: TeamSpatialState { side: Side::Home, players: home_spatial, depth: 0.0 },
            away: TeamSpatialState { side: Side::Away, players: away_spatial, depth: 0.0 },
            ball: BallState::at(50.0, 50.0),
            possession: Side::Home,
            ball_zone: Zone::Midfield,
        };

        Self { state, frames: Vec::with_capacity(110) }
    }

    /// Advance one logical minute of the match, reacting to the given events,
    /// then store the resulting positions as a `SpatialFrame` for that minute.
    pub fn advance<R: Rng>(
        &mut self,
        events: &[MatchEvent],
        minute: u8,
        possession: Side,
        ball_zone: Zone,
        rng: &mut R,
    ) {
        self.state.possession = possession;
        self.state.ball_zone = ball_zone;

        if events.is_empty() {
            // No events: hold possession shape and run physics
            self.assign_and_tick_many(TICKS_PER_MINUTE, rng);
        } else {
            // Spread ticks evenly across events; minimum 3 ticks per event
            let ticks_each = (TICKS_PER_MINUTE / events.len() as u32).max(3);
            for event in events {
                // React spatially to the event
                events::react(&mut self.state, event, rng);
                // Re-assign roles after possession/zone may have changed
                self.assign_roles(rng);
                // Physics burst to animate the consequence
                self.tick_many(ticks_each, rng);
            }
        }

        self.frames.push(self.capture(minute));
    }

    // ── internals ────────────────────────────────────────────────────────────

    fn assign_roles<R: Rng>(&mut self, rng: &mut R) {
        let possession = self.state.possession;
        let zone = self.state.ball_zone;
        let ball_pos = self.state.ball.pos;
        tactical::assign_roles_and_targets(&mut self.state.home, possession, zone, ball_pos, rng);
        tactical::assign_roles_and_targets(&mut self.state.away, possession, zone, ball_pos, rng);
    }

    fn assign_and_tick_many<R: Rng>(&mut self, ticks: u32, rng: &mut R) {
        self.assign_roles(rng);
        self.tick_many(ticks, rng);
    }

    fn tick_many<R: Rng>(&mut self, ticks: u32, rng: &mut R) {
        for i in 0..ticks {
            // Occasionally re-jitter targets so movement stays organic
            if i % 5 == 0 {
                self.assign_roles(rng);
            }
            self.tick_once();
        }
    }

    fn tick_once(&mut self) {
        // Tick ball (friction, position update)
        physics::tick_ball(&mut self.state.ball);

        // Tick all players
        let carrier_id = self.state.ball.carrier_id.clone();
        for p in self.state.home.players.iter_mut().chain(self.state.away.players.iter_mut()) {
            physics::tick_player(p);
            // If this player is the ball carrier, ball sticks to them
            if carrier_id.as_deref() == Some(&p.id) {
                self.state.ball.pos = p.pos;
            }
        }
    }

    fn capture(&self, minute: u8) -> SpatialFrame {
        let mut players = HashMap::with_capacity(22);
        for p in self.state.home.players.iter().chain(self.state.away.players.iter()) {
            players.insert(p.id.clone(), (p.pos.x, p.pos.y));
        }
        SpatialFrame {
            minute,
            ball_x: self.state.ball.pos.x,
            ball_y: self.state.ball.pos.y,
            possession: self.state.possession,
            ball_zone: self.state.ball_zone,
            players,
        }
    }

    // ── public accessors ─────────────────────────────────────────────────────

    pub fn frame_at(&self, minute: u8) -> Option<&SpatialFrame> {
        // Search from end — latest frame for a given minute wins
        self.frames.iter().rev().find(|f| f.minute == minute)
    }

    pub fn all_frames(&self) -> &[SpatialFrame] {
        &self.frames
    }

    pub fn frames_in_range(&self, start: u8, end: u8) -> Vec<&SpatialFrame> {
        self.frames.iter().filter(|f| f.minute >= start && f.minute <= end).collect()
    }
}
