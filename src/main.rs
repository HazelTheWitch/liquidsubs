use std::{collections::BTreeMap, iter::repeat_with, time::Instant};

use liquidsubs::{
    artifact::{Slot, Stat},
    er_accounted_rater, example_rater, liquid_sub_rater, max_score_from_runs, simple_validator,
};

fn main() {
    let iterations = 1000;
    let mut average_scores = BTreeMap::default();

    let validator = simple_validator(
        vec![Stat::Atk_],
        vec![Stat::Electro],
        vec![Stat::CritRate, Stat::CritDam],
    );

    let mut max_scores: Vec<BTreeMap<Slot, f32>> = repeat_with(|| Default::default())
        .take(iterations)
        .collect();
    let mut max_scores_off: Vec<BTreeMap<Slot, f32>> = repeat_with(|| Default::default())
        .take(iterations)
        .collect();

    let start = Instant::now();

    let total_artifacts = 30 * 12 * iterations * 9;
    println!("Will generate {total_artifacts} artifacts");

    let mut last = 1.0;

    for days in 1..=365 {
        println!("day: {days}");
        let mut total = 0.0;

        for i in 0..iterations {
            total += max_score_from_runs(
                last,
                max_scores.get_mut(i).unwrap(),
                max_scores_off.get_mut(i).unwrap(),
                &validator,
                er_accounted_rater(0.0, |sub| match sub {
                    Stat::CritRate | Stat::CritDam => 1.4,
                    Stat::Atk_ => 1.0,
                    Stat::Em => 0.75,
                    _ => 0.0,
                }),
                9,
                false,
            );
        }

        last = total / iterations as f32;

        average_scores.insert(days, total / iterations as f32);
    }

    let duration = Instant::now() - start;

    println!("completed in {:.2} seconds", duration.as_secs_f32());

    println!("day,runs,score");
    for (key, value) in average_scores.iter() {
        println!("{key},{runs},{value}", runs = key * 9);
    }
}
