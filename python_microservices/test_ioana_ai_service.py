import pytest
from fastapi.testclient import TestClient
from python_microservices.ioana_ai_service import app, call_together_ai

client = TestClient(app)

@pytest.mark.asyncio
async def test_call_together_ai(monkeypatch):
    async def mock_call(prompt):
        return "Mocked quest text"
    monkeypatch.setattr("python_microservices.ioana_ai_service.call_together_ai", mock_call)
    response = await mock_call("test prompt")
    assert response == "Mocked quest text"

def test_design_quest(monkeypatch):
    async def mock_call(prompt):
        return "Mocked quest text"
    monkeypatch.setattr("python_microservices.ioana_ai_service.call_together_ai", mock_call)
    response = client.post("/ioana_ai/design_quest/", json={
        "player_weaknesses": {"stamina": "low"},
        "preferred_activities": ["PvP", "Crafting"],
        "performance_metrics": {"DPS": 100, "SurvivalTime": 300}
    })
    assert response.status_code == 200
    assert "quest_template" in response.json()
