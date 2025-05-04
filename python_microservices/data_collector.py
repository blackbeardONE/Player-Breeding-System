from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import databases
import sqlalchemy
import logging
import toml
import os
from python_microservices.logging_setup import setup_logging

setup_logging()

config_path = os.getenv("CONFIG_PATH")
if not config_path:
    config_path = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "src", "config.toml")

logging.info(f"Resolved config path: {config_path}")

if not os.path.exists(config_path):
    logging.error(f"Config file not found at {config_path}")
    raise FileNotFoundError(f"Config file not found at {config_path}")

logging.info(f"Loading config from {config_path}")
config = toml.load(config_path)
database_url = config.get("database_url")

database = databases.Database(database_url)
metadata = sqlalchemy.MetaData()

app = FastAPI()

class PlayerStats(BaseModel):
    player_id: str
    game_id: str
    combat_efficiency: float = 0.0
    resource_gather_rate: float = 0.0
    survival_time: int = 0

player_stats = sqlalchemy.Table(
    "player_stats",
    metadata,
    sqlalchemy.Column("player_id", sqlalchemy.String(36), primary_key=True),
    sqlalchemy.Column("game_id", sqlalchemy.String(36)),
    sqlalchemy.Column("combat_efficiency", sqlalchemy.Float),
    sqlalchemy.Column("resource_gather_rate", sqlalchemy.Float),
    sqlalchemy.Column("survival_time", sqlalchemy.Integer),
)

@app.on_event("startup")
async def startup():
    await database.connect()
    logging.info("Database connected")

@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()
    logging.info("Database disconnected")

@app.post("/player_stats/")
async def create_player_stats(stats: PlayerStats):
    query = player_stats.insert().values(
        player_id=stats.player_id,
        game_id=stats.game_id,
        combat_efficiency=stats.combat_efficiency,
        resource_gather_rate=stats.resource_gather_rate,
        survival_time=stats.survival_time,
    )
    try:
        await database.execute(query)
        return {"message": "Player stats recorded"}
    except Exception as e:
        logging.error(f"Error inserting player stats: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
