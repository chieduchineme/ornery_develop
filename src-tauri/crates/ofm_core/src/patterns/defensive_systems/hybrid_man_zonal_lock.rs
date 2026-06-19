use engine::PlayStyle;
use crate::patterns::types::{EliteDefensiveSystemDefinition, DefensiveSystemPhase};

pub fn system() -> EliteDefensiveSystemDefinition {
    EliteDefensiveSystemDefinition {
        id: "hybrid_man_zonal_lock",
        name: "Hybrid Man-Zonal Lock System",
        source_md: "Hybrid_Man_Zonal_Lock_System.md",
        structural_idea: "Stay zonal away from the ball, lock opponents near the ball, and overload the danger zone.",
        real_world_archetypes: &["Atalanta (Gasperini)", "Bayer Leverkusen (Alonso)", "Borussia Dortmund (press-heavy)"],
        base_play_style: PlayStyle::Balanced,
        phases: vec![
            DefensiveSystemPhase {
                name: "Base Shape",
                defensive_form_id: "zonal_defending",
                tactical_purpose: "Shift as connected block, protect zones",
                entry_condition: "Opponent has ball in their half",
                handoff: "Ball enters prep zone",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Local Locks",
                defensive_form_id: "man_oriented_pressing",
                tactical_purpose: "Lock receivers near ball tightly",
                entry_condition: "Ball enters near-side zone",
                handoff: "Duel won or passed",
                weight: 0.35,
            },
            DefensiveSystemPhase {
                name: "Ball-Side Superiority",
                defensive_form_id: "defensive_overload",
                tactical_purpose: "Overload danger zone around ball",
                entry_condition: "Dribbler or dangerous receiver isolated",
                handoff: "Ball cleared or recycled",
                weight: 0.30,
            },
        ],
        final_output: "Opponent cannot progress locally, forced backward, wide, or into a contested duel",
    }
}
