use crate::patterns::types::DefensivePatternForm;

pub mod back_five_defending;
pub mod box_defending;
pub mod compactness_defense;
pub mod counter_press_defense;
pub mod cover_shadow_defending;
pub mod defensive_overload;
pub mod funnel_defending;
pub mod goalkeeper_sweeper_defense;
pub mod high_press_defense;
pub mod hybrid_defending;
pub mod low_block_defense;
pub mod man_oriented_pressing;
pub mod mid_block_defense;
pub mod offside_trap;
pub mod pressing_traps;
pub mod rest_defense;
pub mod set_piece_defense;
pub mod touchline_press;
pub mod transition_defense;
pub mod zonal_defending;

pub fn all_forms() -> Vec<DefensivePatternForm> {
    vec![
        high_press_defense::form(),
        low_block_defense::form(),
        mid_block_defense::form(),
        compactness_defense::form(),
        zonal_defending::form(),
        back_five_defending::form(),
        box_defending::form(),
        cover_shadow_defending::form(),
        counter_press_defense::form(),
        funnel_defending::form(),
        defensive_overload::form(),
        hybrid_defending::form(),
        man_oriented_pressing::form(),
        offside_trap::form(),
        pressing_traps::form(),
        rest_defense::form(),
        set_piece_defense::form(),
        touchline_press::form(),
        transition_defense::form(),
        goalkeeper_sweeper_defense::form(),
    ]
}

pub fn form_by_id(id: &str) -> Option<DefensivePatternForm> {
    all_forms().into_iter().find(|f| f.id == id)
}
