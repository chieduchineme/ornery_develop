use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "switch_of_play",
        name: "Switch of Play",
        source_md: "Switch_of_Play_Patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.30,
        reward: 0.65,
        phases: vec![
            PatternPhase {
                name: "Build-Up Side Congestion and Switch",
                trigger: "Left side overloaded by opponent — CB switches to isolated RW",
                tempo_seconds: (4.0, 8.0),
                width_m: 65.0,
                depth_m: 40.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftInside, 8.0, 0.0, 2.0, Intensity::Jog),
                        BallAction::None,
                        "left half-space — attract press",
                        "Deliberately draws opponent pressure left; acts as decoy for switch trigger",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk),
                        BallAction::Recycle,
                        "DM — recycle for switch",
                        "Plays inside to DM under pressure; quick recycle before switch is made",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::DiagonalSwitch,
                        "RW — isolated on far side",
                        "One-touch long switch to RW; diagonal pass 40-50m across pitch in 1 second",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 6.0, 2.0, 1.5, Intensity::Sprint),
                        BallAction::Receive,
                        "right wide channel — isolated 1v1",
                        "Receives in stride with 1v1 space; RB caught on wrong side after switch",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Overlap, 24.0, 3.5, 3.5, Intensity::Explosive),
                        BallAction::None,
                        "outside RW — overlap run",
                        "Explosive overlap after switch; creates 2v1 on RW's isolated defender",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::DiagonalRun, 18.0, 3.0, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "near post run after switch",
                        "Diagonal run to near post triggered by DM's switch; arrives as RW delivers",
                    ),
                ],
                outcome: "RW isolated 1v1 after switch; crosses to ST or plays RB overlap for cutback",
            },
            PatternPhase {
                name: "Midfield Switch to Half-Space",
                trigger: "Pressure on one side — DM switches to advanced CM in opposite half-space",
                tempo_seconds: (2.5, 5.0),
                width_m: 56.0,
                depth_m: 30.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::DiagonalSwitch,
                        "RCM in right half-space",
                        "Switch to opposite half-space CM; diagonal 30-35m pass bypasses press",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DriftInside, 12.0, 0.5, 2.0, Intensity::Accelerate),
                        BallAction::Receive,
                        "right half-space between lines",
                        "Positions in half-space before switch; receives on move to maintain momentum",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::BlindsideRun, 20.0, 1.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "behind defensive line on right",
                        "Blindside run exploiting RCM's reception; stays onside by timing off ball",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::DiagonalRun, 14.0, 1.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "central penalty area",
                        "Diagonal central run; occupies CBs during switch exploitation phase",
                    ),
                ],
                outcome: "RCM between lines plays RW through or drives at goal from half-space",
            },
        ],
    }
}
