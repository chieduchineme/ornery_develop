use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "cover_shadow_defending",
        name: "Cover Shadow Defending",
        source_md: "Cover_Shadow_Defending_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.65,
        aggression: 0.75,
        phases: vec![
            DefensivePatternPhase {
                name: "Shadow Setup",
                trigger: "Opponent builds through CB",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 58.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 12.0, 0.0, 2.5, Intensity::Accelerate), BallAction::None, "Dm", "Curve run to shadow Dm from CB"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Sprint), BallAction::None, "Lb", "Press Lb while hiding Rcm pass"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 5.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Rcm pass", "Position to block Rcm pass"),
                ],
                outcome: "Pivot hidden, opponent options narrowed",
            },
            DefensivePatternPhase {
                name: "Trap Open Lane",
                trigger: "Opponent looks for wide escape",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 58.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Hold, 2.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Wide bait", "Invite wide pass deliberately"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::StepUp, 6.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Rb", "Draw Rb into touchline"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::MarkRunner, 4.0, 0.5, 1.5, Intensity::Accelerate), BallAction::None, "Jump target", "Prepare jump on pass"),
                ],
                outcome: "Opponent commits to bait lane",
            },
            DefensivePatternPhase {
                name: "Collapse",
                trigger: "Ball played into bait zone",
                tempo_seconds: (0.0, 1.5),
                block_height_pct: 58.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 5.0, 0.0, 1.2, Intensity::Explosive), BallAction::Tackle, "Wide receiver", "Close from outside"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::BlockLane, 4.0, 0.2, 1.2, Intensity::Sprint), BallAction::Intercept, "Inside option", "Jump inside option"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::CoverChannel, 5.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Depth channel", "Cover depth behind press"),
                ],
                outcome: "Turnover in prepared zone",
            },
        ],
    }
}
