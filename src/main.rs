use rand::RngExt;



   struct Reading { 
      day: String,
      high: i32,
}
   struct Summary {
      average: f64,
      hottest_day: String,
      days_above: usize,
}

fn average_temp(log: &Vec<Reading>) -> f64 {
    if log.is_empty() {
       return 0.0;
 }
 let sum: i32 = log.iter().map(|r| r.high).sum();
 sum as f64 / log.len() as f64
}

fn hottest_day(log: &Vec<Reading>) -> usize {
    if log.is_empty() {
       return 0;
 }
 let mut max_idx = 0;
 for (i, reading) in log.iter().enumerate() {
     if reading.high > log[max_idx].high {
         max_idx = i;
        }
   }
   max_idx
}

fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    log.iter().filter(|r| r.high > threshold).count()
}

fn summarize(log: &Vec<Reading>, threshold: i32) -> Summary {
    if log.is_empty() {
        return Summary {
            average: 0.0,
            hottest_day: String::from("N/A"),
            days_above: 0,
        };
    }
    let avg = average_temp(log);
    let peak_idx = hottest_day(log);
    let hottest_label = log[peak_idx].day.clone();
    let count = count_above(log, threshold);
 
    Summary {
        average: avg,
        hottest_day: hottest_label,
        days_above: count,
    }

}
      
fn main() {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let initial_highs = vec![72, 68, 75, 81, 79];

    let mut log: Vec<Reading> = Vec::new();

    // Populate log with initial readings
    for (i, &high) in initial_highs.iter().enumerate() {
        log.push(Reading {
            day: days[i].to_string(),
            high,
        });
    }

    // Add two random days for Sat/Sun
    let mut rng = rand::rng();
    log.push(Reading {
        day: days[5].to_string(),
        high: rng.random_range(60..=100),
    });
    log.push(Reading {
        day: days[6].to_string(),
        high: rng.random_range(60..=100),
    });

    // Print full 7-day forecast
    println!("Full 7-day forecast:");
    for reading in &log {
        println!("{}: {}", reading.day, reading.high);
    }

    // Calculate summary
    let threshold = 75;
    let summary = summarize(&log, threshold);

    println!("\n--- Temperature Summary ---");
    println!("Average Temperature: {:.2}°F", summary.average);
    println!("Hottest Day: {}", summary.hottest_day);
    println!("Days Above {}°F: {}", threshold, summary.days_above);
}




