import asyncio
from playwright.async_api import async_playwright

async def run():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        context = await browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = await context.new_page()
        await page.add_init_script("""
            window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };
        """)
        await page.goto("http://localhost:5173")
        await page.wait_for_timeout(2000)

        # Test keyboard navigation for EQ Band
        eq_band = page.locator(".eq-band__track").first
        await eq_band.focus()
        await page.keyboard.press("ArrowUp")
        await page.keyboard.press("ArrowUp")
        await page.wait_for_timeout(500)

        await page.screenshot(path="/home/jules/verification/screenshots/eq-band-focus.png")

        await context.close()
        await browser.close()

asyncio.run(run())
