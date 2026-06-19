use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "hybrid_defending",
        name: "Hybrid Defending",
        source_md: "Hybrid_Defending_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Balanced,
        compactness: 0.75,
        aggression: 0.65,
        phases: vec![
            DefensivePatternPhase {
                name: "Zonal Structure",
                trigger: "Opponent in possession midfield",
                tempo_seconds: (0.0, 4.0),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk), BallAction::None, "Central zone", "Hold central zone"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::CoverChannel, 5.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Left channel", "Protect left channel"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::CoverChannel, 5.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Right channel", "Protect right channel"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk), BallAction::None, "Back line", "Compact back four"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Left half-space", "Tuck inside"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Right half-space", "Tuck inside"),
                ],
                outcome: "Zonal block set, channels protected",
            },
            DefensivePatternPhase {
                name: "Local Man Lock",
                trigger: "Ball carrier near playmaker",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::MarkRunner, 5.0, 0.0, 1.8, Intensity::Accelerate), BallAction::None, "Playmaker", "Lock receiving playmaker"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 4.0, 0.0, 1.5, Intensity::Accelerate), BallAction::None, "Lw", "Lock wide attacker"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Press, 6.0, 0.0, 1.5, Intensity::Sprint), BallAction::None, "Lb", "Press fullback"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::CoverChannel, 4.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Space behind jumps", "Cover space vacated by Cm"),
                ],
                outcome: "Local pressure without chaos",
            },
            DefensivePatternPhase {
                name: "Cover Behind",
                trigger: "Press beaten",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 2.5, Intensity::Walk), BallAction::None, "Back line", "Hold depth"),
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::RecoverShape, 8.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Sweeping position", "Position to sweep"),
                ],
                outcome: "Combined aggression and structure",
            },
        ],
    }
}
