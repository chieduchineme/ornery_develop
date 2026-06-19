use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "high_press_attack",
        name: "High Press Attack",
        source_md: "High_Press_Attack.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        risk: 0.60,
        reward: 0.85,
        phases: vec![
            PatternPhase {
                name: "Pressing Trap to Goal",
                trigger: "Opponent CB in possession — coordinated press trap sprung",
                tempo_seconds: (3.0, 8.0),
                width_m: 35.0,
                depth_m: 18.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::Press, 15.0, 0.0, 2.0, Intensity::Sprint),
                        BallAction::PressingRecovery,
                        "opponent CB with ball",
                        "Presses CB from front; angles run to cut off pass to GK, forcing play inside",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Accelerate),
                        BallAction::None,
                        "opponent DM — cover shadow",
                        "Press in cover shadow of DM; stops easy outlet from CB to DM",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Press, 12.0, 0.5, 2.0, Intensity::Sprint),
                        BallAction::PressingRecovery,
                        "opponent midfielders",
                        "Aggressive press to block escape route; tries to force backward pass",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftInside, 8.0, 0.0, 2.0, Intensity::Jog),
                        BallAction::None,
                        "opposite side — cover shadow",
                        "Compact inward to block through-ball options; holds shape not chases",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 10.0, 1.0, 2.5, Intensity::Accelerate),
                        BallAction::PressingRecovery,
                        "interception zone — forced pass lane",
                        "Intercepts forced pass as RW press triggers panic; ball-steal position",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Advance, 12.0, 1.5, 2.0, Intensity::Accelerate),
                        BallAction::None,
                        "recovered ball — immediate forward outlet",
                        "Ready to receive won ball and play immediately vertical",
                    ),
                ],
                outcome: "Ball won high; immediate vertical attack within 2s of recovery",
            },
            PatternPhase {
                name: "Won Ball — Instant Strike",
                trigger: "High turnover won within 20m of opponent goal",
                tempo_seconds: (2.0, 5.0),
                width_m: 28.0,
                depth_m: 20.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 0.5, Intensity::Walk),
                        BallAction::ThroughBall,
                        "ST in behind",
                        "Wins ball and plays immediately; no hesitation — vertical pass in 1 touch",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::BlindsideRun, 12.0, 0.0, 1.8, Intensity::Explosive),
                        BallAction::Shoot,
                        "goal — near corner",
                        "Already positioned for blindside run before press; 1 touch finish",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::Advance, 8.0, 0.5, 1.5, Intensity::Sprint),
                        BallAction::None,
                        "support run right of goal",
                        "Support run in case ST needs to lay off; occupies nearest defender",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DiagonalRun, 10.0, 0.5, 2.0, Intensity::Sprint),
                        BallAction::Shoot,
                        "rebound zone",
                        "Crashes into rebound position; any save or block falls into this area",
                    ),
                ],
                outcome: "ST finishes in 1-2 touches within 5s of pressing turnover",
            },
        ],
    }
}
