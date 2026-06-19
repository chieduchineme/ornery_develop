use rand::Rng;

use engine::{MatchCommand, MatchSnapshot, PlayStyle, Side};

#[derive(Debug, Clone, Copy)]
pub enum DefensivePattern {
    HighPress,
    MidBlock,
    LowBlock,
    CounterPress,
    ManOrientedPress,
    ZonalDefending,
    HybridDefending,
    Compactness,
    PressingTrap,
    TouchlinePress,
    RestDefense,
    TransitionDefense,
    DefensiveOverload,
    CoverShadow,
    OffsideTrap,
    FunnelDefending,
    SweeperKeeper,
}

pub fn decide_defensive_commands<R: Rng>(
    snapshot: &MatchSnapshot,
    side: Side,
    rng: &mut R,
) -> Vec<MatchCommand> {
    let mut commands = Vec::new();
    let play_style = team_play_style(snapshot, side);
    let goal_diff = team_goal_difference(snapshot, side);
    let minute = snapshot.current_minute;

    if play_style == PlayStyle::HighPress || play_style == PlayStyle::Defensive {
        return commands;
    }

    if goal_diff > 0 && minute >= 60 && rng.gen_bool(0.18) {
        commands.push(MatchCommand::ChangePlayStyle {
            side,
            play_style: PlayStyle::Defensive,
        });
    }

    if goal_diff < 0 && minute >= 70 && rng.gen_bool(0.15) {
        commands.push(MatchCommand::ChangePlayStyle {
            side,
            play_style: PlayStyle::Counter,
        });
    }

    if minute >= 80 && rng.gen_bool(0.12) {
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
