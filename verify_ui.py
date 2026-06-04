import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page(record_video_dir="/home/jules/verification/videos/")
        await page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };")
        await page.goto("http://localhost:5173")
        await page.wait_for_selector(".visualizer canvas", timeout=10000)
        await page.screenshot(path="/home/jules/verification/screenshots/visualizer.png")
        await browser.close()

asyncio.run(main())
