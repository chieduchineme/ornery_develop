use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "offside_trap",
        name: "Offside Trap",
        source_md: "Offside_Trap_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.80,
        aggression: 0.50,
        phases: vec![
            DefensivePatternPhase {
                name: "Line Coordination",
                trigger: "Opponent prepares through ball",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 52.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Passer", "Press passer"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::CallOffsideTrap, 0.0, 0.5, 0.5, Intensity::Walk), BallAction::None, "Back line", "Call step to all defenders"),
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::StepUp, 6.0, 0.5, 1.0, Intensity::Accelerate), BallAction::None, "Line", "Step up together"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::StepUp, 6.0, 0.5, 1.0, Intensity::Accelerate), BallAction::None, "Line", "Step up together"),
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::StepUp, 12.0, 0.0, 2.5, Intensity::Jog), BallAction::None, "High position", "Position high to sweep"),
                ],
                outcome: "Coordinated step ready to trap",
            },
            DefensivePatternPhase {
                name: "Trap Execution",
                trigger: "Passer releases ball",
                tempo_seconds: (0.5, 1.5),
                block_height_pct: 52.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::StepUp, 6.0, 0.5, 0.8, Intensity::Sprint), BallAction::None, "Offside line", "Step simultaneously"),
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::StepUp, 6.0, 0.5, 0.8, Intensity::Sprint), BallAction::None, "Offside line", "Step simultaneously"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::StepUp, 6.0, 0.5, 0.8, Intensity::Sprint), BallAction::None, "Offside line", "Step simultaneously"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::StepUp, 6.0, 0.5, 0.8, Intensity::Sprint), BallAction::None, "Offside line", "Step simultaneously"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 0.0, 1.0, 0.5, Intensity::Walk), BallAction::None, "Runner position", "Runner caught beyond last line"),
                ],
                outcome: "Offside flagged or pass delayed",
            },
            DefensivePatternPhase {
                name: "Recovery if Beaten",
                trigger: "Runner beats trap",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 30.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::RecoverShape, 20.0, 0.0, 3.0, Intensity::Sprint), BallAction::Clearance, "Through ball", "Sweep forward to clear"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::RecoverShape, 8.0, 0.0, 2.0, Intensity::Sprint), BallAction::None, "Cover", "Drop to cover GK"),
                ],
                outcome: "GK clears before striker arrives",
            },
        ],
    }
}
