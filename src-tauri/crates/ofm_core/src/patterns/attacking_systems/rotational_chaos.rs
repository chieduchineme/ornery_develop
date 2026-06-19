use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "rotational_chaos",
        name: "Rotational Chaos Breakdown System",
        source_md: "Rotational_Chaos_System.md",
        structural_idea: "Destroy marking systems via constant role exchange; create unmarked third-man runs from positional chaos",
        real_world_archetypes: &["Ajax (van Gaal / modern era)", "Netherlands national team", "Bayer Leverkusen (Alonso)"],
        base_play_style: PlayStyle::Attacking,
        phases: vec![
            SystemPhase {
                name: "Structural Role Swap",
                attacking_form_id: "rotational_attack",
                tactical_purpose: "LW-LCM-LB continuously exchange zones; man-markers lose track of assignments within 10-15 seconds",
                entry_condition: "Team in settled possession in middle or final third; opponent using man-marking",
                handoff: "Once defensive shape is distorted, expand fluidity to all 10 outfield players",
                weight: 0.35,
            },
            SystemPhase {
                name: "Total Fluidity Expansion",
                attacking_form_id: "total_football",
                tactical_purpose: "Every player interchangeable; CB steps into midfield, ST drops to CM — continuous self-reorganization",
                entry_condition: "Rotation phase has disrupted defensive structure; opponent cannot restore shape",
                handoff: "Chaos creates one clearly unmarked player — trigger third-man run into vacated space",
                weight: 0.30,
            },
            SystemPhase {
                name: "Third-Man Exploitation",
                attacking_form_id: "third_man_attack",
                tactical_purpose: "Exploit the blind-side run opened by rotation; third man arrives unmarked from chaos",
                entry_condition: "Defensive structure collapsed; at least one channel behind defensive line unguarded",
                handoff: "Unpredictable final pass → shot on goal within 2-3 seconds of third-man reception",
                weight: 0.35,
            },
        ],
        final_output: "Unpredictable final pass to third-man arrival → goal; defender cannot identify runner due to rotation chaos",
    }
}
