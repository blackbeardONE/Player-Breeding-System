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

class FinancialData(BaseModel):
    transaction_id: str
    player_id: str
    in_game_currency: float = 0.0
    microtransaction_amount: float = 0.0

financial_data = sqlalchemy.Table(
    "financial_data",
    metadata,
    sqlalchemy.Column("transaction_id", sqlalchemy.String(36), primary_key=True),
    sqlalchemy.Column("player_id", sqlalchemy.String(36)),
    sqlalchemy.Column("in_game_currency", sqlalchemy.Float),
    sqlalchemy.Column("microtransaction_amount", sqlalchemy.Float),
    sqlalchemy.Column("transaction_date", sqlalchemy.TIMESTAMP, server_default=sqlalchemy.func.now()),
)

@app.on_event("startup")
async def startup():
    await database.connect()
    logging.info("Database connected")

@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()
    logging.info("Database disconnected")

@app.post("/financial_data/")
async def create_financial_data(data: FinancialData):
    query = financial_data.insert().values(
        transaction_id=data.transaction_id,
        player_id=data.player_id,
        in_game_currency=data.in_game_currency,
        microtransaction_amount=data.microtransaction_amount,
    )
    try:
        await database.execute(query)
        return {"message": "Financial data recorded"}
    except Exception as e:
        logging.error(f"Error inserting financial data: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
