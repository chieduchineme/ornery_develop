use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "defensive_overload",
        name: "Defensive Overload",
        source_md: "Defensive_Overload_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.75,
        aggression: 0.70,
        phases: vec![
            DefensivePatternPhase {
                name: "Wide Overload",
                trigger: "Opponent dribbler isolated on wing",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::TrackBack, 15.0, 0.0, 2.5, Intensity::Sprint), BallAction::None, "Right flank", "Recover to create 2v1 with Rb"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::CoverChannel, 8.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Half-space", "Slide to cover half-space"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 4.0, 0.3, 1.5, Intensity::Jog), BallAction::Tackle, "Dribbler", "Engage dribbler"),
                    def_instruction(PlayerRole::Rcb, movement(MovementDirection::CoverChannel, 5.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Inside channel", "Cover inside channel"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Jog), BallAction::BlockShot, "Cutback zone", "Protect cutback zone"),
                ],
                outcome: "Wide dribbler caught in 2v1",
            },
            DefensivePatternPhase {
                name: "Half-Space Overload",
                trigger: "Ball enters right half-space",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 6.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "Half-space", "Cover half-space"),
                    def_instruction(PlayerRole::Rcb, movement(MovementDirection::CoverChannel, 5.0, 0.3, 1.5, Intensity::Jog), BallAction::None, "Channel", "Step into channel"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 4.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Wide attacker", "Lock wide attacker"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Hold, 0.0, 0.0, 2.5, Intensity::Walk), BallAction::BlockShot, "Inside pocket", "Seal inside pocket"),
                ],
                outcome: "Half-space entry denied",
            },
            DefensivePatternPhase {
                name: "Far-Side Balance",
                trigger: "Ball on right side, far side exposed",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 35.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 10.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Far side", "Tuck inside to protect switch"),
                    def_instruction(PlayerRole::Lcb, movement(MovementDirection::Tuck, 4.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Central zone", "Shift centrally"),
                ],
                outcome: "Far-side switch covered",
            },
        ],
    }
}
