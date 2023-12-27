//! Liquid subs generator thingy.
//! 
//! Stats labeled STAT_ are percent.

use std::{collections::BTreeMap, borrow::BorrowMut};

use artifact::{Artifact, Slot, Stat, Set};

use crate::artifact::{DOMAIN_4_START, Source, STRONGBOX_4_START};

pub mod artifact;

pub fn max_score_from_runs(validator: impl Fn(&Artifact) -> bool, scorer: impl Fn(&Artifact) -> f32, runs: u32, strongbox: bool) -> f32 {
    let mut max_scores: BTreeMap<Slot, f32> = Default::default();

    let total_artifacts = 0;

    for _ in 0..runs {
        let artifacts = if fastrand::f32() < 0.07 {
            2
        } else {
            1
        };

        for _ in 0..artifacts {
            let artifact = Artifact::new_random(DOMAIN_4_START, Source::Domain);

            if !validator(&artifact) {
                continue;
            }

            let score = scorer(&artifact);

            let max_score = max_scores.entry(artifact.slot).or_default().borrow_mut();

            if score > *max_score {
                *max_score = score;
            }
        }
    }

    if strongbox {
        let strongbox_artis = (total_artifacts as f32 * 1.5) as i32;

        for _ in 0..strongbox_artis {
            let artifact = Artifact::new_random(STRONGBOX_4_START, Source::Strongbox);

            if !validator(&artifact) {
                continue;
            }

            let score = scorer(&artifact);

            let max_score = max_scores.entry(artifact.slot).or_default().borrow_mut();

            if score > *max_score {
                *max_score = score;
            }
        }
    }

    max_scores.into_values().sum()
}

pub fn simple_validator(sands: Vec<Stat>, goblet: Vec<Stat>, circlet: Vec<Stat>) -> impl Fn(&Artifact) -> bool {
    move |artifact| {
        if artifact.set != Set::Main {
            return false;
        }

        match &artifact.slot {
            Slot::Feather => true,
            Slot::Flower => true,
            Slot::Sands => sands.contains(&artifact.main_stat),
            Slot::Goblet => goblet.contains(&artifact.main_stat),
            Slot::Circlet => circlet.contains(&artifact.main_stat)
        }
    }
}

pub fn example_rater(artifact: &Artifact) -> f32 {
    let mut total = 0.0;

    for sub in &artifact.subs {
        total += match sub {
            Stat::Atk_ | Stat::CritRate | Stat::CritDam => 1.0,
            Stat::Atk => 0.25,
            _ => 0.0,
        };
    }

    total
}
