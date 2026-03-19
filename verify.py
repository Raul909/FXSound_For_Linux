import time
from playwright.sync_api import sync_playwright

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(record_video_dir="verification/video")
        page = context.new_page()

        print("Navigating to http://localhost:8080...")
        page.goto("http://localhost:8080")

        # Wait for main app window
        page.wait_for_selector(".app-window", state="visible")
        page.wait_for_timeout(1000)

        # 1. Equalizer Tab - Drag the first EQ band (32Hz)
        print("Testing EQ Band drag...")
        # Get the track of the first EQ band
        eq_track = page.locator(".eq-band__track").first
        eq_track.wait_for(state="visible")

        # Get its bounding box
        box = eq_track.bounding_box()

        # Move mouse to the center of the track and click-drag down
        page.mouse.move(box["x"] + box["width"] / 2, box["y"] + box["height"] / 2)
        page.mouse.down()
        page.wait_for_timeout(100)

        # Drag down by 40 pixels
        for i in range(10):
            page.mouse.move(box["x"] + box["width"] / 2, box["y"] + box["height"] / 2 + (i * 4))
            page.wait_for_timeout(20)

        page.mouse.up()
        page.wait_for_timeout(500)

        # Take a screenshot of the EQ tab
        page.screenshot(path="verification/eq_tab.png")
        print("Screenshot saved to verification/eq_tab.png")

        # 2. Effects Tab
        print("Switching to EFFECTS tab...")
        page.get_by_role("button", name="EFFECTS").click()
        page.wait_for_timeout(500)

        print("Testing Effect Slider drag...")
        # Get the track of the first effect slider (Fidelity)
        fx_track = page.locator(".effect-slider__track").first
        fx_track.wait_for(state="visible")

        # Get its bounding box
        box2 = fx_track.bounding_box()

        # Move mouse to the center of the track and click-drag right
        page.mouse.move(box2["x"] + box2["width"] / 2, box2["y"] + box2["height"] / 2)
        page.mouse.down()
        page.wait_for_timeout(100)

        # Drag right by 40 pixels
        for i in range(10):
            page.mouse.move(box2["x"] + box2["width"] / 2 + (i * 4), box2["y"] + box2["height"] / 2)
            page.wait_for_timeout(20)

        page.mouse.up()
        page.wait_for_timeout(500)

        # Take a screenshot of the Effects tab
        page.screenshot(path="verification/fx_tab.png")
        print("Screenshot saved to verification/fx_tab.png")

        page.wait_for_timeout(1000)

        context.close()
        browser.close()

if __name__ == "__main__":
    import os
    os.makedirs("verification/video", exist_ok=True)
    run()
