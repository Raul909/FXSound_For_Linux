from playwright.sync_api import sync_playwright
import os

def run():
    os.makedirs("/home/jules/verification/screenshots/", exist_ok=True)
    os.makedirs("/home/jules/verification/videos/", exist_ok=True)
    with sync_playwright() as p:
        browser = p.chromium.launch()
        context = browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = context.new_page()
        page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve([]) };")
        page.goto("http://localhost:5173")
        page.wait_for_selector(".eq-band__track")
        page.screenshot(path="/home/jules/verification/screenshots/ui.png")
        context.close()
        browser.close()

if __name__ == "__main__":
    run()
