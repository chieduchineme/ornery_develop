use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "direct_attack",
        name: "Direct Attack",
        source_md: "Direct-Attacks_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.55,
        reward: 0.72,
        phases: vec![
            PatternPhase {
                name: "Direct Ball to Target Striker",
                trigger: "Ball in defensive third — 1-4 passes to final third intended",
                tempo_seconds: (3.0, 7.0),
                width_m: 44.0,
                depth_m: 55.0,
                instructions: vec![
                    instruction(
                        PlayerRole::BallSideCb,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::LongPass,
                        "ST chest",
                        "Long diagonal pass targeted at ST chest; weight chosen to allow ST to shield",
                    ),
                    instruction(
                        PlayerRole::TargetSt,
                        movement(MovementDirection::Drop, 5.0, 0.0, 1.5, Intensity::Jog),
                        BallAction::Layoff,
                        "half-space — back to defender",
                        "Pins CB with body; shields ball on chest and lays off to arriving CM",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Advance, 22.0, 0.5, 3.5, Intensity::Sprint),
                        BallAction::Receive,
                        "top of box — arriving runner",
                        "Support runner arrives to collect ST layoff; 1-2 touch to shoot or switch",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::SprintForward, 25.0, 0.0, 4.0, Intensity::Explosive),
                        BallAction::None,
                        "left channel for second phase",
                        "Sprints in anticipation; available for wide outlet if CM cannot shoot",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DiagonalRun, 22.0, 0.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "right channel near post run",
                        "Diagonal run toward near post; pinning right-side CB during layoff phase",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Advance, 18.0, 1.0, 3.0, Intensity::Accelerate),
                        BallAction::ScreenRestDefense,
                        "edge of box — second ball",
                        "Arrives edge of box for rebound or second ball if CM's shot is blocked",
                    ),
                ],
                outcome: "CM shoots from top of box, or plays to wide for cross after ST layoff",
            },
            PatternPhase {
                name: "Direct Wing Bypass",
                trigger: "Midfield bypassed — CB to advanced winger in 1 pass",
                tempo_seconds: (2.5, 5.0),
                width_m: 60.0,
                depth_m: 50.0,
                instructions: vec![
                    instruction(
                        PlayerRole::FarSideCb,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::DiagonalSwitch,
                        "RW wide position",
                        "Diagonal switch in one touch; long ball over opponent's midfield block",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 5.0, 0.0, 1.5, Intensity::Sprint),
                        BallAction::Receive,
                        "wide right channel",
                        "Had been lurking wide; receives in stride and attacks RB immediately",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 15.0, 1.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Immediate near-post run on switch trigger; pins CB before they recover",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 30.0, 1.5, 4.5, Intensity::Sprint),
                        BallAction::None,
                        "far post long run",
                        "Full pitch sprint to far post; arrives as RW reaches byline",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 20.0, 1.5, 3.5, Intensity::Sprint),
                        BallAction::Shoot,
                        "penalty spot / cutback zone",
                        "Runs to penalty spot; primary receiver of cutback for direct shot",
                    ),
                ],
                outcome: "RW crosses, cuts back, or plays 1v1 vs isolated FB; box occupied in 5s",
            },
        ],
    }
}
