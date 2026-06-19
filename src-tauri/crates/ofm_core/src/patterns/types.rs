use engine::PlayStyle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerRole {
    Gk,
    Cb,
    BallSideCb,
    FarSideCb,
    Lb,
    Rb,
    Dm,
    Cm,
    Lcm,
    Rcm,
    Am,
    Lw,
    Rw,
    St,
    TargetSt,
    BallCarrier,
    DecoyRunner,
    ScoringRunner,
    Lwb,
    Rwb,
    Lcb,
    Rcb,
    Swk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementDirection {
    Hold,
    Drop,
    Advance,
    SprintForward,
    Overlap,
    Underlap,
    Invert,
    DriftWide,
    DriftInside,
    DiagonalRun,
    BlindsideRun,
    Rotate,
    Press,
    CounterBalance,
    AttackNearPost,
    AttackFarPost,
    AttackCutbackZone,
    Tuck,
    MarkRunner,
    StepUp,
    SqueezeUp,
    RecoverShape,
    CoverChannel,
    ShieldGoal,
    BlockLane,
    DoubleTeam,
    TrackBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallAction {
    None,
    Receive,
    Carry,
    Dribble,
    OneTouchPass,
    BouncePass,
    ThroughBall,
    DiagonalSwitch,
    Cross,
    Cutback,
    Shoot,
    PressingRecovery,
    Layoff,
    LongPass,
    Recycle,
    ScreenRestDefense,
    Intercept,
    Tackle,
    Header,
    Clearance,
    BlockShot,
    PunchClear,
    FoulTactical,
    CallOffsideTrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intensity {
    Walk,
    Jog,
    Accelerate,
    Sprint,
    Explosive,
}

#[derive(Debug, Clone, Copy)]
pub struct MovementSpec {
    pub direction: MovementDirection,
    pub distance_m: f32,
    pub start_second: f32,
    pub duration_seconds: f32,
    pub intensity: Intensity,
}

#[derive(Debug, Clone)]
pub struct PlayerInstruction {
    pub role: PlayerRole,
    pub movement: MovementSpec,
    pub action: BallAction,
    pub target: &'static str,
    pub physics_detail: &'static str,
}

#[derive(Debug, Clone)]
pub struct PatternPhase {
    pub name: &'static str,
    pub trigger: &'static str,
    pub tempo_seconds: (f32, f32),
    pub width_m: f32,
    pub depth_m: f32,
    pub instructions: Vec<PlayerInstruction>,
    pub outcome: &'static str,
}

#[derive(Debug, Clone)]
pub struct AttackingPatternForm {
    pub id: &'static str,
    pub name: &'static str,
    pub source_md: &'static str,
    pub base_formation: &'static str,
    pub preferred_play_style: PlayStyle,
    pub risk: f32,
    pub reward: f32,
    pub phases: Vec<PatternPhase>,
}

#[derive(Debug, Clone)]
pub struct SystemPhase {
    pub name: &'static str,
    pub attacking_form_id: &'static str,
    pub tactical_purpose: &'static str,
    pub entry_condition: &'static str,
    pub handoff: &'static str,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct EliteAttackingSystemDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub source_md: &'static str,
    pub structural_idea: &'static str,
    pub real_world_archetypes: &'static [&'static str],
    pub base_play_style: PlayStyle,
    pub phases: Vec<SystemPhase>,
    pub final_output: &'static str,
}

pub fn movement(
    direction: MovementDirection,
    distance_m: f32,
    start_second: f32,
    duration_seconds: f32,
    intensity: Intensity,
) -> MovementSpec {
    MovementSpec {
        direction,
        distance_m,
        start_second,
        duration_seconds,
        intensity,
    }
}

pub fn instruction(
    role: PlayerRole,
    movement: MovementSpec,
    action: BallAction,
    target: &'static str,
    physics_detail: &'static str,
) -> PlayerInstruction {
    PlayerInstruction {
        role,
        movement,
        action,
        target,
        physics_detail,
    }
}

#[derive(Debug, Clone)]
pub struct DefensivePlayerInstruction {
    pub role: PlayerRole,
    pub movement: MovementSpec,
    pub action: BallAction,
    pub marks: &'static str,
    pub physics_detail: &'static str,
}

#[derive(Debug, Clone)]
pub struct DefensivePatternPhase {
    pub name: &'static str,
    pub trigger: &'static str,
    pub tempo_seconds: (f32, f32),
    pub block_height_pct: f32,
    pub instructions: Vec<DefensivePlayerInstruction>,
    pub outcome: &'static str,
}

#[derive(Debug, Clone)]
pub struct DefensivePatternForm {
    pub id: &'static str,
    pub name: &'static str,
    pub source_md: &'static str,
    pub base_formation: &'static str,
    pub preferred_play_style: PlayStyle,
    pub compactness: f32,
    pub aggression: f32,
    pub phases: Vec<DefensivePatternPhase>,
}

#[derive(Debug, Clone)]
pub struct DefensiveSystemPhase {
    pub name: &'static str,
    pub defensive_form_id: &'static str,
    pub tactical_purpose: &'static str,
    pub entry_condition: &'static str,
    pub handoff: &'static str,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct EliteDefensiveSystemDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub source_md: &'static str,
    pub structural_idea: &'static str,
    pub real_world_archetypes: &'static [&'static str],
    pub base_play_style: PlayStyle,
    pub phases: Vec<DefensiveSystemPhase>,
    pub final_output: &'static str,
}

pub fn def_instruction(
    role: PlayerRole,
    movement: MovementSpec,
    action: BallAction,
    marks: &'static str,
    physics_detail: &'static str,
) -> DefensivePlayerInstruction {
    DefensivePlayerInstruction { role, movement, action, marks, physics_detail }
}