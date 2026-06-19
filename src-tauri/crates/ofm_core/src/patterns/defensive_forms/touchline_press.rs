use engine::PlayStyle;
use crate::patterns::types::{
    DefensivePatternForm, DefensivePatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, def_instruction, movement,
};

pub fn form() -> DefensivePatternForm {
    DefensivePatternForm {
        id: "touchline_press",
        name: "Touchline Press",
        source_md: "Touchline_Press_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::HighPress,
        compactness: 0.70,
        aggression: 0.80,
        phases: vec![
            DefensivePatternPhase {
                name: "Invite Wide Pass",
                trigger: "Opponent CB in build-up",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 52.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Hold, 3.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Lb bait", "Open body showing Lb as free"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::Hold, 4.0, 0.0, 2.0, Intensity::Walk), BallAction::None, "Dm bait", "Withhold pressure on Dm briefly"),
                ],
                outcome: "Opponent commits wide pass",
            },
            DefensivePatternPhase {
                name: "Touchline Trap",
                trigger: "Ball played to Lb",
                tempo_seconds: (0.0, 2.0),
                block_height_pct: 52.0,
                instructions: vec![
                    def_instruction(PlayerRole::Rw, movement(MovementDirection::Press, 6.0, 0.0, 1.2, Intensity::Explosive), BallAction::Tackle, "Lb", "Press from outside shoulder"),
                    def_instruction(PlayerRole::Rcm, movement(MovementDirection::BlockLane, 4.0, 0.2, 1.2, Intensity::Sprint), BallAction::Intercept, "Inside pass", "Close inside pass"),
                    def_instruction(PlayerRole::Rb, movement(MovementDirection::MarkRunner, 5.0, 0.3, 1.8, Intensity::Sprint), BallAction::None, "Lw", "Lock wide attacker"),
                    def_instruction(PlayerRole::St, movement(MovementDirection::CoverChannel, 5.0, 0.3, 1.5, Intensity::Accelerate), BallAction::None, "Return to Cb", "Block return pass"),
                    def_instruction(PlayerRole::Rcb, movement(MovementDirection::CoverChannel, 4.0, 0.5, 2.0, Intensity::Jog), BallAction::None, "Channel", "Cover channel behind press"),
                ],
                outcome: "Throw-in, clearance, or turnover",
            },
            DefensivePatternPhase {
                name: "Far-Side Cover",
                trigger: "Trap set right, far side open",
                tempo_seconds: (0.0, 2.5),
                block_height_pct: 52.0,
                instructions: vec![
                    def_instruction(PlayerRole::Lw, movement(MovementDirection::Tuck, 10.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Far side", "Tuck inside"),
                    def_instruction(PlayerRole::Lcm, movement(MovementDirection::BlockLane, 6.0, 0.0, 2.0, Intensity::Jog), BallAction::None, "Switch lane", "Protect switch lane"),
                ],
                outcome: "Block intact on far side",
            },
        ],
    }
}
