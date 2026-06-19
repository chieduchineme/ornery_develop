use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "possession_based",
        name: "Possession-Based Attack",
        source_md: "Possession-Based_pattern.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.25,
        reward: 0.55,
        phases: vec![
            PatternPhase {
                name: "U-Shape Circulation",
                trigger: "Team in settled possession — side-to-side movement to create gaps",
                tempo_seconds: (8.0, 20.0),
                width_m: 65.0,
                depth_m: 40.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Advance, 18.0, 0.0, 4.0, Intensity::Jog),
                        BallAction::Receive,
                        "left flank — high position",
                        "Left back pushes high; receives from CB and holds until defensive shape shifts",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::DriftInside, 8.0, 2.0, 2.5, Intensity::Jog),
                        BallAction::OneTouchPass,
                        "left half-space — midfield level",
                        "Checks into half-space; one-touch pass to DM to draw pressure across",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::Recycle,
                        "central midfield — axis",
                        "Receives and switches play; central pivot for direction change",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DriftWide, 10.0, 3.0, 2.5, Intensity::Jog),
                        BallAction::Receive,
                        "right half-space",
                        "Moves wide to receive after switch; opens next circulation pattern",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Advance, 15.0, 4.0, 3.0, Intensity::Jog),
                        BallAction::None,
                        "right flank — high overlap",
                        "Pushes forward after switch to right side; threatens wide overlap",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 5.0, 0.0, 8.0, Intensity::Walk),
                        BallAction::None,
                        "central — holding line",
                        "Stays central but adjusts 2-3m side to side; maintains vertical threat on CBs",
                    ),
                ],
                outcome: "Defense shifted enough to expose gap — trigger penetration pass",
            },
            PatternPhase {
                name: "Penetration Trigger",
                trigger: "Defender overcommits or gap appears after circulation",
                tempo_seconds: (1.5, 3.5),
                width_m: 50.0,
                depth_m: 30.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Advance, 14.0, 0.0, 2.0, Intensity::Accelerate),
                        BallAction::ThroughBall,
                        "gap between lines",
                        "Plays penetrating pass into gap the moment defender overcommits",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::BlindsideRun, 20.0, 0.5, 2.5, Intensity::Sprint),
                        BallAction::Receive,
                        "behind defensive line — right channel",
                        "Times run to stay onside; had been patient during circulation, now explosive",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::DiagonalRun, 12.0, 0.5, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "central penalty area",
                        "Diagonal run from waiting position; occupies CBs while RW receives",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 18.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "penalty spot",
                        "Late run to penalty spot; primary shooting position if RW plays cutback",
                    ),
                ],
                outcome: "RW in 1v1 behind defense; cutback to LCM or shot at near post",
            },
        ],
    }
}
