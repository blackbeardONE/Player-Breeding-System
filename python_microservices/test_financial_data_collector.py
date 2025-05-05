import sys
import os
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

import pytest_asyncio
from python_microservices.financial_data_collector import app
from httpx import AsyncClient

@pytest_asyncio.fixture
async def client():
    async with AsyncClient(app=app, base_url="http://test") as client:
        yield client

@pytest.mark.asyncio
async def test_create_financial_data_success(client):
    response = await client.post("/financial_data/", json={
        "transaction_id": "txn1",
        "player_id": "player1",
        "in_game_currency": 100.0,
        "microtransaction_amount": 5.0
    })
    assert response.status_code == 200
    assert response.json() == {"message": "Financial data recorded"}

@pytest.mark.asyncio
async def test_create_financial_data_missing_field(client):
    response = await client.post("/financial_data/", json={
        "transaction_id": "txn2"
    })
    assert response.status_code == 422  # Unprocessable Entity due to missing required fields
