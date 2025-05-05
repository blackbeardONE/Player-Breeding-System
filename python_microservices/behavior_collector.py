from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import databases
import sqlalchemy
import logging
import toml
import os
from python_microservices.logging_setup import setup_logging

setup_logging()

config_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "src", "config.toml"))
if not os.path.exists(config_path):
    logging.error(f"Config file not found at {config_path}")
    raise FileNotFoundError(f"Config file not found at {config_path}")
logging.info(f"Loading config from {config_path}")
config = toml.load(config_path)
database_url = config.get("database_url")

database = databases.Database(database_url)
metadata = sqlalchemy.MetaData()

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import databases
import sqlalchemy
import logging
import toml
import os
from python_microservices.logging_setup import setup_logging
from contextlib import asynccontextmanager

setup_logging()

config_path = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "src", "config.toml"))
if not os.path.exists(config_path):
    logging.error(f"Config file not found at {config_path}")
    raise FileNotFoundError(f"Config file not found at {config_path}")
logging.info(f"Loading config from {config_path}")
config = toml.load(config_path)
database_url = config.get("database_url")

database = databases.Database(database_url)
metadata = sqlalchemy.MetaData()

app = FastAPI()

class PlayerBehavior(BaseModel):
    behavior_id: str
    player_id: str
    feature_name: str
    usage_count: int = 0

player_behavior = sqlalchemy.Table(
    "player_behavior",
    metadata,
    sqlalchemy.Column("behavior_id", sqlalchemy.String(36), primary_key=True),
    sqlalchemy.Column("player_id", sqlalchemy.String(36)),
    sqlalchemy.Column("feature_name", sqlalchemy.String(255)),
    sqlalchemy.Column("usage_count", sqlalchemy.Integer),
    sqlalchemy.Column("last_used", sqlalchemy.TIMESTAMP, server_default=sqlalchemy.func.now()),
)

@asynccontextmanager
async def lifespan(app: FastAPI):
    await database.connect()
    logging.info("Database connected")
    yield
    await database.disconnect()
    logging.info("Database disconnected")

app.router.lifespan_context = lifespan

@app.post("/player_behavior/")
async def create_player_behavior(behavior: PlayerBehavior):
    query = player_behavior.insert().values(
        behavior_id=behavior.behavior_id,
        player_id=behavior.player_id,
        feature_name=behavior.feature_name,
        usage_count=behavior.usage_count,
    )
    try:
        await database.execute(query)
        return {"message": "Player behavior recorded"}
    except Exception as e:
        logging.error(f"Error inserting player behavior: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
