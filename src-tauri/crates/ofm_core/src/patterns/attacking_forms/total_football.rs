use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "total_football",
        name: "Total Football",
        source_md: "Total_football.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Possession,
        risk: 0.55,
        reward: 0.80,
        phases: vec![
            PatternPhase {
                name: "Continuous Positional Interchange",
                trigger: "Any player in any zone — automatic replacement fills vacated space",
                tempo_seconds: (8.0, 20.0),
                width_m: 65.0,
                depth_m: 50.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Advance, 25.0, 0.0, 5.0, Intensity::Jog),
                        BallAction::None,
                        "LW zone — LB becomes winger",
                        "Occupies left wing zone when LW inverts; acts as full winger with crossing quality",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DriftInside, 18.0, 0.0, 4.0, Intensity::Jog),
                        BallAction::Receive,
                        "left half-space — LW becomes CM",
                        "Inverts into left CM zone; receives and distributes as midfielder",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::SprintForward, 22.0, 1.0, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "ST zone — CM becomes striker",
                        "Vacates midfield zone and attacks behind defense as effective striker",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Drop, 20.0, 1.0, 4.0, Intensity::Jog),
                        BallAction::BouncePass,
                        "CM zone — ST becomes midfielder",
                        "Drops into midfield; receives and distributes as CM; CB cannot follow without leaving gap",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::DriftWide, 12.0, 2.0, 3.0, Intensity::Jog),
                        BallAction::None,
                        "CB zone — DM drops into defense",
                        "Fills CB position when CB steps into midfield; automatic chain replacement",
                    ),
                    instruction(
                        PlayerRole::BallSideCb,
                        movement(MovementDirection::Advance, 15.0, 2.0, 3.5, Intensity::Accelerate),
                        BallAction::None,
                        "DM zone — CB becomes midfielder",
                        "Fills vacated DM zone; system auto-reorganizes without positional instruction",
                    ),
                ],
                outcome: "Defense loses track of roles; every space occupied; superiority emerges organically",
            },
            PatternPhase {
                name: "Fluid Final Third Penetration",
                trigger: "System creates free man through role exchange — vertical exploitation",
                tempo_seconds: (2.5, 5.0),
                width_m: 50.0,
                depth_m: 28.0,
                instructions: vec![
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::Hold, 0.0, 0.0, 1.0, Intensity::Walk),
                        BallAction::ThroughBall,
                        "LCM now in striker role",
                        "In striker zone, plays through ball as if a midfielder — role reversal complete",
                    ),
                    instruction(
                        PlayerRole::Lcm,
                        movement(MovementDirection::BlindsideRun, 20.0, 0.5, 2.5, Intensity::Explosive),
                        BallAction::Receive,
                        "behind defense — striker's finish zone",
                        "Receives in behind; has arrived through defender's blind side after role swap",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 18.0, 1.0, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "far post — now in FB/winger hybrid role",
                        "Far-post run as if an attacking fullback; hybrid role exploits confusion",
                    ),
                    instruction(
                        PlayerRole::Lb,
                        movement(MovementDirection::Cross,  0.0, 2.5, 1.0, Intensity::Walk),
                        BallAction::Cross,
                        "penalty area — LB in winger role delivers",
                        "LB now in winger zone delivers cross; opponents' positional assignments in chaos",
                    ),
                ],
                outcome: "LCM finishes as striker, or LB crosses from winger zone to arriving runners",
            },
        ],
    }
}
