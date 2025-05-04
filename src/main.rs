use env_logger::Env;
use log::{info, error};
use anyhow::Result;

mod config;
mod db;
mod agents;
mod logging;
mod communication;
mod orchestration;
mod monitoring;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::init_logging();

    info!("Starting Player Breeding System...");

    // Load configuration
    let config = config::Config::load("src/config.toml")?;
    info!("Configuration loaded.");

    // Initialize database connection
    let pool = db::init_db(&config.database_url).await?;
    info!("Database connection established.");

    // Start metrics server
    let metrics_route = monitoring::metrics_route();
    tokio::spawn(async move {
        warp::serve(metrics_route)
            .run(([0, 0, 0, 0], 9090))
            .await;
    });
    info!("Metrics server running on port 9090");

    info!("Initializing AI agents...");

    // Example orchestration run
    let orchestrator = orchestration::Orchestrator::new();
    let metadata = communication::PlayerMetadata {
        quest_logs: 12,
        character_level: 6,
        gear_system: true,
        resource_management: false,
        army_building: false,
        territory_control: false,
        crafting_system: false,
        hunger_thirst_mechanics: false,
        permadeath_features: false,
    };
    let player_wealth = 500.0;
    let behavior_data = communication::BehaviorData {
        feature_usage_counts: vec![
            ("Crafting Table".to_string(), 15),
            ("PvP Arena".to_string(), 5),
        ],
    };

    match orchestrator.run(metadata, player_wealth, behavior_data).await {
        Ok(_) => info!("Orchestration completed successfully."),
        Err(e) => error!("Orchestration failed: {:?}", e),
    }

    info!("Player Breeding System is running.");

    // For MVP, just keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, exiting.");

    Ok(())
}
