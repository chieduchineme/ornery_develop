use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "low_block_defense",
        name: "Low Block Defense",
        source_md: "Low_Block_Defense_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.95,
        aggression: 0.25,
        phases: vec![
            DefensivePatternPhase {
                name: "Block Set",
                trigger: "Opponent in final third, no immediate threat",
                tempo_seconds: (0.0, 5.0),
                block_height_pct: 22.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::RecoverShape, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Right flank", "Drop to compact mid line"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::RecoverShape, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Left flank", "Drop to compact mid line"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::Hold, 0.0, 0.0, 5.0, Intensity::Walk), BallAction::ScreenRestDefense, "Central lane", "Screen pivot pass"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 5.0, Intensity::Walk), BallAction::None, "Box", "Protect six-yard area"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 5.0, 0.0, 5.0, Intensity::Walk), BallAction::None, "Recycling pass", "Screen pivot option"),
                ],
                outcome: "Compact block denies central entry",
            },
            DefensivePatternPhase {
                name: "Cross Defense",
                trigger: "Wide player about to cross",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 22.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::MarkRunner, 6.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Lb crosser", "Track wide crosser"),
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::AttackNearPost, 4.0, 0.0, 1.2, Intensity::Sprint), BallAction::Header, "Near post", "Attack near-post zone"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::AttackFarPost, 5.0, 0.0, 2.0, Intensity::Jog), BallAction::Header, "Far post", "Protect back post"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::RecoverShape, 4.0, 0.0, 2.0, Intensity::Jog), BallAction::Clearance, "Cutback zone", "Guard cutback zone"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Jog), BallAction::BlockShot, "Edge of box", "Block edge shot"),
                ],
                outcome: "Box cleared, second ball contested",
            },
            DefensivePatternPhase {
                name: "Counter Exit",
                trigger: "Defensive clearance won",
                tempo_seconds: (0.0, 4.5),
                block_height_pct: 22.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Press, 20.0, 0.0, 3.0, Intensity::Sprint), BallAction::None, "Ball carrier", "Press carrier after clearance"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Advance, 25.0, 0.5, 4.0, Intensity::Sprint), BallAction::Receive, "Left channel", "Sprint forward as outlet"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Advance, 25.0, 0.5, 4.0, Intensity::Sprint), BallAction::Receive, "Right channel", "Sprint forward as outlet"),
                ],
                outcome: "Exit possession before re-press",
            },
        ],
    }
}
