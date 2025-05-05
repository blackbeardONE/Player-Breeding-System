import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import pytest_asyncio
from python_microservices.data_collector import app, database
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
async def test_create_player_stats_success(client):
    response = await client.post("/player_stats/", json={
        "player_id": "player1",
        "game_id": "game1",
        "combat_efficiency": 75.5,
        "resource_gather_rate": 10.0,
        "survival_time": 120
    })
    assert response.status_code == 200
    assert response.json() == {"message": "Player stats recorded"}

@pytest.mark.asyncio
async def test_create_player_stats_missing_field(client):
    response = await client.post("/player_stats/", json={
        "player_id": "player2",
        "combat_efficiency": 50.0
    })
    assert response.status_code == 422  # Unprocessable Entity due to missing required fields
