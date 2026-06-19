use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "overload_attack",
        name: "Overload Attack",
        source_md: "Overload_attack_pattern.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.50,
        reward: 0.72,
        phases: vec![
            PatternPhase {
                name: "Wide Overload 3v2",
                trigger: "Ball on left flank — LW + LB + LCM create 3v2 vs FB and wide mid",
                tempo_seconds: (4.0, 8.0),
                width_m: 60.0,
                depth_m: 32.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 4.0, 0.0, 1.5, Intensity::Jog),
                        BallAction::Dribble,
                        "touchline — attract press",
                        "Invites 1v1 engagement from FB to create timing gap for LB overlap",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Overlap, 22.0, 0.8, 3.2, Intensity::Sprint),
                        BallAction::None,
                        "outside channel beyond LW",
                        "First overloading runner; commits defender to choosing whom to track",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 18.0, 1.2, 3.0, Intensity::Accelerate),
                        BallAction::None,
                        "inside channel — third overloader",
                        "Third body floods inside channel; defense now outnumbered on left side",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 0.0, 2.0, 1.5, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "free man — LB or LCM",
                        "Identifies which defender tracked whom; plays to free man",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 12.0, 3.0, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Near-post run ensures at least one body for any cross or cutback",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::CounterBalance, 8.0, 0.0, 5.0, Intensity::Jog),
                        BallAction::ScreenRestDefense,
                        "right side — shape balance",
                        "Holds right to prevent defensive overcommit and maintain structural balance",
                    ),
                ],
                outcome: "Free man on left side crosses or drives into box; ST occupies near post",
            },
            PatternPhase {
                name: "Central Overload Crash",
                trigger: "Ball in central midfield — 4 players crash into box simultaneously",
                tempo_seconds: (2.5, 5.0),
                width_m: 36.0,
                depth_m: 22.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Advance, 10.0, 0.0, 1.5, Intensity::Accelerate),
                        BallAction::ThroughBall,
                        "central gap between CBs",
                        "Plays through ball into space between CBs; triggers simultaneous box runs",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::SprintForward, 14.0, 0.0, 2.0, Intensity::Explosive),
                        BallAction::Receive,
                        "between CBs — on through ball",
                        "First runner onto through ball; occupies both CBs in race to ball",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DiagonalRun, 20.0, 0.3, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "left of goal — box crash",
                        "Diagonal run into left zone of penalty area; arrives as third attacker",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DiagonalRun, 20.0, 0.3, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "right of goal — box crash",
                        "Mirrors LW on right; four attackers vs four defenders — numerical equality but positional overload",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 16.0, 0.5, 2.5, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot",
                        "Fifth wave; arrives at penalty spot for second ball or ST layoff",
                    ),
                ],
                outcome: "Through ball to ST in channel; 4-body box overload creates positional chaos and goal chance",
            },
        ],
    }
}
