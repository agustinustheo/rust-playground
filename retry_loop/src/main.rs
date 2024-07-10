use std::time::{Duration, Instant};
use tokio;

async fn potentially_failing_function() -> Result<(), &'static str> {
    // Simulate a function that might fail
    if rand::random() {
        Ok(())
    } else {
        Err("Simulated error")
    }
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let max_duration = Duration::from_secs(10);

    tokio::spawn(async move {
        while Instant::now().duration_since(start_time) < max_duration {
            match potentially_failing_function().await {
                Ok(_) => {
                    println!("Function succeeded");
                }
                Err(e) => {
                    println!("Function failed: {}", e);
                    // Instead of returning, we'll continue the loop
                    // This ensures the sleep is always evaluated
                    return;
                }
            }

            // This sleep will now always be evaluated
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Slept for 1 second");
        }
        println!("Loop finished");
    });

    // Wait for the spawned task to complete
    tokio::time::sleep(Duration::from_secs(11)).await;
}
