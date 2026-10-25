//! Shared shape for both games' completionism panels: a list of category cards, each with one or
//! two progress metrics. The per-game data and set logic live in `botw::completism` /
//! `totk::completism`; this module only defines the common return types.

/// One progress line within a category card (e.g. Koroks "217/900"). `verb` is the past-tense
/// action word used both as the row label and to build its "Set all as {verb}" button.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletismMetric {
    pub verb: &'static str,
    pub value: usize,
    pub max: usize,
}

/// A completionism category card. `id` routes its set-all actions; `metrics` is one line for most
/// categories, two for the TOTK towers/shrines/lightroots cards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletismCategory {
    pub id: &'static str,
    pub label: &'static str,
    pub metrics: Vec<CompletismMetric>,
}

pub fn metric(verb: &'static str, value: usize, max: usize) -> CompletismMetric {
    CompletismMetric { verb, value, max }
}
