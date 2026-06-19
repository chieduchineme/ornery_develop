pub mod central_attack;
pub mod combination_crossing;
pub mod combination_play;
pub mod counter_attack;
pub mod crossing_variations;
pub mod direct_attack;
pub mod fast_breaks;
pub mod high_press_attack;
pub mod isolation_attack;
pub mod overlapping_attack;
pub mod overload_attack;
pub mod positional_play;
pub mod possession_based;
pub mod rotational_attack;
pub mod set_piece_attack;
pub mod switch_of_play;
pub mod third_man_attack;
pub mod total_football;
pub mod underlapping_attack;
pub mod wing_play;

use crate::patterns::types::AttackingPatternForm;

pub fn all_forms() -> Vec<AttackingPatternForm> {
    vec![
        wing_play::form(),
        central_attack::form(),
        counter_attack::form(),
        fast_breaks::form(),
        possession_based::form(),
        direct_attack::form(),
        combination_play::form(),
        combination_crossing::form(),
        crossing_variations::form(),
        high_press_attack::form(),
        isolation_attack::form(),
        overlapping_attack::form(),
        overload_attack::form(),
        positional_play::form(),
        rotational_attack::form(),
        set_piece_attack::form(),
        switch_of_play::form(),
        third_man_attack::form(),
        total_football::form(),
        underlapping_attack::form(),
    ]
}

pub fn form_by_id(id: &str) -> Option<AttackingPatternForm> {
    all_forms().into_iter().find(|f| f.id == id)
}
