CREATE TABLE IF NOT EXISTS player_behavior (
    behavior_id VARCHAR(36) PRIMARY KEY,
    player_id VARCHAR(36) NOT NULL,
    feature_name VARCHAR(255),
    usage_count INT DEFAULT 0,
    last_used TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES player_stats(player_id)
);
