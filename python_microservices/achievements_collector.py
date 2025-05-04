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

class Achievement(BaseModel):
    achievement_id: str
    player_id: str
    quest_completed: str = None
    milestone_unlocked: str = None

achievements = sqlalchemy.Table(
    "achievements",
    metadata,
    sqlalchemy.Column("achievement_id", sqlalchemy.String(36), primary_key=True),
    sqlalchemy.Column("player_id", sqlalchemy.String(36)),
    sqlalchemy.Column("quest_completed", sqlalchemy.String(255)),
    sqlalchemy.Column("milestone_unlocked", sqlalchemy.String(255)),
    sqlalchemy.Column("achieved_at", sqlalchemy.TIMESTAMP, server_default=sqlalchemy.func.now()),
)

@app.on_event("startup")
async def startup():
    await database.connect()
    logging.info("Database connected")

@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()
    logging.info("Database disconnected")

@app.post("/achievements/")
async def create_achievement(achievement: Achievement):
    query = achievements.insert().values(
        achievement_id=achievement.achievement_id,
        player_id=achievement.player_id,
        quest_completed=achievement.quest_completed,
        milestone_unlocked=achievement.milestone_unlocked,
    )
    try:
        await database.execute(query)
        return {"message": "Achievement recorded"}
    except Exception as e:
        logging.error(f"Error inserting achievement: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
