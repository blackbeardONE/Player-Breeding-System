import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import pytest
import pytest_asyncio
from python_microservices.behavior_collector import app, database
from httpx import AsyncClient

@pytest_asyncio.fixture(scope="module")
async def db():
    await database.connect()
    yield
    await database.disconnect()

@pytest_asyncio.fixture
async def client(db):
    async with AsyncClient(app=app, base_url="http://test") as client:
        yield client

@pytest.mark.asyncio
async def test_create_player_behavior_success(client):
    response = await client.post("/player_behavior/", json={
        "behavior_id": "behv1",
        "player_id": "player1",
        "feature_name": "auction_house",
        "usage_count": 10
    })
    assert response.status_code == 200
    assert response.json() == {"message": "Player behavior recorded"}

@pytest.mark.asyncio
async def test_create_player_behavior_missing_field(client):
    response = await client.post("/player_behavior/", json={
        "behavior_id": "behv2"
    })
    assert response.status_code == 422  # Unprocessable Entity due to missing required fields
