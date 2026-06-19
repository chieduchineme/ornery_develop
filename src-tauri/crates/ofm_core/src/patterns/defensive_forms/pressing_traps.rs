use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "pressing_traps",
        name: "Pressing Traps",
        source_md: "Pressing_Traps_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.70,
        aggression: 0.85,
        phases: vec![
            DefensivePatternPhase {
                name: "Bait the Pass",
                trigger: "Opponent CB has ball, build-up starts",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 55.0,
                instructions: vec![
                    def_instruction(PlayerRole::St, movement(MovementDirection::Hold, 3.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Rb pass", "Leave Rb pass slightly open"),
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Hold, 4.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Lb bait", "Open body to invite Lb pass"),
                ],
                outcome: "Opponent commits to bait lane",
            },
            DefensivePatternPhase {
                name: "Collapse",
                trigger: "Ball played to Lb",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 55.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Press, 5.0, 0.0, 1.2, Intensity::Explosive), BallAction::Tackle, "Lb", "Close from outside shoulder"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 4.0, 0.2, 1.2, Intensity::Sprint), BallAction::Intercept, "Inside pass", "Jump inside option"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 6.0, 0.3, 1.8, Intensity::Sprint), BallAction::None, "Lw", "Lock wide attacker"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 5.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Return pass", "Seal return to Cb"),
                    def_instruction(PlayerRole::Lcb, movement(MovementDirection::CoverChannel, 5.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Depth cover", "Cover depth behind press"),
                ],
                outcome: "Turnover in prepared zone",
            },
            DefensivePatternPhase {
                name: "Far-Side Guard",
                trigger: "Trap set on right, far side exposed",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 55.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 10.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Far side", "Tuck inside for switch protection"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 6.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Switch lane", "Protect switch lane"),
                ],
                outcome: "Switch blocked",
            },
        ],
    }
}
