use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "funnel_defending",
        name: "Funnel Defending",
        source_md: "Funnel_Defending_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.80,
        aggression: 0.50,
        phases: vec![
            DefensivePatternPhase {
                name: "Close Central Lane",
                trigger: "Opponent CB has ball",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 45.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Accelerate), BallAction::None, "CB", "Press from inside angle"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 6.0, 0.2, 2.0, Intensity::Jog), BallAction::None, "Inside pass", "Block inside pass option"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 6.0, 0.2, 2.0, Intensity::Jog), BallAction::None, "Central switch", "Cover central switch lane"),
                ],
                outcome: "Central lane closed, ball forced wide",
            },
            DefensivePatternPhase {
                name: "Touchline Prep",
                trigger: "Ball moves to wide area",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 45.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 8.0, 0.0, 1.8, Intensity::Accelerate), BallAction::None, "Wide carrier", "Take outside approach angle"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::MarkRunner, 4.0, 0.3, 1.5, Intensity::Jog), BallAction::None, "Rw", "Prepare touchline duel"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::DoubleTeam, 6.0, 0.5, 1.5, Intensity::Accelerate), BallAction::None, "Wide carrier", "Back up wide press"),
                ],
                outcome: "Ball committed to touchline side",
            },
            DefensivePatternPhase {
                name: "Touchline Trap",
                trigger: "Ball carrier at touchline",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 45.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 4.0, 0.0, 1.2, Intensity::Explosive), BallAction::Tackle, "Wide carrier", "Close from outside"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::BlockLane, 3.0, 0.2, 1.2, Intensity::Sprint), BallAction::Intercept, "Inside pass", "Cut inside pass"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 5.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Return pass", "Seal return to Cb"),
                ],
                outcome: "Throw-in, clearance, or turnover",
            },
        ],
    }
}
