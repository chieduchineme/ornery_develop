use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "high_press_defense",
        name: "High Press Defense",
        source_md: "High_Press_Defense_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.70,
        aggression: 0.90,
        phases: vec![
            DefensivePatternPhase {
                name: "First-Line Trigger",
                trigger: "Opponent CB receives in build-up",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 68.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Press, 14.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Cb", "Curved run to block pass to Dm"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 12.0, 0.3, 2.2, Intensity::Sprint), BallAction::None, "Rb", "Press Rb from outside shoulder"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::SqueezeUp, 10.0, 0.5, 2.0, Intensity::Accelerate), BallAction::None, "Dm", "Jump to Dm to deny pivot"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::Tuck, 6.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Central lane", "Narrow to protect inside"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::StepUp, 8.0, 0.8, 2.5, Intensity::Accelerate), BallAction::None, "Back line", "Compress space behind press"),
                ],
                outcome: "Ball carrier is isolated near touchline or forced long",
            },
            DefensivePatternPhase {
                name: "Lane Control",
                trigger: "Ball played to Rb or Lb",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 72.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 8.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Dm", "Body angle blocks pass to Dm"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::MarkRunner, 6.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Lb pass to Rcm", "Shadow pass to Rcm"),
                ],
                outcome: "Pivot is hidden, progression forced wide",
            },
            DefensivePatternPhase {
                name: "Trap Execution",
                trigger: "Ball played to wide player on touchline",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 72.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 4.0, 0.0, 1.5, Intensity::Explosive), BallAction::Tackle, "Rb", "Close from outside shoulder"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 5.0, 0.2, 1.5, Intensity::Sprint), BallAction::Intercept, "Inside pass", "Cut inside option"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::MarkRunner, 6.0, 0.4, 2.0, Intensity::Sprint), BallAction::None, "Rw", "Pin wide attacker"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 4.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Return pass", "Seal return to Cb"),
                ],
                outcome: "Turnover or forced long ball",
            },
        ],
    }
}
