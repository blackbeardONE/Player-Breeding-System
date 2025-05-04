CREATE TABLE financial_data (
    transaction_id VARCHAR(36) PRIMARY KEY,
    player_id VARCHAR(36) NOT NULL,
    in_game_currency FLOAT DEFAULT 0,
    microtransaction_amount FLOAT DEFAULT 0,
    transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES player_stats(player_id)
);
