use rand::Rng;

use engine::{MatchCommand, MatchSnapshot, PlayStyle, Side};

#[derive(Debug, Clone, Copy)]
pub enum AttackingPattern {
    CounterAttack,
    FastBreak,
    PossessionBased,
    DirectAttack,
    WingPlay,
    CentralAttack,
    CrossingAttack,
    CombinationPlay,
    OverloadAttack,
    SwitchOfPlay,
    HighPressAttack,
}

pub fn decide_attacking_commands<R: Rng>(
    snapshot: &MatchSnapshot,
    side: Side,
    rng: &mut R,
) -> Vec<MatchCommand> {
    let mut commands = Vec::new();
    let play_style = team_play_style(snapshot, side);
    let goal_diff = team_goal_difference(snapshot, side);
    let minute = snapshot.current_minute;

    // Prioritize play style shifts that match possession patterns.
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

    // If in possession and losing, bias toward attack patterns.
    if goal_diff < 0 && minute >= 55 && rng.gen_bool(0.25) {
        commands.push(match choose_attacking_pattern(snapshot, side, rng) {
            AttackingPattern::CounterAttack | AttackingPattern::FastBreak => MatchCommand::ChangePlayStyle {
                side,
                play_style: PlayStyle::Counter,
            },
            AttackingPattern::PossessionBased | AttackingPattern::CombinationPlay => MatchCommand::ChangePlayStyle {
                side,
                play_style: PlayStyle::Possession,
            },
            AttackingPattern::DirectAttack | AttackingPattern::WingPlay | AttackingPattern::CentralAttack => {
                MatchCommand::ChangePlayStyle {
                    side,
                    play_style: PlayStyle::Attacking,
                }
            }
            AttackingPattern::CrossingAttack | AttackingPattern::OverloadAttack | AttackingPattern::SwitchOfPlay => {
                MatchCommand::ChangePlayStyle {
                    side,
                    play_style: PlayStyle::Attacking,
                }
            }
            AttackingPattern::HighPressAttack => MatchCommand::ChangePlayStyle {
                side,
                play_style: PlayStyle::HighPress,
            },
        });
    }

    // In possession and already winning, keep the ball but avoid overcommitting.
    if goal_diff > 0 && minute >= 70 && rng.gen_bool(0.12) {
        commands.push(MatchCommand::ChangePlayStyle {
            side,
            play_style: PlayStyle::Possession,
        });
    }

    commands
}

fn choose_attacking_pattern<R: Rng>(snapshot: &MatchSnapshot, side: Side, rng: &mut R) -> AttackingPattern {
    let formation = team_formation(snapshot, side);
    if formation.contains('3') && rng.gen_bool(0.45) {
        return AttackingPattern::WingPlay;
    }
    if snapshot.ball_zone == engine::Zone::WideLeft || snapshot.ball_zone == engine::Zone::WideRight {
        return AttackingPattern::CrossingAttack;
    }

    let roll = rng.gen_range(0..100);
    match roll {
        0..=15 => AttackingPattern::CounterAttack,
        16..=30 => AttackingPattern::FastBreak,
        31..=45 => AttackingPattern::PossessionBased,
        46..=55 => AttackingPattern::DirectAttack,
        56..=65 => AttackingPattern::WingPlay,
        66..=75 => AttackingPattern::CentralAttack,
        76..=80 => AttackingPattern::CrossingAttack,
        81..=85 => AttackingPattern::CombinationPlay,
        86..=90 => AttackingPattern::OverloadAttack,
        91..=95 => AttackingPattern::SwitchOfPlay,
        _ => AttackingPattern::HighPressAttack,
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
