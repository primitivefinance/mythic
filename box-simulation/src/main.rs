use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}

/// Highest level simulation loop.
async fn run_sim_loop() -> Result<()> {
    for index in 1..100 {
        // do sim
    }

    Ok(())
}
