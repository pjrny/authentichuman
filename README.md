# Authentic Human — authentichuman.co

The public website for **Authentic Human** (Houston, TX — Verify. Protect.
Monetize.), built as a **Rust static-site generator** and deployed with
**GitHub Pages**.

Repo: `pjrny/authentichuman` — the site is generated from `main`.

## How it works

GitHub Pages is static-only, so there is no Rust web server. Instead:

- `src/main.rs` is a dependency-free Rust program (standard library only)
  that contains all site content as editable data/templates and writes a
  complete static site into `dist/`.
- `.github/workflows/pages.yml` runs on every push to `main`: it installs
  Rust, runs `cargo run --release`, and deploys `dist/` with the official
  Pages actions.
- `dist/CNAME` contains `authentichuman.co`, so Pages serves the site on the
  custom domain (DNS for the domain points at GitHub Pages — set the apex
  A records / `www` CNAME in Cloudflare, and enable the custom domain in the
  repo's Pages settings).

## Build locally

```sh
# one-time: install Rust from https://rustup.rs
cargo run --release        # builds and generates dist/
# preview:
python3 -m http.server --directory dist 8000
# open http://localhost:8000
```

## Editing content

Everything lives in `src/main.rs`:

- **Services** — the `SERVICES` table (5 services: likeness-agent,
  local-llm, ethical-framework, scientific-simulator, story-factory).
- **Blogs** — the `BLOGS` table (7 niches, hero format: who / hook /
  problem / losers / approach / example / winners / free tools / CTA).
- **Home page** — `home_body()` (hero, value prop, about, winners/losers,
  the master system diagram, blogs, projects, services, governance teaser).
- **Governance page** — `governance_body()` (C2PA policy, model card,
  NIST AI RMF ethics, AI disclosure TOS). Governance lists **no people** —
  policy only, by design.
- **Themes** — the `THEMES` table generates one CSS file per "reality"
  into `dist/assets/themes/` (17 realities + TempleOS = 18). The header
  switcher swaps the stylesheet and persists the choice in `localStorage`
  only; pressing **S** cycles realities.

## Logos — TODO: drop in the originals

`assets/logo/` currently holds **placeholder** logos extracted from the
rebrand deck artifacts (see `assets/logo/README.md`). The original logo
files replace them by **filename swap only** — no code changes:

`logo-minimalist.png` (used in the header/hero), `logo-maximalist.png`,
`logo-brutalist.png`, `logo-glass.png`, `favicon.svg`.

## Brand rules (do not break)

- Public brand is **Authentic Human** alone. Patron Journey / pjrny.com
  appears only as a project/partner/case study and in the footer credit.
- No old brand name anywhere in the output.
- No street address (city only: Houston, TX), no named advisors/employees,
  no placeholder verification signatures, no phone numbers.
- Winners/Losers stats on the home page are sourced (Upwork / LinkedIn /
  WEF) — do not swap in unsourced numbers.

## Layout

```
Cargo.toml
src/main.rs                  # the generator + all content
assets/logo/                 # logo source files (copied into dist)
.github/workflows/pages.yml  # Pages deployment (branch: main)
dist/                        # generated output (git-ignored)
```
