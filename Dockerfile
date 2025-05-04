# Use official Python runtime as a parent image
FROM python:3.10-slim

# Set working directory
WORKDIR /app

# Copy requirements and install dependencies
COPY requirements.txt /app/requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# Copy microservices code
COPY python_microservices /app

# Expose default FastAPI port
EXPOSE 8000

# Default command to run, can be overridden in docker-compose
CMD ["uvicorn", "data_collector:app", "--host", "0.0.0.0", "--port", "8000"]
