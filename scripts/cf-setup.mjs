#!/usr/bin/env node
/**
 * Automated Cloudflare Pages setup for FXSound Linux.
 * Run: npm run cf:setup
 *
 * Steps:
 *   1. Login to Cloudflare (opens browser)
 *   2. Create the Pages project (once)
 *   3. Deploy the landing site
 *   4. Print the live URL + GitHub secrets needed
 */

import { execSync } from "child_process";
import { existsSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const root = resolve(__dirname, "..");
const PROJECT = "fxsound-linux";
const LANDING = "landing";

function run(cmd, opts = {}) {
  return execSync(cmd, { cwd: root, stdio: opts.silent ? "pipe" : "inherit", ...opts });
}

function runCapture(cmd) {
  return run(cmd, { silent: true }).toString().trim();
}

console.log("\n🔧 FXSound Linux — Cloudflare Pages Setup\n");

// ── Step 1: Check wrangler is available ──────────────────────────────────
try {
  runCapture("npx wrangler --version");
} catch {
  console.error("❌ wrangler not found. Run: npm install");
  process.exit(1);
}

// ── Step 2: Check landing dir exists ────────────────────────────────────
if (!existsSync(resolve(root, LANDING, "index.html"))) {
  console.error(`❌ ${LANDING}/index.html not found.`);
  process.exit(1);
}

// ── Step 3: Login (opens browser, skips if already logged in) ───────────
console.log("📋 Step 1/3 — Cloudflare login (browser will open)...");
try {
  const whoami = runCapture("npx wrangler whoami 2>&1");
  if (whoami.includes("You are logged in")) {
    console.log("✅ Already logged in.\n");
  } else {
    run("npx wrangler login");
  }
} catch {
  run("npx wrangler login");
}

// ── Step 4: Create project (idempotent) ─────────────────────────────────
console.log(`📋 Step 2/3 — Creating Pages project '${PROJECT}'...`);
try {
  const projects = runCapture("npx wrangler pages project list 2>&1");
  if (projects.includes(PROJECT)) {
    console.log("✅ Project already exists.\n");
  } else {
    run(`npx wrangler pages project create ${PROJECT} --production-branch=main`);
    console.log("✅ Project created.\n");
  }
} catch (e) {
  // project create exits non-zero if it already exists on some versions
  console.log("ℹ️  Project may already exist, continuing...\n");
}

// ── Step 5: Deploy ───────────────────────────────────────────────────────
console.log(`📋 Step 3/3 — Deploying ${LANDING}/ to Cloudflare Pages...`);
run(`npx wrangler pages deploy ${LANDING} --project-name=${PROJECT}`);

// ── Step 6: Get account ID for GitHub secrets ────────────────────────────
let accountId = "";
try {
  const whoami = runCapture("npx wrangler whoami 2>&1");
  const match = whoami.match(/Account ID[:\s]+([a-f0-9]{32})/i);
  if (match) accountId = match[1];
} catch { /* ignore */ }

// ── Done ─────────────────────────────────────────────────────────────────
console.log(`
✅ Deployed! Your site is live at:
   https://${PROJECT}.pages.dev

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
To enable auto-deploy on GitHub releases, add these
secrets to your repo (Settings → Secrets → Actions):

  CLOUDFLARE_ACCOUNT_ID  →  ${accountId || "(run: npx wrangler whoami)"}
  CLOUDFLARE_API_TOKEN   →  https://dash.cloudflare.com/profile/api-tokens
                             Create Token → "Edit Cloudflare Workers" template
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

To redeploy manually anytime:
  npm run cf:deploy
`);
