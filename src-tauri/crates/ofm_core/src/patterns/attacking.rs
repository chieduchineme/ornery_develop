use rand::Rng;

use engine::{MatchCommand, MatchSnapshot, PlayStyle, Side};

use crate::patterns::attacking_systems;

pub fn decide_attacking_commands<R: Rng>(
    snapshot: &MatchSnapshot,
    side: Side,
    rng: &mut R,
) -> Vec<MatchCommand> {
    let mut commands = Vec::new();
    let play_style = team_play_style(snapshot, side);
    let goal_diff = team_goal_difference(snapshot, side);
    let minute = snapshot.current_minute;

    // Select an elite attacking system that fits the team's play style.
    let system = {
        let mut candidates = attacking_systems::systems_for_play_style(play_style);
        if candidates.is_empty() {
            candidates = attacking_systems::all_systems();
        }
        let idx = rng.gen_range(0..candidates.len());
        candidates.swap_remove(idx)
    };

    // Pick which phase of the system to execute based on match minute.
    let phase_idx = if minute < 30 {
        0
    } else if minute < 65 {
        1
    } else {
        2
    };
    let form_id = system
        .phases
        .get(phase_idx)
        .map(|p| p.attacking_form_id.clone())
        .unwrap_or_else(|| system.phases[0].attacking_form_id.clone());

    commands.push(MatchCommand::SetAttackingPattern {
        side,
        pattern_id: form_id,
    });

    // Play style adjustments driven by match situation.
    match play_style {
        PlayStyle::Attacking | PlayStyle::HighPress | PlayStyle::Counter => {}
        PlayStyle::Possession => {
            if rng.gen_bool(0.15) {
                commands.push(MatchCommand::ChangePlayStyle {
                    side,
                    play_style: PlayStyle::Attacking,
                });
            }
        }
        PlayStyle::Defensive => {
            if minute >= 60 && rng.gen_bool(0.20) {
                commands.push(MatchCommand::ChangePlayStyle {
                    side,
                    play_style: PlayStyle::Attacking,
                });
            }
        }
        _ => {}
    }

    // Losing late — apply the system's own play style urgency.
    if goal_diff < 0 && minute >= 55 && rng.gen_bool(0.25) {
        commands.push(MatchCommand::ChangePlayStyle {
            side,
            play_style: system.base_play_style,
        });
    }

    // Winning late — keep the ball.
    if goal_diff > 0 && minute >= 70 && rng.gen_bool(0.12) {
        commands.push(MatchCommand::ChangePlayStyle {
            side,
            play_style: PlayStyle::Possession,
        });
    }

    commands
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
