from playwright.sync_api import sync_playwright

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        context = browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = context.new_page()
        page.goto("http://localhost:5173")
        page.wait_for_selector(".eq-band__track")

        # Test Effect Slider accessibility
        page.click("text=EFFECTS")
        page.wait_for_selector(".effect-slider__track")
        page.focus(".effect-slider__track")
        page.keyboard.press("ArrowRight")
        page.screenshot(path="/home/jules/verification/screenshots/fx_focus.png")

        context.close()
        browser.close()

run()
