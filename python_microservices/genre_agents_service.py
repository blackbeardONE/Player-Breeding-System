from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import logging
import httpx
import toml
import os

app = FastAPI()

# Load config from src/config.toml using absolute path or environment variable override
config_path = os.getenv("CONFIG_PATH")
if not config_path:
    config_path = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "src", "config.toml")

try:
    config = toml.load(config_path)
except FileNotFoundError:
    logging.error(f"Config file not found at {config_path}. Please ensure the file exists.")
    raise

TOGETHER_API_URL = config.get("together_ai", {}).get("api_url", "")
TOGETHER_API_KEY = config.get("together_ai", {}).get("api_key", "")
MODEL_NAME = config.get("together_ai", {}).get("model_name", "")

class PlayerWealthData(BaseModel):
    player_wealth: float

async def call_together_ai(prompt: str) -> float:
    headers = {
        "Authorization": f"Bearer {TOGETHER_API_KEY}",
        "Content-Type": "application/json"
    }
    json_data = {
        "model": MODEL_NAME,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 50
    }
    async with httpx.AsyncClient() as client:
        response = await client.post(TOGETHER_API_URL, headers=headers, json=json_data)
        response.raise_for_status()
        data = response.json()
        # Parse the response text to extract a float drop rate adjustment
        content = data["choices"][0]["message"]["content"]
        try:
            # Expecting a numeric value in the response
            return float(content.strip())
        except ValueError:
            logging.error(f"Failed to parse drop rate adjustment from response: {content}")
            return 1.0  # default no adjustment

@app.post("/genre_agents/adjust_drop_rate/")
async def adjust_drop_rate(data: PlayerWealthData):
    try:
        prompt = f"Given a player wealth of {data.player_wealth}, suggest a drop rate adjustment multiplier."
        adjusted_drop_rate = await call_together_ai(prompt)
        logging.info(f"Adjusted drop rate to {adjusted_drop_rate} based on player wealth {data.player_wealth}")
        return {"adjusted_drop_rate": adjusted_drop_rate}
    except Exception as e:
        logging.error(f"Error adjusting drop rate: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
