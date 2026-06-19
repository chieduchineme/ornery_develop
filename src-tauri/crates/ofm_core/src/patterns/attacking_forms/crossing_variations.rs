use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "crossing_variations",
        name: "Crossing Variations",
        source_md: "Crossing_Variations_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.45,
        reward: 0.65,
        phases: vec![
            PatternPhase {
                name: "Early Cross",
                trigger: "Winger receives wide with space — crosses within 1-2 touches before box is set",
                tempo_seconds: (1.5, 3.0),
                width_m: 62.0,
                depth_m: 30.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 2.0, 0.0, 0.8, Intensity::Jog),
                        BallAction::Cross,
                        "behind defensive line — early delivery",
                        "No dribble; whips cross within 2 touches before defense has organized",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 14.0, 0.0, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post — first-touch zone",
                        "Attacks near post; early cross arrives before CB can position properly",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DiagonalRun, 20.0, 0.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "penalty spot",
                        "Crash-runs to penalty spot; anticipates early cross before receiving LW's pass",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::AttackFarPost, 28.0, 0.0, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Full sprint to far post; flick-on or second-ball coverage",
                    ),
                ],
                outcome: "Cross arrives before defense is set; ST near post, RCM penalty spot, RW far post",
            },
            PatternPhase {
                name: "Inswinger Cross",
                trigger: "Right-footed player on right byline — inswinger into box",
                tempo_seconds: (2.5, 5.0),
                width_m: 58.0,
                depth_m: 26.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rb,
                        movement(MovementDirection::Overlap, 24.0, 0.0, 3.5, Intensity::Sprint),
                        BallAction::Cross,
                        "near-post zone — inswinging delivery",
                        "Inswinging cross from right side; right-foot curl curves into box, aided by momentum",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackFarPost, 16.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "far post — inswinger target",
                        "Attacks far post; inswinger curls into body's path for headed finish",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::AttackNearPost, 14.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post — flick on",
                        "Near-post run; may flick on inswinger to reverse trajectory for far-post runner",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 20.0, 1.0, 3.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot / edge of six-yard box",
                        "Primary shot zone; collects if ball falls from inswinger challenge",
                    ),
                ],
                outcome: "Inswinging cross met at far post by ST, flicked on by RW, or drops to LCM",
            },
            PatternPhase {
                name: "Driven Low Cross",
                trigger: "Winger in wide area, multiple runners set in box — low cutback area",
                tempo_seconds: (2.0, 4.0),
                width_m: 55.0,
                depth_m: 24.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::Advance, 20.0, 0.0, 2.5, Intensity::Sprint),
                        BallAction::Cross,
                        "six-yard box — low driven delivery",
                        "Driven low cross across face of goal; pace makes GK decision difficult",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 10.0, 0.5, 1.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "near post — low cross first contact",
                        "Attacks the ball at near post; low cross at knee height ideal for redirect",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 30.0, 0.5, 4.0, Intensity::Sprint),
                        BallAction::Shoot,
                        "far post",
                        "Low cross arriving at far post; tap-in if missed by ST",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::AttackCutbackZone, 18.0, 0.5, 3.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot — pull-back zone",
                        "Arrives as if expecting pull-back; deflected cross may reach this zone",
                    ),
                ],
                outcome: "Driven low cross redirected at near post by ST or met at far post by LW",
            },
        ],
    }
}
