import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()

        # Start recording video
        context = await browser.new_context(record_video_dir="videos/")
        page = await context.new_page()

        await page.goto("http://localhost:8080")

        # Wait for app to load
        await page.wait_for_selector(".app-window")

        # --- Test EQ Sliders ---
        print("Testing EQ Band drag...")
        # Get the 125 Hz band thumb
        eq_thumb = page.locator(".eq-band:nth-child(2) .eq-band__thumb").first
        await eq_thumb.wait_for(state='visible')

        box = await eq_thumb.bounding_box()
        if box:
            # Click and drag the thumb down
            await page.mouse.move(box["x"] + box["width"] / 2, box["y"] + box["height"] / 2)
            await page.mouse.down()
            # Move mouse up to increase gain
            await page.mouse.move(box["x"] + box["width"] / 2, box["y"] - 50, steps=10)
            await page.mouse.up()
            print("EQ Band dragged successfully.")

        # Take screenshot of EQ change
        await page.screenshot(path="screenshot_eq.png")

        # --- Test Effect Sliders ---
        print("Switching to EFFECTS tab...")
        effects_tab = page.get_by_text("EFFECTS")
        await effects_tab.click()

        # Wait for the first effect slider thumb to be visible
        effect_thumb = page.locator(".effect-slider .effect-slider__thumb").first
        await effect_thumb.wait_for(state='visible')

        print("Testing Effect Slider drag...")
        box = await effect_thumb.bounding_box()
        if box:
            # Click and drag the thumb to the right
            await page.mouse.move(box["x"] + box["width"] / 2, box["y"] + box["height"] / 2)
            await page.mouse.down()
            # Move mouse right to increase effect
            await page.mouse.move(box["x"] + 150, box["y"] + box["height"] / 2, steps=10)
            await page.mouse.up()
            print("Effect Slider dragged successfully.")

        # Take screenshot of Effect change
        await page.screenshot(path="screenshot_effect.png")

        await context.close()
        await browser.close()

asyncio.run(main())
