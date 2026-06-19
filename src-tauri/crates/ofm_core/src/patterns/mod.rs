use rand::Rng;

use engine::{LiveMatchState, MatchCommand, MatchSnapshot, PlayStyle, Side};

mod attacking;
mod defensive;
pub mod attacking_forms;
pub mod attacking_systems;
pub mod types;

pub use attacking::AttackingPattern;
pub use defensive::DefensivePattern;
pub use attacking_forms::all_forms;
pub use attacking_systems::all_systems;

pub fn decide_match_commands<R: Rng>(
    match_state: &LiveMatchState,
    side: Side,
    rng: &mut R,
) -> Vec<MatchCommand> {
    let snapshot = match_state.snapshot();

    if snapshot.possession == side {
        attacking::decide_attacking_commands(&snapshot, side, rng)
    } else {
        defensive::decide_defensive_commands(&snapshot, side, rng)
    }
}

fn team_play_style(snapshot: &MatchSnapshot, side: Side) -> PlayStyle {
    match side {
        Side::Home => snapshot.home_team.play_style,
        Side::Away => snapshot.away_team.play_style,
    }
}

fn team_goal_difference(snapshot: &MatchSnapshot, side: Side) -> i8 {
    let (own, opp) = match side {
        Side::Home => (snapshot.home_score, snapshot.away_score),
        Side::Away => (snapshot.away_score, snapshot.home_score),
    };
    own as i8 - opp as i8
}

fn team_formation(snapshot: &MatchSnapshot, side: Side) -> &str {
    match side {
        Side::Home => &snapshot.home_team.formation,
        Side::Away => &snapshot.away_team.formation,
    }
}
