use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "rotational_attack",
        name: "Rotational Attack",
        source_md: "Rotational-Attack_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.45,
        reward: 0.72,
        phases: vec![
            PatternPhase {
                name: "Winger-Fullback-Midfielder Rotation",
                trigger: "Ball in wide area — continuous zone exchange to disrupt man-marking",
                tempo_seconds: (5.0, 10.0),
                width_m: 55.0,
                depth_m: 35.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftInside, 14.0, 0.0, 3.0, Intensity::Jog),
                        BallAction::None,
                        "left half-space — vacate wing",
                        "Rotates inward vacating the left wing zone; man-marker follows inside",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::DriftWide, 12.0, 1.0, 2.5, Intensity::Jog),
                        BallAction::None,
                        "vacated left wing zone",
                        "Occupies zone LW vacated; defender now guarding empty space or loses LW inside",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Rotate, 10.0, 1.5, 2.5, Intensity::Accelerate),
                        BallAction::Receive,
                        "LB's vacated halfspace position",
                        "Drops into space LB left by advancing; receives from DM in vacated zone",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "LCM in rotation",
                        "Circulation hub; plays into whichever rotation creates the free man",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Rotate, 8.0, 2.0, 2.5, Intensity::Jog),
                        BallAction::None,
                        "left-of-center after rotation",
                        "Drifts toward the rotated side; becomes additional passing option after rotation",
                    ),
                ],
                outcome: "Rotation creates momentary free man; exploit with vertical pass before defense readjusts",
            },
            PatternPhase {
                name: "Striker-Midfielder Role Exchange",
                trigger: "ST drops deep — CM makes high run into vacated striker zone",
                tempo_seconds: (3.0, 6.0),
                width_m: 40.0,
                depth_m: 30.0,
                instructions: vec![
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 18.0, 0.0, 3.0, Intensity::Jog),
                        BallAction::Receive,
                        "CM position — between lines",
                        "Drops into vacated CM zone; CB must choose whether to follow",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::SprintForward, 22.0, 1.0, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "vacated ST zone — behind CB line",
                        "Sprints into space ST vacated; arrives as effective striker",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Hold, 0.0, 2.0, 1.0, Intensity::Walk),
                        BallAction::ThroughBall,
                        "LCM running behind defense",
                        "Plays through ball into LCM's run; CB caught in transition between following and holding",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 5.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::None,
                        "left wide — stretch FB",
                        "Stays wide throughout; prevents FB from covering central rotation zone",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftInside, 10.0, 2.0, 2.5, Intensity::Accelerate),
                        BallAction::None,
                        "right half-space — secondary option",
                        "Inverts toward goal as secondary option if LCM's run is covered",
                    ),
                ],
                outcome: "LCM receives behind defense as effective striker; or RW arrives in half-space for shot",
            },
        ],
    }
}
