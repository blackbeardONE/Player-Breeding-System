from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import logging
import httpx
import toml

app = FastAPI()

# Load config from src/config.toml
config = toml.load("src/config.toml")
TOGETHER_API_URL = config.get("together_ai", {}).get("api_url", "")
TOGETHER_API_KEY = config.get("together_ai", {}).get("api_key", "")
MODEL_NAME = config.get("together_ai", {}).get("model_name", "")

class QuestInput(BaseModel):
    player_weaknesses: dict  # e.g., {"stamina": "low"}
    preferred_activities: list  # e.g., ["PvP", "Crafting"]
    performance_metrics: dict  # e.g., {"DPS": 100, "SurvivalTime": 300}

async def call_together_ai(prompt: str) -> str:
    headers = {
        "Authorization": f"Bearer {TOGETHER_API_KEY}",
        "Content-Type": "application/json"
    }
    json_data = {
        "model": MODEL_NAME,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 150
    }
    async with httpx.AsyncClient() as client:
        response = await client.post(TOGETHER_API_URL, headers=headers, json=json_data)
        response.raise_for_status()
        data = response.json()
        return data["choices"][0]["message"]["content"]

@app.post("/ioana_ai/design_quest/")
async def design_quest(input_data: QuestInput):
    try:
        # Construct prompt for Together AI
        prompt = f"Generate personalized quests for a player with weaknesses {input_data.player_weaknesses}, preferred activities {input_data.preferred_activities}, and performance metrics {input_data.performance_metrics}."
        quest_text = await call_together_ai(prompt)

        logging.info(f"Ioana AI designed quest plan: {quest_text}")
        return {"quest_template": quest_text}
    except Exception as e:
        logging.error(f"Error designing quest: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
