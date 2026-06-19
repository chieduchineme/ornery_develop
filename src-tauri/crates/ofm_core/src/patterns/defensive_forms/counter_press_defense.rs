use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "counter_press_defense",
        name: "Counter Press Defense",
        source_md: "Counter_Press_Defense_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.60,
        aggression: 0.95,
        phases: vec![
            DefensivePatternPhase {
                name: "Immediate Press",
                trigger: "Possession lost",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 62.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallCarrier, movement(MovementDirection::Press, 4.0, 0.0, 0.8, Intensity::Explosive), BallAction::Tackle, "Ball carrier", "Nearest player presses immediately"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::BlockLane, 6.0, 0.2, 1.2, Intensity::Sprint), BallAction::None, "Lb outlet", "Block Lb outlet pass"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::CoverChannel, 5.0, 0.2, 1.2, Intensity::Sprint), BallAction::None, "Forward pass", "Block forward pass lane"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 10.0, 0.3, 1.5, Intensity::Sprint), BallAction::None, "Central lane", "Narrow to support press"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::StepUp, 5.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Contain", "Step up to contain"),
                ],
                outcome: "Ball carrier panics or loses possession",
            },
            DefensivePatternPhase {
                name: "Secondary Press",
                trigger: "First presser beaten or ball laid off",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 62.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::Press, 6.0, 0.0, 1.5, Intensity::Sprint), BallAction::Tackle, "Secondary carrier", "Press if first presser beaten"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 5.0, 0.2, 1.5, Intensity::Accelerate), BallAction::None, "Inside lane", "Protect inside lane"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::StepUp, 6.0, 0.5, 2.0, Intensity::Accelerate), BallAction::None, "Back line", "Push up to squeeze space"),
                ],
                outcome: "Ball recovered or opponent forced backward",
            },
            DefensivePatternPhase {
                name: "Recovery if Failed",
                trigger: "Counter-press failed, opponent in space",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 50.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::RecoverShape, 12.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Block shape", "Drop quickly to reset block"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Central lane", "Hold central lane anchor"),
                ],
                outcome: "Block reset before opponent can attack",
            },
        ],
    }
}
