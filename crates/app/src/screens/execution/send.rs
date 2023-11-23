//! Handles the logic for executing simulated and live transactions.

use super::*;

#[tracing::instrument(skip(scroll, forker))]
pub async fn handle_simulate_scroll(
    scroll: Scroll,
    forker: Arc<tokio::sync::Mutex<Forker>>,
) -> anyhow::Result<Scroll, anyhow::Error> {
    let mut scroll = scroll.clone();

    let locked = forker.lock().await;

    // Get the block number and load it into the forker.
    let block = locked.load_block_number().await?;
    tracing::debug!("Loaded block number: {}", block);

    // Simulate the tx.
    let _ = scroll.simulate(&locked, Some(block)).await?;

    tracing::debug!(
        "Simulated tx, before storage: {:?}",
        scroll.stages.before.clone()
    );

    Ok(scroll)
}

#[tracing::instrument(skip(scroll, forker))]
pub async fn handle_execute_scroll(
    scroll: Scroll,
    forker: Arc<tokio::sync::Mutex<Forker>>,
) -> anyhow::Result<Scroll, anyhow::Error> {
    let mut scroll = scroll.clone();

    let locked = forker.lock().await;
    let block = locked.load_block_number().await?;

    let res = scroll.execute(&locked, Some(block)).await?;

    tracing::debug!("Executed tx: {:?}", res);

    // Loading into db.
    let block_number = locked.load_block_number().await?;

    scroll.load_after(&locked, Some(block_number))?;

    Ok(scroll)
}
