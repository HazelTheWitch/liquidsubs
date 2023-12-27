#![feature(test)]

//! Liquid subs generator thingy.
//! 
//! Stats labeled STAT_ are percent.

use std::{collections::BTreeMap, borrow::BorrowMut};

use artifact::{Artifact, Slot, Stat, Set};

use crate::artifact::{DOMAIN_4_START, Source, STRONGBOX_4_START};

pub mod artifact;

pub fn max_score_from_runs(max_scores: &mut BTreeMap<Slot, f32>, max_scores_off: &mut BTreeMap<Slot, f32>, validator: impl Fn(&Artifact) -> bool, scorer: impl Fn(&Artifact) -> f32, runs: u32, strongbox: bool) -> f32 {
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
        let strongbox_artis = total_artifacts as f32 * 1.5;

        let total_strongbox_artis = strongbox_artis as i32 + if fastrand::f32() < strongbox_artis.fract() { 1 } else { 0 };

        for _ in 0..total_strongbox_artis {
            let artifact = Artifact::new_random(STRONGBOX_4_START, Source::Strongbox);

            if !validator(&artifact) {
                continue;
            }

            let score = scorer(&artifact);

            let max_score = match artifact.set {
                Set::Main => max_scores.entry(artifact.slot).or_default().borrow_mut(),
                Set::Alt | Set::Off => max_scores_off.entry(artifact.slot).or_default().borrow_mut(),
            };

            if score > *max_score {
                *max_score = score;
            }
        }
    }

    let best_off = max_scores.iter()
        .map(|(k, s)| {
            let off = max_scores_off.get(&k).copied().unwrap_or_default();
            if off > *s {
                off - s
            } else {
                0.0
            }
        })
        .max_by(|s1, s2| s1.partial_cmp(s2).unwrap());

    let mut total: f32 = max_scores.values().copied().sum();

    if let Some(diff) = best_off {
        total += diff;
    }

    total
}

pub fn simple_validator(sands: Vec<Stat>, goblet: Vec<Stat>, circlet: Vec<Stat>) -> impl Fn(&Artifact) -> bool {
    move |artifact| {
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
