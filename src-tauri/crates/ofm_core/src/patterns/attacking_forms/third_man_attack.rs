use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "third_man_attack",
        name: "Third-Man Run Attack",
        source_md: "Third_man_attack_pattern.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.35,
        reward: 0.75,
        phases: vec![
            PatternPhase {
                name: "Classic Wall-Pass Third Man",
                trigger: "Ball in midfield under pressure — wall pass bypasses press, third man runs in",
                tempo_seconds: (2.5, 5.0),
                width_m: 32.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 8.0, 0.0, 1.0, Intensity::Accelerate),
                        BallAction::OneTouchPass,
                        "DM — wall trigger",
                        "Plays to DM under light pressure; immediately sprints on return path",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 0.5, Intensity::Walk),
                        BallAction::BouncePass,
                        "LCM running beyond — one-touch return",
                        "Instant return to LCM who has gone beyond; two-pass sequence takes 1.2 seconds",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::BlindsideRun, 14.0, 0.5, 2.0, Intensity::Sprint),
                        BallAction::ThroughBall,
                        "RW making third-man run",
                        "Receives in behind press; immediately plays RW who has timed third-man run",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::BlindsideRun, 22.0, 1.0, 2.8, Intensity::Explosive),
                        BallAction::Receive,
                        "behind defensive line — third man",
                        "Had been holding wide; blindside sprint precisely timed to LCM receiving; arrives unmarked",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 12.0, 2.0, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post — occupies CBs",
                        "Pins both CBs through near-post run; cannot track RW without leaving gap",
                    ),
                ],
                outcome: "RW receives unmarked behind defense; 1v1 with GK or squares to arriving LCM",
            },
            PatternPhase {
                name: "DM to ST to Third-Man Runner",
                trigger: "ST drops to receive from DM — CM third-man run in behind",
                tempo_seconds: (2.0, 4.5),
                width_m: 28.0,
                depth_m: 25.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 0.5, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "ST dropping",
                        "Plays to dropping ST; pass is vertical to draw CB forward",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 10.0, 0.0, 1.5, Intensity::Jog),
                        BallAction::BouncePass,
                        "Rcm third-man run",
                        "Receives from DM and lays off first-touch to RCM making the third run",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::SprintForward, 20.0, 1.0, 2.5, Intensity::Explosive),
                        BallAction::Receive,
                        "vacated space behind CB who followed ST",
                        "Sprints into gap left by CB who followed ST's dropping run; third man sequence complete",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 5.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::None,
                        "right wide — pin FB",
                        "Stays wide throughout; prevents RB from covering the gap RCM runs into",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DiagonalRun, 16.0, 1.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "left zone of box — secondary runner",
                        "Secondary run into left box zone; second option if RCM needs to lay off",
                    ),
                ],
                outcome: "RCM receives behind CB in space; through ball to RCM who was third man in sequence",
            },
            PatternPhase {
                name: "Winger Combination Third Man",
                trigger: "LW and LB combine — CM arrives as third man through the center",
                tempo_seconds: (2.5, 5.0),
                width_m: 40.0,
                depth_m: 25.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 3.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "LB inside",
                        "Short pass inside to LB; triggers LB's immediate return sequence",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Hold, 0.0, 0.5, 0.5, Intensity::Walk),
                        BallAction::BouncePass,
                        "LW returning run",
                        "One-touch return to LW who has made forward run beyond",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Advance, 10.0, 0.5, 1.5, Intensity::Sprint),
                        BallAction::ThroughBall,
                        "Lcm third-man central run",
                        "Receives LB's return and plays LCM who is the genuine third man through middle",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::BlindsideRun, 18.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "central channel behind defense",
                        "Timed central run while LW-LB combination occupied defensive attention on left",
                    ),
                ],
                outcome: "LCM arrives unmarked central via third-man principle; direct shot or through to ST",
            },
        ],
    }
}
