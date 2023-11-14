use super::*;

#[tokio::test]
async fn build_swapper() {
    let simulation = startup_static().await;
    let agents = simulation.agents;
    let swapper = agents.0.get("swapper").unwrap();
    println!("swapper: {:?}", swapper);
}
