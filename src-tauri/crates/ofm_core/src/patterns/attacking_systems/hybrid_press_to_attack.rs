use engine::PlayStyle;
use crate::patterns::types::{EliteAttackingSystemDefinition, SystemPhase};

pub fn system() -> EliteAttackingSystemDefinition {
    EliteAttackingSystemDefinition {
        id: "hybrid_press_to_attack",
        name: "Hybrid Press-to-Attack System",
        source_md: "Hybrid_Press_to_Attack_System.md",
        structural_idea: "Force mistake with coordinated high press → immediately convert turnover into vertical attack → third-man run completes",
        real_world_archetypes: &["Liverpool (Klopp gegenpressing)", "Napoli (Spalletti)", "Brighton (De Zerbi)"],
        base_play_style: PlayStyle::HighPress,
        phases: vec![
            SystemPhase {
                name: "Coordinated High Press",
                attacking_form_id: "high_press_attack",
                tactical_purpose: "Coordinated pressing traps force predictable pass; RW presses CB cutting off GK outlet",
                entry_condition: "Opponent CB or GK in possession in their own half; press trigger signal given",
                handoff: "Ball won high — immediately play vertical without hesitation; no consolidation phase",
                weight: 0.35,
            },
            SystemPhase {
                name: "Immediate Transition Attack",
                attacking_form_id: "fast_breaks",
                tactical_purpose: "Instant verticality on ball recovery; DM plays ST who plays LW at maximum speed",
                entry_condition: "High press win within 25m of opponent goal; attackers already in advanced positions from press",
                handoff: "Ball in final third within 2 passes; identify third-man run option",
                weight: 0.30,
            },
            SystemPhase {
                name: "Third-Man Run Final Penetration",
                attacking_form_id: "third_man_attack",
                tactical_purpose: "ST receives and lays to LCM who feeds RW blind-side run; defense caught mid-recovery",
                entry_condition: "Fast break reaches final third; one attacker available for third-man role in blind spot",
                handoff: "Shot within 6-10 seconds of original turnover",
                weight: 0.35,
            },
        ],
        final_output: "Shot within 6-10 seconds of press turnover; defender still recovering from being pressed",
    }
}
