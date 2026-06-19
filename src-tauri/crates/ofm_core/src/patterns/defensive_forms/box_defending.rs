use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "box_defending",
        name: "Box Defending",
        source_md: "Box_Defending_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.95,
        aggression: 0.60,
        phases: vec![
            DefensivePatternPhase {
                name: "Cross Preparation",
                trigger: "Wide player preparing to cross",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 15.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::AttackNearPost, 3.0, 0.0, 1.0, Intensity::Accelerate), BallAction::Header, "Near post", "Step to near-post zone"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::AttackFarPost, 4.0, 0.0, 1.5, Intensity::Jog), BallAction::Header, "Far post", "Protect back post"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::RecoverShape, 5.0, 0.0, 2.0, Intensity::Jog), BallAction::BlockShot, "Cutback zone", "Guard cutback zone"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::MarkRunner, 5.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Cutback runner", "Track cutback runner"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 4.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Lw", "Track wide attacker"),
                ],
                outcome: "Box organized before cross arrives",
            },
            DefensivePatternPhase {
                name: "Aerial Contest",
                trigger: "Cross delivered",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 15.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk), BallAction::PunchClear, "Six-yard area", "Dominate six-yard area"),
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::AttackNearPost, 3.0, 0.0, 1.0, Intensity::Explosive), BallAction::Clearance, "Near post", "Attack cross at near post"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::Hold, 3.0, 0.0, 2.0, Intensity::Jog), BallAction::BlockShot, "Edge of box", "Guard edge for rebound"),
                ],
                outcome: "Clearance, punch, or catch",
            },
            DefensivePatternPhase {
                name: "Second Ball",
                trigger: "First clearance made",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 15.0,
                instructions: vec![
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::RecoverShape, 5.0, 0.0, 1.5, Intensity::Sprint), BallAction::Clearance, "Rebound zone", "Attack second ball"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::RecoverShape, 8.0, 0.3, 2.5, Intensity::Jog), BallAction::Receive, "Exit lane", "Exit with pass if won"),
                ],
                outcome: "Possession secured or area cleared",
            },
        ],
    }
}
