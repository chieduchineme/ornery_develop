use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "positional_play",
        name: "Positional Play (Juego de Posicion)",
        source_md: "Positional_play_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.20,
        reward: 0.60,
        phases: vec![
            PatternPhase {
                name: "Structured Superiority Build",
                trigger: "Team has ball — players occupy all five pitch lanes to create superiority",
                tempo_seconds: (10.0, 25.0),
                width_m: 65.0,
                depth_m: 45.0,
                instructions: vec![
                    instruction(
                        PlayerRole::BallSideCb,
                        movement(MovementDirection::DriftWide, 8.0, 0.0, 3.0, Intensity::Jog),
                        BallAction::Recycle,
                        "left CB wide position",
                        "Spreads to left of defensive line; creates width at back to stretch opponent",
                    ),
                    instruction(
                        PlayerRole::FarSideCb,
                        movement(MovementDirection::DriftWide, 8.0, 0.0, 3.0, Intensity::Jog),
                        BallAction::None,
                        "right CB wide position",
                        "Mirrors on right; both CBs at maximum width force opponent wingers deep",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Drop, 6.0, 0.0, 2.5, Intensity::Jog),
                        BallAction::Receive,
                        "between CB line — DM pivot",
                        "Drops to form back-three shape; central axis of all build-up circulation",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Advance, 20.0, 2.0, 4.0, Intensity::Jog),
                        BallAction::None,
                        "left midfield lane",
                        "Inverts into left midfield lane; 2-CB + DM holds behind so LB can advance",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Advance, 20.0, 2.0, 4.0, Intensity::Jog),
                        BallAction::None,
                        "right midfield lane",
                        "Advances into right midfield lane; creates 5v4 midfield block with CBs pushed wide",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::DriftInside, 10.0, 1.0, 3.0, Intensity::Jog),
                        BallAction::OneTouchPass,
                        "left half-space between lines",
                        "Occupies left half-space; zone between opposing CM and FB is the goal",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DriftInside, 10.0, 1.0, 3.0, Intensity::Jog),
                        BallAction::None,
                        "right half-space between lines",
                        "Mirrors LCM; all five lanes occupied creating numerical parity everywhere",
                    ),
                ],
                outcome: "Opponent forced into positional discomfort; circulation until gap opens",
            },
            PatternPhase {
                name: "Half-Space Penetration",
                trigger: "LB/RB receives in advanced position — line-breaking pass into half-space",
                tempo_seconds: (2.0, 4.0),
                width_m: 48.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::ThroughBall,
                        "LCM in left half-space",
                        "Line-breaking pass into LCM in half-space; timing waits for CM's movement away",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::DiagonalRun, 12.0, 0.5, 2.0, Intensity::Accelerate),
                        BallAction::Receive,
                        "left half-space — between lines",
                        "Receives between lines on back foot; turns and plays forward or shoots",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::BlindsideRun, 16.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "behind defensive line",
                        "Blindside run exploiting LCM's ball reception as reference; stays onside",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftInside, 14.0, 1.0, 2.5, Intensity::Accelerate),
                        BallAction::None,
                        "right half-space — third-man option",
                        "Inverts to become third-man option if ST is tracked; arrives at edge of box",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::ScreenRestDefense,
                        "top of D — rest defense anchor",
                        "Never joins attack; structural guarantee against transition",
                    ),
                ],
                outcome: "LCM receives between lines and plays ST through or shoots; Juego de Posición achieved",
            },
        ],
    }
}
