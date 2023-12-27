const SLOTS: &[(Slot, f32)] = &[(Slot::Feather, 1.0), (Slot::Flower, 1.0), (Slot::Sands, 1.0), (Slot::Goblet, 1.0), (Slot::Circlet, 1.0)];

// Taken from https://genshin-impact.fandom.com/wiki/Artifact/Distribution
const SANDS: &[(Stat, f32)] = &[(Stat::Atk_, 0.2668), (Stat::Hp_, 0.2668), (Stat::Def_, 0.2668), (Stat::Er, 0.1), (Stat::Em, 0.1)];
const GOBLETS: &[(Stat, f32)] = &[
    (Stat::Atk_, 0.1925),
    (Stat::Hp_, 0.1925),
    (Stat::Def_, 0.19),
    (Stat::Pyro, 0.05),
    (Stat::Electro, 0.05),
    (Stat::Cryo, 0.05),
    (Stat::Hydro, 0.05),
    (Stat::Dendro, 0.05),
    (Stat::Anemo, 0.05),
    (Stat::Geo, 0.05),
    (Stat::Physical, 0.05),
    (Stat::Em, 0.025)
];
const CIRCLETS: &[(Stat, f32)] = &[(Stat::Atk_, 0.22), (Stat::Hp_, 0.22), (Stat::Def_, 0.22), (Stat::CritRate, 0.1), (Stat::CritDam, 0.1), (Stat::Healing, 0.1), (Stat::Em, 0.04)];

const SUBS: &[(Stat, f32)] = &[(Stat::Hp, 6.0), (Stat::Atk, 6.0), (Stat::Def, 6.0), (Stat::Hp_, 4.0), (Stat::Atk_, 4.0), (Stat::Def_, 4.0), (Stat::Er, 4.0), (Stat::Em, 4.0), (Stat::CritRate, 3.0), (Stat::CritDam, 3.0)];

fn random_choice<T: Copy>(options: &[(T, f32)]) -> T {
    let total: f32 = options.iter().map(|(_, w)| *w).sum();

    let mut rand = fastrand::f32() * total;

    for (element, weight) in options.iter() {
        rand -= weight;

        if rand <= 0.0 {
            return *element;
        }
    }
    
    unreachable!()
}

pub struct Artifact {
    pub slot: Slot,
    pub set: Set,
    pub main_stat: Stat,
    pub subs: Vec<Stat>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Set {
    Main,
    Alt,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Slot {
    Feather,
    Flower,
    Sands,
    Goblet,
    Circlet,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stat {
    Hp,
    Atk,
    Def,
    Hp_,
    Atk_,
    Def_,
    Em,
    Er,
    Anemo,
    Geo,
    Dendro,
    Electro,
    Hydro,
    Pyro,
    Cryo,
    Physical,
    CritRate,
    CritDam,
    Healing,
}

pub const DOMAIN_4_START: f32 = 0.2;
pub const STRONGBOX_4_START: f32 = 0.34;

pub enum Source {
    Domain,
    Strongbox,
    Off,
}

impl Artifact {
    pub fn new_random(four_start_odds: f32, source: Source) -> Self {
        let slot = random_choice(SLOTS);

        let main_stat = match slot {
            Slot::Feather => Stat::Atk,
            Slot::Flower => Stat::Hp,
            Slot::Sands => random_choice(SANDS),
            Slot::Goblet => random_choice(GOBLETS),
            Slot::Circlet => random_choice(CIRCLETS),
        };

        let total_subs = if fastrand::f32() < four_start_odds {
            9
        } else {
            8
        };

        let subs_options: Vec<(Stat, f32)> = SUBS.iter().copied().filter(|(s, _)| s != &main_stat).collect();

        let mut subs = Vec::with_capacity(total_subs);

        for _ in 0..4 {
            subs.push(random_choice(subs_options.as_slice()));
        }

        let final_options: Vec<(Stat, f32)> = subs.iter().copied().map(|s| (s, 1.0)).collect();

        for _ in 0..(total_subs - 4) {
            subs.push(random_choice(final_options.as_slice()));
        }

        assert_eq!(total_subs, subs.len());

        let set = match source {
            Source::Domain => if fastrand::f32() < 0.5 {
                    Set::Main
                } else {
                    Set::Alt
                },
            Source::Strongbox => Set::Main,
            Source::Off => Set::Off,
        };

        Self { slot, set, main_stat, subs }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::{black_box, Bencher};

    use super::{Artifact, DOMAIN_4_START};

    #[bench]
    fn bench_artifact_generation(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10000 {
                black_box(Artifact::new_random(DOMAIN_4_START, super::Source::Domain));
            }
        });
    }
}
