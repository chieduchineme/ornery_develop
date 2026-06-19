use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "mid_block_defense",
        name: "Mid Block Defense",
        source_md: "Mid_Block_Defense_patterns.md",
        base_formation: "4-4-2",
        preferred_play_style: PlayStyle::Defensive,
        compactness: 0.80,
        aggression: 0.55,
        phases: vec![
            DefensivePatternPhase {
                name: "Midfield Screen",
                trigger: "Opponent circulates in their half",
                tempo_seconds: (0.0, 4.0),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 5.0, 0.0, 4.0, Intensity::Jog), BallAction::None, "Pivot", "Screen central pivot"),
                    def_instruction(PlayerRole::TargetSt, movement(MovementDirection::Hold, 5.0, 0.0, 4.0, Intensity::Jog), BallAction::None, "Pivot", "Screen second pivot option"),
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Central lane", "Narrow inside"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 8.0, 0.0, 3.0, Intensity::Jog), BallAction::None, "Central lane", "Narrow inside"),
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk), BallAction::None, "Central lane", "Protect central lane"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk), BallAction::None, "Back line", "Compact back four"),
                ],
                outcome: "Central lane denied, opponent forced wide",
            },
            DefensivePatternPhase {
                name: "Wide Press",
                trigger: "Ball played to wide fullback",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Press, 10.0, 0.0, 2.0, Intensity::Sprint), BallAction::Tackle, "Rb", "Jump when Rb receives"),
                    def_instruction(PlayerRole::Lb, movement(MovementDirection::MarkRunner, 6.0, 0.3, 2.0, Intensity::Accelerate), BallAction::None, "Rw", "Lock wide attacker"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::SqueezeUp, 8.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Inside pass", "Slide across to deny inside"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 6.0, 0.3, 2.0, Intensity::Jog), BallAction::None, "Return pass", "Block return to Cb"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Tuck, 5.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Central lane", "Tuck inside for balance"),
                ],
                outcome: "Forced backward or cross from poor angle",
            },
            DefensivePatternPhase {
                name: "Switch Recovery",
                trigger: "Ball switched to far side",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 42.0,
                instructions: vec![
                    def_instruction(PlayerRole::Cm, movement(MovementDirection::ShieldGoal, 6.0, 0.0, 2.5, Intensity::Jog), BallAction::None, "Switch lane", "Protect far side"),
                    def_instruction(PlayerRole::Cb, movement(MovementDirection::SqueezeUp, 6.0, 0.2, 2.0, Intensity::Jog), BallAction::None, "Back line", "Shift back line intact"),
                ],
                outcome: "Block shifts intact",
            },
        ],
    }
}
