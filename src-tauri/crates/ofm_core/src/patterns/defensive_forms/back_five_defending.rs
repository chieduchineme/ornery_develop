use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "back_five_defending",
        name: "Back Five Defending",
        source_md: "Back_Five_Defending_patterns.md",
        base_formation: "5-3-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.90,
        aggression: 0.40,
        phases: vec![
            DefensivePatternPhase {
                name: "Width Protection",
                trigger: "Opponent prepares to cross from wide",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 20.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lwb, movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Sprint), BallAction::Tackle, "Crosser", "Jump to wide crosser"),
                    def_instruction(PlayerRole::Lcb, movement(MovementDirection::CoverChannel, 6.0, 0.3, 2.0, Intensity::Accelerate), BallAction::None, "Channel", "Cover behind Lwb"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk), BallAction::None, "Six-yard box", "Protect central area"),
                    def_instruction(PlayerRole::Rwb, movement(MovementDirection::RecoverShape, 8.0, 0.5, 2.5, Intensity::Jog), BallAction::None, "Far post", "Recover to far post area"),
                    def_instruction(PlayerRole::Rcb, movement(MovementDirection::Tuck, 4.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Central zone", "Narrow centrally for cross"),
                ],
                outcome: "Width covered, box protected",
            },
            DefensivePatternPhase {
                name: "Central Box",
                trigger: "Cross delivered into box",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 20.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::AttackNearPost, 4.0, 0.0, 1.2, Intensity::Sprint), BallAction::Header, "Near post", "Attack near-post zone"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::MarkRunner, 3.0, 0.0, 2.0, Intensity::Jog), BallAction::Header, "Central striker", "Defend central attacker"),
                    def_instruction(PlayerRole::Rcb, movement(MovementDirection::AttackFarPost, 5.0, 0.0, 2.0, Intensity::Jog), BallAction::Header, "Far post", "Protect far post"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::RecoverShape, 6.0, 0.0, 2.5, Intensity::Jog), BallAction::BlockShot, "Cutback zone", "Track cutback"),
                ],
                outcome: "Cross defended without exposing central defenders",
            },
            DefensivePatternPhase {
                name: "Counter Exit via WB",
                trigger: "Defensive clearance won",
                tempo_seconds: (0.0, 5.0),
                block_height_pct: 20.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lwb, movement(MovementDirection::Advance, 30.0, 0.0, 4.5, Intensity::Sprint), BallAction::Receive, "Left channel", "Sprint forward as outlet"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 2.0, 0.0, 4.5, Intensity::Walk), BallAction::Receive, "Target position", "Hold as second outlet"),
                ],
                outcome: "Wing-back outlet launches transition",
            },
        ],
    }
}
