use engine::PlayStyle;
use crate::patterns::types::{
    AttackingPatternForm, PatternPhase, BallAction, Intensity,
    MovementDirection, PlayerRole, instruction, movement,
};

pub fn form() -> AttackingPatternForm {
    AttackingPatternForm {
        id: "fast_breaks",
        name: "Fast Breaks",
        source_md: "Fast-Breaks_pattern.md",
        base_formation: "4-3-3",
        preferred_play_style: PlayStyle::Counter,
        risk: 0.35,
        reward: 0.75,
        phases: vec![
            PatternPhase {
                name: "Central Fast Break",
                trigger: "Ball recovered in midfield — defense unorganized within 3s",
                tempo_seconds: (4.0, 8.0),
                width_m: 42.0,
                depth_m: 60.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Cm,
                        movement(MovementDirection::Advance, 25.0, 0.0, 3.0, Intensity::Sprint),
                        BallAction::Carry,
                        "central corridor toward final third",
                        "Carries ball at speed; covers ~25m in 3s to maintain attack momentum before defense recovers",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::SprintForward, 30.0, 0.0, 4.0, Intensity::Explosive),
                        BallAction::None,
                        "behind defensive line central",
                        "Runs in tandem with CM carry; splits the two CBs to create shooting lane",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::DiagonalRun, 28.0, 0.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "left channel overlapping",
                        "Diagonal sprint to stretch right side of retreating defense",
                    ),
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::DiagonalRun, 28.0, 0.5, 3.5, Intensity::Sprint),
                        BallAction::None,
                        "right channel overlapping",
                        "Mirrors LW to create 3-on-3 or better; forces defenders into 1v1 decisions",
                    ),
                    instruction(
                        PlayerRole::Dm,
                        movement(MovementDirection::CounterBalance, 10.0, 0.0, 5.0, Intensity::Jog),
                        BallAction::ScreenRestDefense,
                        "behind attack — midfield shield",
                        "Jogging advance only; protects against counter if possession is lost",
                    ),
                ],
                outcome: "Through ball to ST, or CM shoots on sight; reach final third within 8 seconds",
            },
            PatternPhase {
                name: "Wide Fast Break",
                trigger: "Ball switched to advanced winger in space — FB exposed",
                tempo_seconds: (3.0, 6.0),
                width_m: 58.0,
                depth_m: 45.0,
                instructions: vec![
                    instruction(
                        PlayerRole::Rw,
                        movement(MovementDirection::SprintForward, 22.0, 0.0, 3.0, Intensity::Explosive),
                        BallAction::Receive,
                        "right wide channel in behind",
                        "Receives in stride; does not break stride or slow down for ball control",
                    ),
                    instruction(
                        PlayerRole::St,
                        movement(MovementDirection::AttackNearPost, 18.0, 0.5, 2.5, Intensity::Sprint),
                        BallAction::None,
                        "near post cutting in",
                        "Near-post run from inside; one CB must track, opening space for cross or cutback",
                    ),
                    instruction(
                        PlayerRole::Lw,
                        movement(MovementDirection::AttackFarPost, 32.0, 0.5, 4.5, Intensity::Sprint),
                        BallAction::None,
                        "far post long diagonal run",
                        "Long diagonal sprint across the pitch; arrives far post 1s after cross",
                    ),
                    instruction(
                        PlayerRole::Rcm,
                        movement(MovementDirection::Advance, 18.0, 1.0, 3.5, Intensity::Accelerate),
                        BallAction::None,
                        "edge of box — cutback zone",
                        "Second wave support; collects cutback or rebound at edge of penalty area",
                    ),
                ],
                outcome: "Immediate cross or cutback from RW in 1-2 touches; box loaded within 6s",
            },
        ],
    }
}
