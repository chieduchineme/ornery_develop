use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "isolation_attack",
        name: "Isolation Attack",
        source_md: "Isolation_attack_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.50,
        reward: 0.78,
        phases: vec![
            PatternPhase {
                name: "Half-Space Isolation Creation",
                trigger: "High-value attacker receives in half-space — surrounding players clear space",
                tempo_seconds: (3.0, 6.0),
                width_m: 30.0,
                depth_m: 22.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 12.0, 0.0, 2.0, Intensity::Jog),
                        BallAction::None,
                        "left touchline — vacate half-space",
                        "Deliberately moves wide to pull LB with them; clears half-space for isolator",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::DiagonalRun, 10.0, 0.0, 2.0, Intensity::Jog),
                        BallAction::None,
                        "right channel — pull opposite CB",
                        "Diagonal movement away to stretch CB coverage; empties central zone",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::DriftInside, 14.0, 0.5, 2.5, Intensity::Accelerate),
                        BallAction::Receive,
                        "left half-space — isolated 1v1 zone",
                        "Receives in vacated half-space; defender forced to engage alone without cover",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::ScreenRestDefense,
                        "top of D — rest defense",
                        "Holds rest-defense position; does not join attack to maintain counter shield",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::AttackCutbackZone, 16.0, 2.0, 2.5, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot",
                        "Secondary runner into penalty spot; arrives if isolator drives in and cuts back",
                    ),
                ],
                outcome: "LCM in clean 1v1 in half-space; drives at goal, shoots or plays RCM",
            },
            PatternPhase {
                name: "Central Isolation — Top of Box",
                trigger: "Ball played to isolator at top of box — all others hold width",
                tempo_seconds: (2.5, 5.0),
                width_m: 35.0,
                depth_m: 18.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk),
                        BallAction::Dribble,
                        "defender at top of box",
                        "Receives on half-turn; one-touch to set and drives at defender with pace",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 8.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::None,
                        "left wide — stretch space",
                        "Stays wide to prevent defensive narrowing; creates passing safety valve",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 8.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::None,
                        "right wide — stretch space",
                        "Mirrors LW on right; both wingers pin FBs and keep penalty box open",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Hold, 3.0, 1.0, 2.0, Intensity::Walk),
                        BallAction::None,
                        "penalty area — loose ball",
                        "Lurks at edge of box; collects if isolator's shot is blocked or parried",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Hold, 5.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::None,
                        "top-right of box — secondary shooter",
                        "Holds position; available as passing option if isolator is blocked",
                    ),
                ],
                outcome: "Isolator drives at goal and shoots or squares; wide players prevent double-team",
            },
        ],
    }
}
