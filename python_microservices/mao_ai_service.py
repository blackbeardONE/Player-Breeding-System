from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from ml_models import BehaviorClustering
import numpy as np
import logging

app = FastAPI()

# Initialize behavior clustering model
behavior_model = BehaviorClustering(n_clusters=3)

class PlayerBehaviorData(BaseModel):
    feature_usage_counts: dict  # e.g., {"PvP Arena": 10, "Auction House": 5}

@app.post("/mao_ai/cluster_player/")
async def cluster_player(data: PlayerBehaviorData):
    try:
        features = list(data.feature_usage_counts.keys())
        counts = list(data.feature_usage_counts.values())
        X = np.array(counts).reshape(1, -1)
        cluster = behavior_model.predict(X)[0]
        logging.info(f"Player clustered into profile {cluster} based on behavior {data.feature_usage_counts}")
        return {"cluster": int(cluster)}
    except Exception as e:
        logging.error(f"Error clustering player behavior: {e}")
        raise HTTPException(status_code=500, detail="Internal Server Error")
