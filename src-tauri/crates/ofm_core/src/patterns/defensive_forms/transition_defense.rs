use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "transition_defense",
        name: "Transition Defense",
        source_md: "Transition_Defense_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Counter,
        compactness: 0.70,
        aggression: 0.60,
        phases: vec![
            DefensivePatternPhase {
                name: "Immediate Decision",
                trigger: "Possession lost in transition",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 48.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallCarrier, movement(MovementDirection::Press, 5.0, 0.0, 1.0, Intensity::Explosive), BallAction::None, "Ball carrier", "Nearest player presses carrier"),
                    def_instruction(PlayerRole::DecoyRunner, movement(MovementDirection::BlockLane, 6.0, 0.2, 1.2, Intensity::Sprint), BallAction::None, "Forward lanes", "Close forward lanes"),
                ],
                outcome: "Opponent slowed or pressured",
            },
            DefensivePatternPhase {
                name: "Delay if No Pressure",
                trigger: "Counter-press cannot be executed",
                tempo_seconds: (0.0, 4.5),
                block_height_pct: 40.0,
                instructions: vec![
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::RecoverShape, 12.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Block shape", "Drop and narrow"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::RecoverShape, 20.0, 0.0, 3.5, Intensity::Sprint), BallAction::None, "Block shape", "Sprint back to block"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::RecoverShape, 20.0, 0.0, 3.5, Intensity::Sprint), BallAction::None, "Block shape", "Sprint back to block"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::TrackBack, 25.0, 0.0, 4.0, Intensity::Sprint), BallAction::None, "Central lane", "Track back centrally"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::TrackBack, 25.0, 0.0, 4.0, Intensity::Sprint), BallAction::None, "Central lane", "Track back centrally"),
                ],
                outcome: "Block re-organized",
            },
            DefensivePatternPhase {
                name: "Hold Shape",
                trigger: "Block reset after recovery",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 40.0,
                instructions: vec![
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::ScreenRestDefense, "Counter lane", "Hold central counter lane"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Back line", "Hold depth"),
                ],
                outcome: "Open-field attack prevented",
            },
        ],
    }
}
