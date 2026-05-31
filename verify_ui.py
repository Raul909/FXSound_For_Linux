import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        await page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };")
        await page.goto("http://localhost:5173")
        await page.wait_for_selector(".eq-band__track")
        await page.keyboard.press("Tab")
        await page.keyboard.press("Tab")
        await page.keyboard.press("Tab")
        await page.keyboard.press("ArrowUp")
        await page.screenshot(path="/home/jules/verification/screenshots/focus_state.png")
        await browser.close()

asyncio.run(main())
