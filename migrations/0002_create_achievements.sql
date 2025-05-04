CREATE TABLE achievements (
    achievement_id VARCHAR(36) PRIMARY KEY,
    player_id VARCHAR(36) NOT NULL,
    quest_completed VARCHAR(255),
    milestone_unlocked VARCHAR(255),
    achieved_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES player_stats(player_id)
);
