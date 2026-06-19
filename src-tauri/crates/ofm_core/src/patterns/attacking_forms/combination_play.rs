use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "combination_play",
        name: "Combination Play",
        source_md: "Combination-Play_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.35,
        reward: 0.68,
        phases: vec![
            PatternPhase {
                name: "One-Two Wall Pass",
                trigger: "Pressure on CM — wall-pass to bypass press",
                tempo_seconds: (1.5, 3.0),
                width_m: 30.0,
                depth_m: 20.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 8.0, 0.0, 1.0, Intensity::Accelerate),
                        BallAction::OneTouchPass,
                        "DM — wall pass initiation",
                        "Plays short pass and immediately sprints beyond pressing opponent",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 0.5, Intensity::Walk),
                        BallAction::BouncePass,
                        "LCM running beyond",
                        "First-touch return pass into space behind the press; 1-touch only",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::BlindsideRun, 12.0, 0.5, 1.5, Intensity::Sprint),
                        BallAction::Receive,
                        "beyond pressing player",
                        "Receives in stride behind the pressing opponent; line broken",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 8.0, 0.0, 2.0, Intensity::Jog),
                        BallAction::None,
                        "pocket between lines",
                        "Drops to draw CBs forward; creates depth for LCM's run",
                    ),
                ],
                outcome: "Line broken via wall pass; LCM now in space to drive at goal or play ST",
            },
            PatternPhase {
                name: "Triangle Rotation Through Press",
                trigger: "3-player triangle under pressure — fluid 1-2 touch rotations",
                tempo_seconds: (3.0, 6.0),
                width_m: 25.0,
                depth_m: 18.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "LCM left",
                        "Plays short and rotates; fills LCM vacated space",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Rotate, 5.0, 0.5, 1.0, Intensity::Jog),
                        BallAction::OneTouchPass,
                        "ST dropping",
                        "Receives, immediately plays ST; moves to new triangle vertex",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 6.0, 0.5, 1.5, Intensity::Jog),
                        BallAction::BouncePass,
                        "DM who rotated into space",
                        "One-touch bounce to DM who has rotated into vacated zone",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::BlindsideRun, 18.0, 2.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "gap created by rotation — final third entry",
                        "Third-man runner exploits gap opened by triangle rotations",
                    ),
                ],
                outcome: "Triangle rotations create confusion; third-man exploits gap into final third",
            },
            PatternPhase {
                name: "Layered Combination Into Box",
                trigger: "Ball near top of penalty area — layered 1-2 combinations to penetrate box",
                tempo_seconds: (2.0, 4.0),
                width_m: 28.0,
                depth_m: 20.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftInside, 10.0, 0.0, 1.5, Intensity::Accelerate),
                        BallAction::OneTouchPass,
                        "ST lateral pass",
                        "Inverted inside; plays ST and continues diagonal run into box",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Hold, 2.0, 0.5, 0.5, Intensity::Walk),
                        BallAction::BouncePass,
                        "LCM behind",
                        "Touch and bounce to LCM; draws one CB to create gap",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 10.0, 1.0, 1.5, Intensity::Accelerate),
                        BallAction::ThroughBall,
                        "LW running into box",
                        "Through ball into LW's path who continued run; 2-touch sequence",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DiagonalRun, 14.0, 0.5, 2.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "top left corner of goal",
                        "Collects through ball in stride; shot or squared to penalty spot",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::AttackCutbackZone, 16.0, 1.5, 2.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "penalty spot",
                        "Arrives penalty spot as third man; receives squared pass or rebound",
                    ),
                ],
                outcome: "LW shoots or squares to RCM; combination bypassed defense via 3-touch sequence",
            },
        ],
    }
}
