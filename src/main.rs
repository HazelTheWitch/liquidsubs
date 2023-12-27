use std::{collections::BTreeMap, time::Instant};

use liquidsubs::{max_score_from_runs, simple_validator, artifact::Stat, example_rater};

fn main() {
    let iterations = 1000;
    let mut average_scores = BTreeMap::default();

    let validator = simple_validator(vec![Stat::Atk_], vec![Stat::Anemo], vec![Stat::CritDam, Stat::CritRate]);

    let start = Instant::now();

    let total_artifacts: i32 = (1..=30).map(|i| i * iterations * 9).sum();
    println!("Will generate {total_artifacts} artifacts");

    for days in 1..=30 {
        println!("day: {days}");
        let total_runs = 9 * days;
        let mut total = 0.0;

        for _ in 0..iterations {
            total += max_score_from_runs(&validator, example_rater, total_runs, false);
        }

        average_scores.insert(days, total / iterations as f32);
    }

    let duration = Instant::now() - start;

    println!("completed in {:.2} seconds", duration.as_secs_f32());

    println!("{average_scores:#?}");
}
