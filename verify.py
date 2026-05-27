from playwright.sync_api import sync_playwright
import os

os.makedirs('/home/jules/verification/screenshots', exist_ok=True)
os.makedirs('/home/jules/verification/videos', exist_ok=True)

with sync_playwright() as p:
    browser = p.chromium.launch()
    context = browser.new_context(record_video_dir='/home/jules/verification/videos/')
    page = context.new_page()
    page.goto('http://localhost:5173')

    page.wait_for_selector('.eq-band__track')

    # Press tab to focus the first EQ band
    page.keyboard.press("Tab")
    page.keyboard.press("Tab")
    page.keyboard.press("Tab")

    # Increase the value
    page.keyboard.press("ArrowUp")
    page.keyboard.press("ArrowUp")
    page.keyboard.press("ArrowUp")

    page.screenshot(path='/home/jules/verification/screenshots/eq_focus.png')

    # Try the effects tab
    page.click('text=EFFECTS')
    page.wait_for_selector('.effect-slider__track')

    # Tab to effect
    page.keyboard.press("Tab")
    page.keyboard.press("ArrowRight")

    page.screenshot(path='/home/jules/verification/screenshots/effect_focus.png')

    context.close()
    browser.close()
