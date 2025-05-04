import pytest
from fastapi.testclient import TestClient
from python_microservices.genre_agents_service import app, call_together_ai

client = TestClient(app)

@pytest.mark.asyncio
async def test_call_together_ai(monkeypatch):
    async def mock_call(prompt):
        return 1.23
    monkeypatch.setattr("python_microservices.genre_agents_service.call_together_ai", mock_call)
    response = await mock_call("test prompt")
    assert response == 1.23

def test_adjust_drop_rate(monkeypatch):
    async def mock_call(prompt):
        return 1.23
    monkeypatch.setattr("python_microservices.genre_agents_service.call_together_ai", mock_call)
    response = client.post("/genre_agents/adjust_drop_rate/", json={
        "player_wealth": 1500.0
    })
    assert response.status_code == 200
    assert "adjusted_drop_rate" in response.json()
