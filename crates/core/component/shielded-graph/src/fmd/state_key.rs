pub mod parameters {
    pub fn current() -> &'static str {
        "shielded_graph/fmd_parameters/current"
    }

    pub fn previous() -> &'static str {
        "shielded_graph/fmd_parameters/previous"
    }
}

pub(crate) mod meta_parameters {
    pub fn algorithm_state() -> &'static str {
        "shielded_graph/fmd_meta_parameters/algorithm_state"
    }
}

pub(crate) mod clue_count {
    pub fn current() -> &'static str {
        "shielded_graph/fmd_clue_count/current"
    }

    pub fn previous() -> &'static str {
        "shielded_graph/fmd_clue_count/previous"
    }
}
