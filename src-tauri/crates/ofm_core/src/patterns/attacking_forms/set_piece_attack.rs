use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "set_piece_attack",
        name: "Set-Piece Attack",
        source_md: "Set-Piece_Attack_pattern.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Balanced,
        risk: 0.30,
        reward: 0.65,
        phases: vec![
            PatternPhase {
                name: "Corner — Zonal Box Loading",
                trigger: "Corner kick awarded — scripted box occupation triggered",
                tempo_seconds: (3.0, 6.0),
                width_m: 40.0,
                depth_m: 18.0,
                instructions: vec![
                    instruction(
                        PlayerRole::TargetSt,
                        movement(MovementDirection::AttackNearPost, 8.0, 0.0, 2.0, Intensity::Explosive),
                        BallAction::None,
                        "near post — blocking and flick run",
                        "Near-post dart timed to delivery; sets block for second runner or flicks on",
                    ),
                    instruction(
                        PlayerRole::Cb,
                        movement(MovementDirection::AttackFarPost, 10.0, 0.5, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "far post — aerial target",
                        "Primary aerial target at far post; timing run from penalty area edge",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::DiagonalRun, 12.0, 0.5, 2.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "penalty spot — second-ball shooter",
                        "Diagonal run to penalty spot; arrives to meet flick-on or loose ball",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::AttackCutbackZone, 8.0, 1.0, 2.0, Intensity::Accelerate),
                        BallAction::Shoot,
                        "edge of six-yard box left",
                        "Peels off back of zone; second-ball crash position to left of penalty spot",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::ScreenRestDefense,
                        "top of penalty area — second-ball shield",
                        "Holds top of D for loose clearances; prevents counter-attack on transition",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 2.0, 0.0, 4.0, Intensity::Walk),
                        BallAction::None,
                        "edge of penalty area — blocker",
                        "Sets stationary block at edge of box; disrupts zonal markers' run paths",
                    ),
                ],
                outcome: "Inswinging or outswinging delivery into crowded box; ST flick, CB header, or LCM second ball",
            },
            PatternPhase {
                name: "Free Kick — Wall-Bypass Routine",
                trigger: "Direct free kick 20-25m from goal — pre-set routing",
                tempo_seconds: (2.5, 5.0),
                width_m: 30.0,
                depth_m: 22.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::Hold, 0.0, 0.0, 2.0, Intensity::Walk),
                        BallAction::Shoot,
                        "top corner — over/around wall",
                        "Primary shooter; curls or dips ball over wall toward far post corner",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::DiagonalRun, 8.0, 1.0, 1.5, Intensity::Sprint),
                        BallAction::None,
                        "wall end — run through gap",
                        "Dummy run at wall end; disrupts GK's sightline and positioning",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 10.0, 1.5, 2.0, Intensity::Sprint),
                        BallAction::None,
                        "near post — rebound",
                        "Near-post run for deflected shot or GK rebound; arrives just after kick",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::Hold, 3.0, 0.0, 3.0, Intensity::Walk),
                        BallAction::None,
                        "left side — dummy runner",
                        "Makes decoy run toward ball; forces wall to move or GK to account for second man",
                    ),
                    instruction(
                        PlayerRole::BallSideCb,
                        movement(MovementDirection::AttackFarPost, 14.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "far post — late aerial run",
                        "Late run from deepest position; arrives unmarked at far post as defense focused on wall zone",
                    ),
                ],
                outcome: "Shot over wall toward far corner; or lateral lay-off to LW for low-driven shot",
            },
        ],
    }
}
