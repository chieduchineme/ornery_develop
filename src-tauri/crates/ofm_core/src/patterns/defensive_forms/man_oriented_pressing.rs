use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "man_oriented_pressing",
        name: "Man Oriented Pressing",
        source_md: "Man_Oriented_Pressing_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.55,
        aggression: 0.88,
        phases: vec![
            DefensivePatternPhase {
                name: "Assign Press",
                trigger: "Opponent starts build-up",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 62.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Press, 14.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Cb", "Lock Cb tightly"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::MarkRunner, 10.0, 0.0, 2.0, Intensity::Sprint), BallAction::None, "Rb", "Lock Rb"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::MarkRunner, 10.0, 0.0, 2.0, Intensity::Sprint), BallAction::None, "Lb", "Lock Lb"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::MarkRunner, 8.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Dm", "Lock Dm"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::MarkRunner, 8.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Am", "Lock Am"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::CoverChannel, 5.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Cover zone", "Spare Cb provides cover"),
                ],
                outcome: "Every opponent locked, no easy pass",
            },
            DefensivePatternPhase {
                name: "Individual Duels",
                trigger: "Ball circulates under pressure",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 62.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Press, 6.0, 0.0, 1.8, Intensity::Sprint), BallAction::Tackle, "Cb", "Press Cb while screening Dm"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 4.0, 0.0, 1.2, Intensity::Explosive), BallAction::Tackle, "Rb", "Close Rb facing backward"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::MarkRunner, 4.0, 0.2, 1.2, Intensity::Explosive), BallAction::Tackle, "Dm", "Step hard onto Dm"),
                ],
                outcome: "Ball forced long or rushed",
            },
            DefensivePatternPhase {
                name: "Cover if Beaten",
                trigger: "Individual duel lost",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 55.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::CoverChannel, 8.0, 0.0, 2.0, Intensity::Sprint), BallAction::None, "Space behind duel", "Cover behind lost duel"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::RecoverShape, 10.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Central lane", "Drop to cover"),
                ],
                outcome: "Defensive balance maintained",
            },
        ],
    }
}
