use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "combination_crossing",
        name: "Combination Crossing Attack",
        source_md: "Combination_Crossing_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.40,
        reward: 0.70,
        phases: vec![
            PatternPhase {
                name: "Wide Triangle Build",
                trigger: "Ball on flank — 2-3 pass combination before crossing",
                tempo_seconds: (4.0, 8.0),
                width_m: 58.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Advance, 15.0, 0.0, 3.0, Intensity::Jog),
                        BallAction::OneTouchPass,
                        "RW — inside pocket",
                        "Passes inside to RW then overlaps; triangle with RW and RCM",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftInside, 5.0, 0.5, 1.5, Intensity::Jog),
                        BallAction::OneTouchPass,
                        "RCM arriving inside",
                        "Inside touch to RCM; runs to create overlap option or underlap for RB",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Advance, 12.0, 1.0, 2.5, Intensity::Accelerate),
                        BallAction::BouncePass,
                        "RB overlapping wide",
                        "Plays overlapping RB at the exact moment defense has shifted inside",
                    ),
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Overlap, 22.0, 1.0, 3.5, Intensity::Sprint),
                        BallAction::Cross,
                        "box — near post / penalty spot",
                        "Receives on overlap and delivers cross; crosses in stride for quality",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 12.0, 2.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Near-post dart timed to RB's crossing stride; first-touch finish zone",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 24.0, 2.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Long far-post run; covers cutback zone or flicked-on headers",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 18.0, 2.5, 3.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot",
                        "Arrives penalty spot for cutback; positioned for first-time shot",
                    ),
                ],
                outcome: "Cross from RB in stride into pre-loaded box; ST near post, LCM penalty spot, LW far post",
            },
            PatternPhase {
                name: "Cutback Combination Cross",
                trigger: "Winger reaches byline — pulls back rather than crossing",
                tempo_seconds: (3.0, 6.0),
                width_m: 50.0,
                depth_m: 22.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Advance, 18.0, 0.0, 2.5, Intensity::Sprint),
                        BallAction::Cutback,
                        "edge of six-yard box / penalty spot",
                        "Reaches byline after 1-2 combination; pulls ball back to penalty spot",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 20.0, 0.0, 3.0, Intensity::Sprint),
                        BallAction::Shoot,
                        "penalty spot",
                        "Times run to arrive at penalty spot as LW pulls back; first-time shot",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DiagonalRun, 18.0, 0.5, 3.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "edge of box — secondary shooting zone",
                        "Second option at edge of box; collects if cutback rolls through",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 10.0, 1.5, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post — dummy or flick",
                        "Near-post decoy run; may flick on or dummy to let ball reach penalty spot",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Jog),
                        BallAction::ScreenRestDefense,
                        "left back — defensive cover",
                        "Holds position; does not join attack to maintain defensive balance",
                    ),
                ],
                outcome: "Cutback to LCM or RCM for first-time shot; 3 bodies in box for rebounds",
            },
        ],
    }
}
