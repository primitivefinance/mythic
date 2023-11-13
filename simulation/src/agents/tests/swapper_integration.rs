use super::*;

#[tokio::test]
async fn build_swapper() {
    let simulation = startup_static().await;
    let swapper = simulation.agents;
    println!("{:?}", swapper);
}
