import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        context = await browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = await context.new_page()

        await page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };")
        await page.goto("http://localhost:5173")

        await page.wait_for_selector(".eq-band__track")
        await page.focus(".eq-band__track")
        await page.keyboard.press("ArrowUp")
        await page.screenshot(path="/home/jules/verification/screenshots/eq_tab.png")

        await context.close()
        await browser.close()

asyncio.run(main())
