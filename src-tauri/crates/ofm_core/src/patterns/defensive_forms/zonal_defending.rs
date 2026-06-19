use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "zonal_defending",
        name: "Zonal Defending",
        source_md: "Zonal_Defending_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.80,
        aggression: 0.40,
        phases: vec![
            DefensivePatternPhase {
                name: "Block Shift",
                trigger: "Ball circulates to one side",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 38.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::SqueezeUp, 8.0, 0.0, 1.5, Intensity::Jog), BallAction::None, "Ball side", "Block slides to ball side"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::Press, 8.0, 0.0, 1.8, Intensity::Accelerate), BallAction::None, "Ball carrier", "Near Cm presses ball"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::CoverChannel, 5.0, 0.2, 1.8, Intensity::Jog), BallAction::None, "Inside lane", "Cover inside from press"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 6.0, 0.2, 1.8, Intensity::Jog), BallAction::None, "Switch lane", "Protect far switch"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::SqueezeUp, 6.0, 0.3, 1.5, Intensity::Jog), BallAction::None, "Back line", "Back four shifts together"),
                ],
                outcome: "Ball side overloaded, ball carrier isolated",
            },
            DefensivePatternPhase {
                name: "Runner Handoff",
                trigger: "Runner enters back four zone",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 38.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Zone boundary", "Pass runner to Rb"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 6.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Runner", "Pick up runner entering zone"),
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::CoverChannel, 5.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Channel", "Shift to cover vacated channel"),
                ],
                outcome: "Runners tracked without pulling defenders out of shape",
            },
            DefensivePatternPhase {
                name: "Switch Protection",
                trigger: "Ball switched to far side",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 38.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::TrackBack, 12.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Switch receiver", "Sprint to far side receiver"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::RecoverShape, 8.0, 0.3, 2.0, Intensity::Accelerate), BallAction::None, "Wide zone", "Cover wide on far side"),
                ],
                outcome: "Switch neutralized",
            },
        ],
    }
}
