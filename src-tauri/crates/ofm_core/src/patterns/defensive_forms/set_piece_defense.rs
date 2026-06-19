use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "set_piece_defense",
        name: "Set Piece Defense",
        source_md: "Set_Piece_Defense_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Balanced,
        compactness: 0.95,
        aggression: 0.70,
        phases: vec![
            DefensivePatternPhase {
                name: "Corner Assignment",
                trigger: "Opponent corner kick",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 10.0,
                instructions: vec![
                    def_instruction(PlayerRole::BallSideCb, movement(MovementDirection::AttackNearPost, 3.0, 0.0, 1.2, Intensity::Sprint), BallAction::Header, "Near post zone", "Attack near post zone"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::MarkRunner, 2.0, 0.0, 1.5, Intensity::Jog), BallAction::Header, "Central attacker", "Mark central attacker"),
                    def_instruction(PlayerRole::FarSideCb, movement(MovementDirection::AttackFarPost, 4.0, 0.0, 1.8, Intensity::Jog), BallAction::Header, "Far post", "Attack far post"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 3.0, 0.0, 1.2, Intensity::Jog), BallAction::None, "Near-post runner", "Mark near-post runner"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 2.5, Intensity::Walk), BallAction::BlockShot, "Edge of box", "Guard edge of box"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::RecoverShape, 5.0, 0.0, 3.0, Intensity::Jog), BallAction::Receive, "Outlet position", "Hold as counter outlet"),
                ],
                outcome: "All zones assigned, delivery covered",
            },
            DefensivePatternPhase {
                name: "First Contact",
                trigger: "Corner delivered",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 10.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::Hold, 0.0, 0.0, 1.5, Intensity::Walk), BallAction::PunchClear, "Six-yard area", "Dominate if in range"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 2.0, 0.0, 1.5, Intensity::Explosive), BallAction::Header, "First ball", "Win first aerial ball"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::RecoverShape, 4.0, 0.0, 1.5, Intensity::Jog), BallAction::BlockShot, "Rebound zone", "Contest rebound"),
                ],
                outcome: "Clearance, catch, or punch",
            },
            DefensivePatternPhase {
                name: "Second-Phase",
                trigger: "First ball cleared or dropped",
                tempo_seconds: (0.0, 4.0),
                block_height_pct: 10.0,
                instructions: vec![
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::RecoverShape, 6.0, 0.0, 1.5, Intensity::Sprint), BallAction::Clearance, "Second ball", "Contest second ball"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::Advance, 20.0, 0.5, 3.5, Intensity::Sprint), BallAction::Receive, "Counter position", "Launch counter"),
                ],
                outcome: "Exit before re-delivery",
            },
        ],
    }
}
