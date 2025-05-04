CREATE TABLE player_stats (
    player_id VARCHAR(36) PRIMARY KEY,
    game_id VARCHAR(36) NOT NULL,
    combat_efficiency FLOAT DEFAULT 0,
    resource_gather_rate FLOAT DEFAULT 0,
    survival_time INT DEFAULT 0,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
