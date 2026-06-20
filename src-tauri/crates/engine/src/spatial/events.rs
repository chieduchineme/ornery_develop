use rand::{Rng, RngExt};

use crate::event::{EventType, MatchEvent};
use crate::types::{Side, Zone};
use super::physics::fire_ball;
use super::state::SpatialMatchState;
use super::tactical::zone_x_range;
use super::vec2::Vec2;

/// React to a logical match event: update ball trajectory / state to match.
pub fn react<R: Rng>(state: &mut SpatialMatchState, event: &MatchEvent, rng: &mut R) {
    match event.event_type {
        EventType::KickOff | EventType::SecondHalfStart => {
            state.ball.pos = Vec2::new(50.0, 50.0);
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            state.ball_zone = Zone::Midfield;
            state.possession = event.side;
        }

        EventType::PassCompleted | EventType::Cross => {
            // Ball moves toward next zone with realistic pass speed
            let next_zone = event.zone.advance_towards(event.side);
            let (x_min, x_max) = zone_x_range(next_zone);
            let target_x = x_min + rng.random::<f64>() * (x_max - x_min);
            let target_y = 18.0 + rng.random::<f64>() * 64.0;
            let target = Vec2::new(target_x, target_y);
            let dist = state.ball.pos.dist(target).max(1.0);
            let speed = (1.2 + dist / 30.0).min(2.6);
            fire_ball(&mut state.ball, target, speed);
            state.ball_zone = next_zone;
            state.possession = event.side;
        }

        EventType::Dribble => {
            // Ball stays with dribbler, nudges toward next zone
            let next_zone = event.zone.advance_towards(event.side);
            let (x_min, x_max) = zone_x_range(next_zone);
            let tx = x_min + rng.random::<f64>() * (x_max - x_min);
            let ty = state.ball.pos.y + (rng.random::<f64>() - 0.5) * 10.0;
            let target = Vec2::new(tx, ty);
            fire_ball(&mut state.ball, target, 0.9);
            state.ball_zone = next_zone;
            state.possession = event.side;
        }

        EventType::PassIntercepted | EventType::DribbleTackled | EventType::Interception => {
            // Possession flip + deflection
            state.possession = event.side;
            let deflect_x = state.ball.vel.x * -0.3 + (rng.random::<f64>() - 0.5) * 0.4;
            let deflect_y = state.ball.vel.y * -0.3 + (rng.random::<f64>() - 0.5) * 0.6;
            state.ball.vel.x = deflect_x;
            state.ball.vel.y = deflect_y;
            state.ball.carrier_id = None;
        }

        EventType::Tackle => {
            state.possession = event.side;
            state.ball.vel.x *= -0.2;
            state.ball.vel.y = (rng.random::<f64>() - 0.5) * 0.8;
            state.ball.carrier_id = None;
        }

        EventType::Clearance => {
            let tx = 38.0 + rng.random::<f64>() * 24.0;
            let ty = 15.0 + rng.random::<f64>() * 70.0;
            fire_ball(&mut state.ball, Vec2::new(tx, ty), 3.0 + rng.random::<f64>() * 0.8);
            state.ball_zone = Zone::Midfield;
        }

        EventType::ShotOnTarget => {
            let goal_x = if event.side == Side::Home { 98.0 } else { 2.0 };
            let goal_y = 43.0 + rng.random::<f64>() * 14.0;
            fire_ball(&mut state.ball, Vec2::new(goal_x, goal_y), 3.8 + rng.random::<f64>() * 0.6);
            state.ball_zone = Zone::attacking_box(event.side);
        }

        EventType::ShotOffTarget | EventType::ShotBlocked => {
            let tx = 35.0 + (rng.random::<f64>() - 0.5) * 30.0;
            let ty = 15.0 + rng.random::<f64>() * 70.0;
            fire_ball(&mut state.ball, Vec2::new(tx, ty), 1.8 + rng.random::<f64>() * 0.5);
            state.ball_zone = Zone::Midfield;
            state.possession = event.side.opposite();
        }

        EventType::ShotSaved | EventType::GoalKick => {
            let gk_x = if event.side == Side::Home { 4.0 } else { 96.0 };
            let gk_y = 44.0 + rng.random::<f64>() * 12.0;
            state.ball.pos = Vec2::new(gk_x, gk_y);
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            state.possession = event.side.opposite();
            state.ball_zone = if event.side == Side::Home {
                Zone::HomeBox
            } else {
                Zone::AwayBox
            };
        }

        EventType::Goal | EventType::PenaltyGoal => {
            // Ball in net; then logically it resets but visually we park near the net briefly
            let net_x = if event.side == Side::Home { 99.0 } else { 1.0 };
            state.ball.pos = Vec2::new(net_x, 50.0 + (rng.random::<f64>() - 0.5) * 6.0);
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            // Will be reset to kickoff after the tick burst
        }

        EventType::Corner => {
            let cx = if event.side == Side::Home { 98.5 } else { 1.5 };
            let cy: f64 = if rng.random::<bool>() { 3.5 } else { 96.5 };
            state.ball.pos = Vec2::new(cx, cy);
            // Fire toward the box
            let target_x = if event.side == Side::Home { 89.0 } else { 11.0 };
            fire_ball(&mut state.ball, Vec2::new(target_x, 48.0 + rng.random::<f64>() * 4.0), 2.8);
            state.ball_zone = Zone::attacking_box(event.side);
            state.possession = event.side;
        }

        EventType::FreeKick => {
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            state.possession = event.side;
        }

        EventType::PenaltyAwarded => {
            let spot_x = if event.side == Side::Home { 89.0 } else { 11.0 };
            state.ball.pos = Vec2::new(spot_x, 50.0);
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            state.ball_zone = Zone::attacking_box(event.side);
        }

        EventType::PenaltyMiss => {
            let tx = 35.0 + (rng.random::<f64>() - 0.5) * 20.0;
            fire_ball(&mut state.ball, Vec2::new(tx, 30.0 + rng.random::<f64>() * 40.0), 2.5);
            state.possession = event.side.opposite();
        }

        EventType::Foul | EventType::YellowCard | EventType::RedCard |
        EventType::SecondYellow => {
            // Ball stops at foul location
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
            state.possession = event.side.opposite();
        }

        EventType::Substitution | EventType::Injury |
        EventType::HalfTime | EventType::FullTime => {
            state.ball.vel = Vec2::default();
            state.ball.carrier_id = None;
        }
    }
}
