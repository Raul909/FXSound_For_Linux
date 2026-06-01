import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        context = await browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = await context.new_page()

        await page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve() };")

        await page.goto("http://localhost:5173")
        await page.wait_for_timeout(2000)

        await page.screenshot(path="/home/jules/verification/screenshots/initial.png")

        eq_tracks = await page.locator(".eq-band__track").all()
        if len(eq_tracks) > 0:
            first_eq = eq_tracks[0]
            await first_eq.focus()
            await page.keyboard.press("ArrowUp")
            await page.wait_for_timeout(500)
            await page.screenshot(path="/home/jules/verification/screenshots/eq_focused.png")

        await context.close()
        await browser.close()

asyncio.run(main())
