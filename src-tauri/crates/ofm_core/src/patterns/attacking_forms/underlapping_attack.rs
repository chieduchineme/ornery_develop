use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "underlapping_attack",
        name: "Underlapping Attack",
        source_md: "Underlapping_Attack_patterns.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Attacking,
        risk: 0.45,
        reward: 0.72,
        phases: vec![
            PatternPhase {
                name: "Classic Fullback Underlap",
                trigger: "LW holds wide — LB makes inside channel run into half-space",
                tempo_seconds: (3.0, 6.0),
                width_m: 50.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftWide, 4.0, 0.0, 1.5, Intensity::Walk),
                        BallAction::Carry,
                        "left touchline — freeze FB defender",
                        "Stays wide and delays; body shape drags opposing FB outward to create inside gap",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Underlap, 22.0, 1.0, 3.0, Intensity::Sprint),
                        BallAction::None,
                        "inside channel — half-space behind defensive line",
                        "Inside run between LW and ST; attacks the channel between opposing CB and FB",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 0.0, 2.0, 1.5, Intensity::Walk),
                        BallAction::ThroughBall,
                        "LB underlapping inside",
                        "Slips ball into LB's inside run at exact moment FB commits outward",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Advance, 8.0, 3.0, 1.5, Intensity::Sprint),
                        BallAction::Cutback,
                        "six-yard box entry — cutback or low cross",
                        "Receives in half-space; drives toward goal and cuts back or shoots",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 10.0, 2.0, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post",
                        "Near-post run occupies CBs; prevents doubling onto underlapping LB",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::AttackCutbackZone, 16.0, 2.5, 2.5, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot",
                        "Arrives penalty spot for LB's cutback; first-time shot option",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::AttackFarPost, 26.0, 2.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "far post",
                        "Far-post run; covers low cross from inside channel or deep cutback",
                    ),
                ],
                outcome: "LB in half-space cuts back to LCM for shot or drives on goal; 3 bodies in box",
            },
            PatternPhase {
                name: "CM Underlap from Midfield",
                trigger: "Winger pinned wide, CM makes underlapping central run",
                tempo_seconds: (2.5, 5.0),
                width_m: 44.0,
                depth_m: 26.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DriftWide, 5.0, 0.0, 1.5, Intensity::Jog),
                        BallAction::None,
                        "right touchline — attract FB engagement",
                        "Pins RB toward touchline; creates inside channel for CM underlap",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Underlap, 20.0, 1.0, 2.8, Intensity::Sprint),
                        BallAction::None,
                        "inside right channel — behind defensive line",
                        "Underlaps inside RW; runs between opposing CM and FB into penalty area",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk),
                        BallAction::ThroughBall,
                        "RCM underlapping run",
                        "Plays through ball to RCM; timing waits for RW's decoy to pull FB wide",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Advance, 6.0, 2.8, 1.2, Intensity::Sprint),
                        BallAction::Shoot,
                        "edge of six-yard box",
                        "Receives inside and shoots low immediately; arrives before GK can set",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 8.0, 1.5, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post — CB occupier",
                        "Near-post run stops both CBs from sliding across to block RCM's run",
                    ),
                ],
                outcome: "RCM arrives in six-yard box via underlap for close-range finish; ST occupies CBs",
            },
        ],
    }
}
