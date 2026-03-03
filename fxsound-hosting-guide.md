# FXSound Linux — Free Hosting Guide

This guide walks you through deploying your FXSound Linux React app so anyone can access it via a public URL. All options below are **100% free**.

---

## Before You Start — Build Your App

Your `.jsx` file needs to be wrapped in a proper Vite/React project first. Run these once on your machine:

```bash
# 1. Create a new Vite React project
npm create vite@latest fxsound-linux -- --template react
cd fxsound-linux

# 2. Replace the default App component
# Copy your fxsound-linux.jsx content into src/App.jsx

# 3. Install dependencies & test locally
npm install
npm run dev

# 4. Build for production (creates a /dist folder)
npm run build
```

Your deployable app now lives in the `dist/` folder.

---

## Option 1 — Vercel (Recommended ⭐)

**Best for:** Easiest setup, auto-deploys on every code push, fast global CDN.

**Free tier includes:** Unlimited personal projects, 100GB bandwidth/month, custom domain support.

### Steps

```bash
# 1. Push your project to GitHub
git init
git add .
git commit -m "initial commit"
gh repo create fxsound-linux --public --push
# (or push manually via github.com)

# 2. Install Vercel CLI
npm install -g vercel

# 3. Deploy
vercel

# Follow the prompts:
# - Link to your GitHub repo
# - Framework: Vite
# - Build command: npm run build
# - Output directory: dist
```

Your app goes live at `https://fxsound-linux.vercel.app` (or a custom domain).

**Subsequent deploys:** Just `git push` — Vercel auto-redeploys.

---

## Option 2 — Netlify

**Best for:** Drag-and-drop deploy (no CLI needed), great free tier, easy custom domains.

**Free tier includes:** 100GB bandwidth/month, 300 build minutes/month, 1 custom domain.

### Method A — Drag & Drop (Fastest, no account linking needed)

1. Run `npm run build` locally
2. Go to [netlify.com](https://netlify.com) → sign up free
3. Drag your `dist/` folder onto the Netlify dashboard
4. Done — you get a live URL instantly like `https://random-name.netlify.app`

### Method B — Git-based (Auto-deploys)

```bash
# Push to GitHub first (same as Vercel step above), then:
# 1. Netlify dashboard → "Add new site" → "Import from Git"
# 2. Select your repo
# 3. Set:
#    Build command:  npm run build
#    Publish dir:    dist
# 4. Click Deploy
```

**Fix for React Router (if you use it):** Create a `public/_redirects` file:
```
/*  /index.html  200
```

---

## Option 3 — GitHub Pages (Free Forever, No Limits)

**Best for:** Open-source projects, developers already using GitHub, zero cost forever.

**Free tier includes:** Unlimited bandwidth for public repos, free subdomain at `username.github.io`.

### Steps

```bash
# 1. Install the deploy helper
npm install --save-dev gh-pages

# 2. Add to package.json:
#    "homepage": "https://YOUR_USERNAME.github.io/fxsound-linux"
#    In "scripts" add:
#      "predeploy": "npm run build",
#      "deploy": "gh-pages -d dist"

# 3. Deploy
npm run deploy
```

Your app goes live at `https://YOUR_USERNAME.github.io/fxsound-linux`

**Fix blank page issue:** Add this to `vite.config.js`:
```js
export default {
  base: '/fxsound-linux/',  // must match your repo name
}
```

---

## Option 4 — Render

**Best for:** Developers who want a full-stack option later (add a Node backend), automatic HTTPS.

**Free tier includes:** Static site hosting (no sleep/cold starts for static sites), custom domains, auto-deploy from Git.

### Steps

1. Push to GitHub
2. Go to [render.com](https://render.com) → sign up free
3. Click **New → Static Site**
4. Connect your GitHub repo
5. Set:
   - Build command: `npm run build`
   - Publish directory: `dist`
6. Click **Create Static Site**

Live at `https://fxsound-linux.onrender.com`

---

## Comparison Table

| Platform     | Setup Speed | Auto-Deploy | Custom Domain | Bandwidth   | Best For               |
|--------------|-------------|-------------|---------------|-------------|------------------------|
| **Vercel**   | ⚡ Very Fast | ✅ Yes       | ✅ Free        | 100 GB/mo   | Best all-round pick    |
| **Netlify**  | ⚡ Very Fast | ✅ Yes       | ✅ Free        | 100 GB/mo   | Drag & drop simplicity |
| **GitHub Pages** | 🔧 Medium | ✅ Yes    | ✅ Free        | Unlimited   | Open-source projects   |
| **Render**   | 🔧 Medium   | ✅ Yes       | ✅ Free        | 100 GB/mo   | Future full-stack use  |

---

## Adding a Custom Domain (All Platforms)

If you own a domain (e.g. from Namecheap or Cloudflare — both free for `.xyz` domains):

1. In your hosting platform, go to **Domain Settings → Add custom domain**
2. Add your domain (e.g. `fxsound.yourdomain.com`)
3. In your DNS provider, add a **CNAME record**:
   - Name: `fxsound`
   - Value: your platform's provided URL (e.g. `fxsound-linux.vercel.app`)
4. Wait 5–30 minutes for DNS to propagate
5. HTTPS is automatically provisioned — free SSL included on all platforms

---

## Quick Recommendation

**Just want it live in 5 minutes?** → Use **Netlify drag & drop** (Option 2A).

**Want auto-deploys every time you make a change?** → Use **Vercel** (Option 1) with GitHub.

**Want it to stay free forever with no vendor lock-in?** → Use **GitHub Pages** (Option 3).
