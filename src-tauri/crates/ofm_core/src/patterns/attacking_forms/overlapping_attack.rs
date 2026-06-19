use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "overlapping_attack",
        name: "Overlapping Attack",
        source_md: "Overlapping_attacks_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.45,
        reward: 0.70,
        phases: vec![
            PatternPhase {
                name: "Classic Overlap with Timing Fix",
                trigger: "Winger holds ball wide — fullback timed overlap run",
                tempo_seconds: (3.5, 7.0),
                width_m: 60.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 3.0, 0.0, 1.5, Intensity::Walk),
                        BallAction::Carry,
                        "touchline — draw and fix defender",
                        "Carries slowly to freeze RB; body shape invites defensive engagement",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Overlap, 26.0, 1.0, 3.5, Intensity::Explosive),
                        BallAction::None,
                        "outside RW — overlapping channel",
                        "Explosive outside run triggered when RW touches ball; 26m sprint in 3.5s",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::Hold, 0.0, 1.5, 2.0, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "RB overlapping in space",
                        "Plays to overlapping RB at the moment defender has committed inward",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Advance, 8.0, 3.5, 1.5, Intensity::Sprint),
                        BallAction::Cross,
                        "penalty area — near post or cutback",
                        "Receives on run; crosses in stride within 1-2 touches at full pace",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 12.0, 2.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Near-post run timed to RB arrival at byline",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 26.0, 2.5, 4.0, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Full diagonal run to far post; coverage for second ball and headed clearances",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::AttackCutbackZone, 18.0, 2.5, 3.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot",
                        "Arrives penalty spot; primary receiver of RB's pull-back or low cross",
                    ),
                ],
                outcome: "RB crosses into pre-loaded box; ST near post, RCM penalty spot, LW far post",
            },
            PatternPhase {
                name: "Double Overlap Overload",
                trigger: "Fullback and CM overlap simultaneously — 3v2 on flank",
                tempo_seconds: (4.0, 8.0),
                width_m: 62.0,
                depth_m: 30.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 4.0, 0.0, 1.5, Intensity::Jog),
                        BallAction::Carry,
                        "left touchline",
                        "Draws two defenders with ball at feet; deliberate slowdown creates overload trigger",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Overlap, 24.0, 0.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "outside channel",
                        "Outside overlap; first overlapping option to stretch the defensive shape",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Overlap, 20.0, 1.0, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "second outside channel beyond LB",
                        "Second overlap arriving after LB; creates numerical overload 3v2 on wing",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 0.0, 2.0, 1.5, Intensity::Walk),
                        BallAction::OneTouchPass,
                        "furthest overlapper — LCM in space",
                        "Plays to the furthest overlapper in most space; defense cannot cover all three",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 14.0, 3.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Runs to near post as final pass is played",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::AttackFarPost, 28.0, 3.0, 4.0, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Arrives far post from opposite wing; wide delivery coverage",
                    ),
                ],
                outcome: "LCM or LB crosses from overloaded flank into box loaded by ST and RW",
            },
        ],
    }
}
