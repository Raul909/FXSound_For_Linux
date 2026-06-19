from playwright.sync_api import sync_playwright

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        context = browser.new_context(record_video_dir="/home/jules/verification/videos/")
        page = context.new_page()
        page.add_init_script("window.__TAURI_INTERNALS__ = { invoke: () => Promise.resolve([]) };")
        page.goto("http://localhost:5173")
        page.wait_for_timeout(2000)
        page.keyboard.press("Tab")
        page.wait_for_timeout(500)
        page.screenshot(path="/home/jules/verification/screenshots/slider_accessible.png")
        context.close()
        browser.close()

if __name__ == "__main__":
    run()
