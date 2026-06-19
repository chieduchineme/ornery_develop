use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "goalkeeper_sweeper_defense",
        name: "Goalkeeper Sweeper Defense",
        source_md: "Goalkeeper_Sweeper_Defense_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        compactness: 0.65,
        aggression: 0.55,
        phases: vec![
            DefensivePatternPhase {
                name: "High Line Hold",
                trigger: "Team in possession, high line active",
                tempo_seconds: (0.0, 5.0),
                block_height_pct: 60.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::StepUp, 18.0, 0.0, 5.0, Intensity::Walk), BallAction::None, "High position", "Start 18m outside six-yard area"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::StepUp, 10.0, 0.0, 5.0, Intensity::Jog), BallAction::None, "High line", "Hold compact and high"),
                    def_instruction(PlayerRole::Dm, movement(MovementDirection::Press, 12.0, 0.0, 2.5, Intensity::Accelerate), BallAction::None, "Passer", "Pressure passer to force error"),
                ],
                outcome: "High line maintained, opponent compressed",
            },
            DefensivePatternPhase {
                name: "Through-Ball Sweep",
                trigger: "Through ball played behind high line",
                tempo_seconds: (0.0, 3.0),
                block_height_pct: 60.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::RecoverShape, 20.0, 0.0, 2.5, Intensity::Explosive), BallAction::Clearance, "Through ball", "Sprint to intercept or clear"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::StepUp, 0.0, 0.0, 2.5, Intensity::Jog), BallAction::CallOffsideTrap, "Offside line", "Hold offside line"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::RecoverShape, 10.0, 0.3, 2.0, Intensity::Sprint), BallAction::None, "Cover", "Recover inside"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::RecoverShape, 10.0, 0.3, 2.0, Intensity::Sprint), BallAction::None, "Cover", "Recover inside"),
                ],
                outcome: "GK claims or clears before striker arrives",
            },
            DefensivePatternPhase {
                name: "Distribute",
                trigger: "GK wins the ball",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 60.0,
                instructions: vec![
                    def_instruction(PlayerRole::Gk, movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk), BallAction::Receive, "Outlet", "Play quick pass to outlet"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Drop, 10.0, 0.0, 2.0, Intensity::Jog), BallAction::Receive, "Short option", "Provide short option"),
                ],
                outcome: "Possession retained after sweep",
            },
        ],
    }
}
