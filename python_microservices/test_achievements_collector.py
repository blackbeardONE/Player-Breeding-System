import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import pytest
import pytest_asyncio
from python_microservices.achievements_collector import app, database
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
async def test_create_achievement_success(client):
    response = await client.post("/achievements/", json={
        "achievement_id": "achv1",
        "player_id": "player1",
        "quest_completed": "quest1",
        "milestone_unlocked": "milestone1"
    })
    assert response.status_code == 200
    assert response.json() == {"message": "Achievement recorded"}

@pytest.mark.asyncio
async def test_create_achievement_missing_field(client):
    response = await client.post("/achievements/", json={
        "achievement_id": "achv2"
    })
    assert response.status_code == 422  # Unprocessable Entity due to missing required fields
