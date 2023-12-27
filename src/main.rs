use std::{collections::BTreeMap, time::Instant, iter::repeat_with};

use liquidsubs::{max_score_from_runs, simple_validator, artifact::{Stat, Slot}, example_rater};

fn main() {
    let iterations = 1000;
    let mut average_scores = BTreeMap::default();

    let validator = simple_validator(vec![Stat::Atk_], vec![Stat::Anemo], vec![Stat::CritDam, Stat::CritRate]);

    let mut max_scores: Vec<BTreeMap<Slot, f32>> = repeat_with(|| Default::default()).take(iterations).collect();

    let start = Instant::now();

    let total_artifacts = 30 * 12 * iterations * 9;
    println!("Will generate {total_artifacts} artifacts");

    for days in 1..=(30 * 12) {
        println!("day: {days}");
        let mut total = 0.0;

        for i in 0..iterations {
            total += max_score_from_runs(max_scores.get_mut(i).unwrap(), &validator, example_rater, 9, false);
        }

        average_scores.insert(days, total / iterations as f32);
    }

    let duration = Instant::now() - start;

    println!("completed in {:.2} seconds", duration.as_secs_f32());

    println!("day,score");
    for (key, value) in average_scores.iter() {
        println!("{key},{value}");
    }
}
