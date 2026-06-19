use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "compactness_defense",
        name: "Compactness Defense",
        source_md: "Compactness_Defense_patterns.md",
        base_formation: "4-2-3-1",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.95,
        aggression: 0.30,
        phases: vec![
            DefensivePatternPhase {
                name: "Vertical Compression",
                trigger: "Opponent has ball in midfield",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Am, movement(MovementDirection::Tuck, 6.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Central lane", "Narrow behind striker"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 10.0, 0.0, 2.5, Intensity::Jog), BallAction::None, "Left half-space", "Tuck inside to compress"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 10.0, 0.0, 2.5, Intensity::Jog), BallAction::None, "Right half-space", "Tuck inside to compress"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk), BallAction::None, "Double pivot lane", "Hold behind midfield"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::StepUp, 5.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Back line", "Step up to squeeze space"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 4.0, 0.0, 4.0, Intensity::Walk), BallAction::ScreenRestDefense, "Dm", "Screen Dm from passing option"),
                ],
                outcome: "Block compressed vertically, no space between lines",
            },
            DefensivePatternPhase {
                name: "Horizontal Shift",
                trigger: "Ball moves to one side",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::SqueezeUp, 8.0, 0.0, 1.5, Intensity::Jog), BallAction::None, "Ball side", "Whole block shifts to ball"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 6.0, 0.0, 1.5, Intensity::Walk), BallAction::None, "Far side lane", "Stay inside for balance"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Back line", "Communicate and hold line"),
                ],
                outcome: "Central passing lane crowded",
            },
            DefensivePatternPhase {
                name: "Central Block",
                trigger: "Ball played into central pocket",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::MarkRunner, 5.0, 0.0, 1.5, Intensity::Sprint), BallAction::Tackle, "Receiving player", "Jump on receiver immediately"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 4.0, 0.2, 1.5, Intensity::Accelerate), BallAction::None, "Inside support", "Cover inside support option"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::CoverChannel, 5.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Space behind Cm", "Cover if Cm is beaten"),
                ],
                outcome: "Combination broken",
            },
        ],
    }
}
