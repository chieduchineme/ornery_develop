use super::state::{BallState, PlayerSpatialState};

/// Ball friction per tick (speed multiplier). Ball decelerates naturally.
const BALL_FRICTION: f64 = 0.90;
/// How quickly a player steers toward their new velocity target (0=instant, 1=no change).
const PLAYER_STEER: f64 = 0.30;
/// Braking multiplier when player is near their target.
const PLAYER_BRAKE: f64 = 0.68;
/// Stop trying to accelerate if closer than this to target.
const AT_TARGET_DIST: f64 = 0.6;
/// Minimum speed for ball to have any effect (below this = stopped).
const BALL_STOP_SPEED: f64 = 0.05;

pub fn tick_ball(ball: &mut BallState) {
    if ball.carrier_id.is_some() {
        return; // ball moves with carrier — handled by tick_player
    }
    if ball.speed() < BALL_STOP_SPEED {
        ball.vel = Default::default();
        return;
    }
    ball.pos = ball.pos.add(ball.vel).clamp_to_pitch();
    ball.vel.x *= BALL_FRICTION;
    ball.vel.y *= BALL_FRICTION;
}

pub fn tick_player(player: &mut PlayerSpatialState) {
    let dx = player.target.x - player.pos.x;
    let dy = player.target.y - player.pos.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < AT_TARGET_DIST {
        player.vel.x *= PLAYER_BRAKE;
        player.vel.y *= PLAYER_BRAKE;
    } else {
        let desired_vx = (dx / dist) * player.max_speed;
        let desired_vy = (dy / dist) * player.max_speed;
        player.vel.x += (desired_vx - player.vel.x) * PLAYER_STEER;
        player.vel.y += (desired_vy - player.vel.y) * PLAYER_STEER;

        // Cap at max speed
        let speed = (player.vel.x * player.vel.x + player.vel.y * player.vel.y).sqrt();
        if speed > player.max_speed {
            let s = player.max_speed / speed;
            player.vel.x *= s;
            player.vel.y *= s;
        }
    }

    player.pos.x = (player.pos.x + player.vel.x).clamp(2.0, 98.0);
    player.pos.y = (player.pos.y + player.vel.y).clamp(3.0, 97.0);
}

/// Fire the ball from its current position toward a target at the given speed (units/tick).
pub fn fire_ball(ball: &mut BallState, target: super::vec2::Vec2, speed: f64) {
    ball.carrier_id = None;
    let dir = ball.pos.direction_to(target);
    ball.vel.x = dir.x * speed;
    ball.vel.y = dir.y * speed;
}
