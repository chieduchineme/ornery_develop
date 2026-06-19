use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "rest_defense",
        name: "Rest Defense",
        source_md: "Rest_Defense_patterns.md",
        base_formation: "4-3-1-2",
        preferred_play_style: PlayStyle::Possession,
        compactness: 0.65,
        aggression: 0.30,
        phases: vec![
            DefensivePatternPhase {
                name: "Behind-Ball Structure",
                trigger: "Team in possession in final third",
                tempo_seconds: (0.0, 5.0),
                block_height_pct: 45.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::Hold, 5.0, 0.0, 5.0, Intensity::Jog), BallAction::None, "Central split", "Split centrally"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::Hold, 5.0, 0.0, 5.0, Intensity::Jog), BallAction::None, "Central split", "Split centrally"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 5.0, Intensity::Walk), BallAction::ScreenRestDefense, "Counter lane", "Hold central counter lane"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::Tuck, 8.0, 0.0, 4.0, Intensity::Jog), BallAction::None, "Third cover", "Tuck inside as third cover"),
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::StepUp, 10.0, 0.0, 5.0, Intensity::Walk), BallAction::None, "High sweeper", "Position high as sweeper"),
                ],
                outcome: "Secure structure behind attack",
            },
            DefensivePatternPhase {
                name: "Counter-Press Trigger",
                trigger: "Possession lost",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 45.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallCarrier, movement(MovementDirection::Press, 4.0, 0.0, 1.0, Intensity::Explosive), BallAction::Tackle, "Ball carrier", "Press immediately on loss"),
                    def_instruction(PlayerRole::DecoyRunner, movement(MovementDirection::BlockLane, 5.0, 0.2, 1.2, Intensity::Sprint), BallAction::None, "Forward pass", "Block forward pass"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::CoverChannel, 5.0, 0.3, 1.5, Intensity::Sprint), BallAction::None, "Central lane", "Cover central lane"),
                ],
                outcome: "Ball recovered or opponent delayed",
            },
            DefensivePatternPhase {
                name: "Recovery Shape",
                trigger: "Counter-press did not win ball",
                tempo_seconds: (0.0, 4.0),
                block_height_pct: 40.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::RecoverShape, 20.0, 0.0, 3.5, Intensity::Sprint), BallAction::None, "Central lane", "Recover central lanes first"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::RecoverShape, 20.0, 0.0, 3.5, Intensity::Sprint), BallAction::None, "Central lane", "Recover central lanes first"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Back line", "Hold depth"),
                ],
                outcome: "Block set before attack develops",
            },
        ],
    }
}
