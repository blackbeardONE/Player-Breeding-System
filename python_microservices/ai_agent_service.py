from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import logging

app = FastAPI()

class PlayerMetadata(BaseModel):
    quest_logs: int = 0
    character_level: int = 0
    gear_system: bool = False
    resource_management: bool = False
    army_building: bool = False
    territory_control: bool = False
    crafting_system: bool = False
    hunger_thirst_mechanics: bool = False
    permadeath_features: bool = False

@app.post("/blade_ai/classify_game_type/")
async def classify_game_type(metadata: PlayerMetadata):
    # Simple rules-based classifier as per Blade AI logic
    if (metadata.quest_logs >= 10 and
        metadata.character_level >= 5 and
        metadata.gear_system):
        game_type = "MMORPG"
    elif (metadata.resource_management and
          metadata.army_building and
          metadata.territory_control):
        game_type = "MMORTS"
    elif (metadata.crafting_system and
          metadata.hunger_thirst_mechanics and
          metadata.permadeath_features):
        game_type = "MMO Survival"
    else:
        game_type = "Unknown"

    logging.info(f"Blade AI classified game type as {game_type}")
    return {"game_type": game_type}
