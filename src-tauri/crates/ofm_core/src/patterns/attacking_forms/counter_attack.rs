use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "counter_attack",
        name: "Counter Attack",
        source_md: "Counter-Attack_Patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Counter,
        risk: 0.40,
        reward: 0.80,
        phases: vec![
            PatternPhase {
                name: "Vertical Break Counter",
                trigger: "Ball won in own half while opponent attacks — 3-5 players exposed high",
                tempo_seconds: (4.0, 10.0),
                width_m: 50.0,
                depth_m: 70.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::PressingRecovery,
                        "recovery zone — own half",
                        "Wins ball; immediately scans for vertical outlet; does not carry forward",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::SprintForward, 35.0, 0.0, 4.5, Intensity::Explosive),
                        BallAction::None,
                        "behind opponent defensive line",
                        "Starts run before ball is won (anticipation); exploits exposed space at depth",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::SprintForward, 30.0, 0.3, 4.0, Intensity::Sprint),
                        BallAction::None,
                        "left channel at depth",
                        "Parallel sprint to ST; pins LB and LCB to prevent double-team on ST",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::SprintForward, 30.0, 0.3, 4.0, Intensity::Sprint),
                        BallAction::None,
                        "right channel at depth",
                        "Mirrors LW run; creates 3v4 or 3v3 attacking scenario if transition is fast",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 20.0, 1.0, 4.0, Intensity::Accelerate),
                        BallAction::None,
                        "midfield advance — second wave",
                        "Second wave runner; arrives if first attack is blocked for second-ball recovery",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::CounterBalance, 5.0, 0.0, 6.0, Intensity::Jog),
                        BallAction::None,
                        "central midfield — shape balance",
                        "Holds shape to prevent counter; does not join attack unless team wins clear chance",
                    ),
                ],
                outcome: "Through ball to ST on the run; or wide pass to winger for 1v1 vs lone FB",
            },
            PatternPhase {
                name: "Wide Channel Counter",
                trigger: "Ball won with winger already high, opposing FB out of position",
                tempo_seconds: (3.0, 6.0),
                width_m: 60.0,
                depth_m: 55.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::PressingRecovery,
                        "own half — ball recovery",
                        "Wins ball and immediately plays wide pass in 1 touch before opponent recovers",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::SprintForward, 25.0, 0.0, 3.5, Intensity::Explosive),
                        BallAction::Receive,
                        "left channel, in behind defensive line",
                        "Receives ball in stride; first touch forward at pace to maintain momentum",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::DiagonalRun, 22.0, 0.5, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "near post run from central position",
                        "Diagonal run toward near post; occupies two CBs to isolate LW vs FB",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::AttackFarPost, 28.0, 1.0, 4.0, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Long far-post run; covers cutback if LW passes back to LCM or pulls cross back",
                    ),
                ],
                outcome: "LW beats FB and crosses or cuts inside to shoot; or plays ST through near post",
            },
        ],
    }
}
