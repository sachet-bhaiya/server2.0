from fastapi import FastAPI
from typing import Optional

app = FastAPI()

screenshot_data: Optional[bytes] = None

@app.get("/")
def read_root():
    return {"message": "screen share server"}

@app.post("/screenshot")
async def receive_screenshot(data: bytes):
    global screenshot_data
    screenshot_data = data
    return "uploaded"

@app.get("/screenshot")
def get_screenshot():
    if screenshot_data:
        return screenshot_data
    else:
        return "error"
