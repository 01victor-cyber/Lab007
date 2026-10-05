use rand::RngExt;

const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

fn average_temp(log: &Vec<i32>) -> f64 {
    if log.is_empty() {
       return 0.0;
 }
 let sum: i32 = log.iter().sum();
 sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<i32>) -> usize {
    if log.is_empty() {
       return 0;
 }
 let mut max_idx = 0;
 for (i, &temp) in log.iter().enumerate() {
     if temp > log[max_idx] {
         max_idx = i;
        }
   }
   max_idx
}

fn count_above(log: &Vec<i32>, threshold: i32) -> usize {
    log.iter().filter(|&&temp| temp > threshold).count()
}

fn main() {
    let mut highs: Vec<i32> = vec![72, 68, 75, 81, 79];

    println!("Initial temperatures:");
    for (i, &temp) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], temp);
    }

    // Add two random temperatures in 60..=100
    let mut rng = rand::rng();
    highs.push(rng.random_range(60..=100));
    highs.push(rng.random_range(60..=100));

    println!("\nFull 7-day forecast:");
    for (i, &temp) in highs.iter().enumerate() {
        println!("{}: {}", DAYS[i], temp);
    }
    let avg = average_temp(&highs);
    println!("\nAverage temperature: {:.2}°F", avg);

    let hottest_idx = hottest_day(&highs);
    println!("Hottest day: {} ({}°F)", DAYS[hottest_idx], highs[hottest_idx]);

    let threshold = 75;
    let above_threshold_count = count_above(&highs, threshold);
    println!("Days exceeding {}°F: {}", threshold, above_threshold_count);
}



