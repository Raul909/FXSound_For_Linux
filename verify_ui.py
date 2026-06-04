import asyncio
from playwright.async_api import async_playwright

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        context = await browser.new_context(
            record_video_dir="/home/jules/verification/videos/"
        )
        page = await context.new_page()

        await page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };")
        await page.goto("http://localhost:5173")

        eq_band = page.locator('.eq-band__track').first
        await eq_band.wait_for()
        await eq_band.focus()
        await eq_band.press("ArrowUp")
        await eq_band.press("ArrowUp")
        await eq_band.press("ArrowUp")

        await page.wait_for_timeout(500)
        await page.screenshot(path="/home/jules/verification/screenshots/eq_slider_focus.png")
        await browser.close()

asyncio.run(run())
