//! Authentic Human — static site generator (Rust, standard library only).
//!
//! Running `cargo run --release` writes a complete static site into `dist/`,
//! ready for GitHub Pages (custom domain: authentichuman.co, see dist/CNAME).
//!
//! HOW TO EDIT CONTENT
//! -------------------
//! * Services: edit the `SERVICES` table below.
//! * Blog niches: edit the `BLOGS` table below.
//! * Home page sections: edit `home_body()`.
//! * Governance page: edit `governance_body()`.
//! * Themes ("17 Realities" + TempleOS): edit the `THEMES` table — one CSS
//!   file per theme is generated into `dist/assets/themes/`.
//! * Logos: drop original files into `assets/logo/` (same filenames) — they
//!   are copied to `dist/assets/logo/` untouched. See assets/logo/README.md.
//!
//! Brand rules baked into this generator:
//! * Public brand is "Authentic Human" alone. Patron Journey appears only
//!   as a project / partner / case study and in the footer credit line.
//! * No street address, no named people in governance, no placeholder
//!   verification signatures anywhere in the output.

use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Themes — the 17 Realities + TempleOS
// ---------------------------------------------------------------------------

struct Theme {
    slug: &'static str,
    name: &'static str,
    vibe: &'static str,
    bg: &'static str,
    fg: &'static str,
    accent: &'static str,
    surface: &'static str,
    line: &'static str,
    font_display: &'static str,
    font_body: &'static str,
    font_mono: &'static str,
    radius: &'static str,
    extra: &'static str,
}

const THEMES: &[Theme] = &[
    Theme { slug: "minimalist", name: "Minimalist", vibe: "White linen over concrete — wheatfield horizon",
        bg: "#F6F5F0", fg: "#0A0A0A", accent: "#FF4D00", surface: "#FCFBF8", line: "#D8D4C8",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "'Instrument Sans', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "20px", extra: "" },
    Theme { slug: "maximalist", name: "Maximalist", vibe: "Lava over snow aerial — neon bleed",
        bg: "#FFF7E8", fg: "#1A0E00", accent: "#FF4D00", surface: "#FFEAD9", line: "#FFB98A",
        font_display: "'Syne', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'Space Mono', ui-monospace, monospace", radius: "28px", extra: "h1{letter-spacing:-0.03em;text-transform:uppercase}" },
    Theme { slug: "brutalist", name: "Brutalist", vibe: "Black asphalt cut by safety orange tape",
        bg: "#F2F2F2", fg: "#0A0A0A", accent: "#FF4D00", surface: "#FFFFFF", line: "#0A0A0A",
        font_display: "'Archivo Black', sans-serif", font_body: "'Archivo', system-ui, sans-serif",
        font_mono: "'JetBrains Mono', ui-monospace, monospace", radius: "0px", extra: ".card,.tile,.chip{border-width:2px}h1{text-transform:uppercase}" },
    Theme { slug: "glassmorphism", name: "Glassmorphism", vibe: "Frosted studio glass over thundercloud",
        bg: "#E8E9F0", fg: "#14161F", accent: "#5B5BD6", surface: "rgba(255,255,255,0.55)", line: "rgba(20,22,31,0.18)",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "'Instrument Sans', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "24px", extra: ".card,.tile{backdrop-filter:blur(14px);-webkit-backdrop-filter:blur(14px)}body{background:linear-gradient(160deg,#D9E6FF,#E8E9F0 45%,#F5E9C9)}" },
    Theme { slug: "neumorphism", name: "Neumorphism", vibe: "Soft clay pressed into same clay",
        bg: "#EDECE8", fg: "#2A2A26", accent: "#8A6D3B", surface: "#EDECE8", line: "#D8D5CC",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "'Inter', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "26px", extra: ".card,.tile{box-shadow:8px 8px 18px rgba(0,0,0,0.12),-8px -8px 18px rgba(255,255,255,0.8);border-color:transparent}" },
    Theme { slug: "retro", name: "Retro", vibe: "Mustard vinyl over VHS static",
        bg: "#F5E9C9", fg: "#26200F", accent: "#D97706", surface: "#FFF3D6", line: "#C9AE6A",
        font_display: "'Syne', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "6px", extra: "body{background-image:repeating-linear-gradient(0deg,rgba(0,0,0,0.035) 0 1px,transparent 1px 3px)}" },
    Theme { slug: "tumblr", name: "2014 Tumblr", vibe: "Film grain sky over parking lot sunset",
        bg: "#FDF6EC", fg: "#33302A", accent: "#C46A4A", surface: "#FFFFFF", line: "#E3D5C0",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "Georgia, serif",
        font_mono: "'Courier Prime', 'Courier New', monospace", radius: "4px", extra: "h1{font-style:italic}" },
    Theme { slug: "alternative", name: "Alternative", vibe: "Flannel over glitch screen",
        bg: "#101010", fg: "#EDECE8", accent: "#B8FF00", surface: "#1A1A16", line: "#33332C",
        font_display: "'Archivo Black', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'Space Mono', ui-monospace, monospace", radius: "2px", extra: "h1{text-shadow:2px 0 0 rgba(184,255,0,0.45),-2px 0 0 rgba(255,0,128,0.35)}" },
    Theme { slug: "americana", name: "Americana", vibe: "Wheatfield under a Van Gogh storm",
        bg: "#F8F6F0", fg: "#1E2430", accent: "#B8860B", surface: "#FFFFFF", line: "#D9CFB4",
        font_display: "'Newsreader', Georgia, serif", font_body: "Georgia, serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "10px", extra: "" },
    Theme { slug: "coquette", name: "Coquette", vibe: "Lace over wet asphalt",
        bg: "#FFEAF2", fg: "#33141F", accent: "#D6336C", surface: "#FFF5F9", line: "#F3C2D4",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "'Instrument Sans', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "30px", extra: "h1{font-style:italic}" },
    Theme { slug: "eurotrash", name: "Eurotrash", vibe: "Chrome rave flyer over concrete",
        bg: "#0C0C0B", fg: "#F2F2F2", accent: "#00FFFF", surface: "#171716", line: "#333330",
        font_display: "'Syne', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'JetBrains Mono', ui-monospace, monospace", radius: "14px", extra: "h1{background:linear-gradient(92deg,#fff,#9be9ff 55%,#fff);-webkit-background-clip:text;background-clip:text;color:transparent}" },
    Theme { slug: "glitter", name: "Glitter Graphics", vibe: "Glitter over midnight highway",
        bg: "#0A0A0A", fg: "#FFF7FB", accent: "#FF00FF", surface: "#181018", line: "#3A2438",
        font_display: "'Syne', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'Space Grotesk', ui-monospace, monospace", radius: "18px", extra: "h1{text-shadow:0 0 18px rgba(255,0,255,0.55)}" },
    Theme { slug: "hyperpop", name: "Hyperpop", vibe: "Acid green slime over cloud",
        bg: "#F4FFE8", fg: "#0F1A00", accent: "#65A300", surface: "#FFFFFF", line: "#CBE8A6",
        font_display: "'Archivo Black', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'Space Grotesk', ui-monospace, monospace", radius: "32px", extra: ":root{--accent-2:#B8FF00}h1{letter-spacing:-0.02em}" },
    Theme { slug: "punk", name: "Punk Derivatives", vibe: "Xerox flyer over safety vest",
        bg: "#FFFFFF", fg: "#000000", accent: "#FF4D00", surface: "#FFFFFF", line: "#000000",
        font_display: "'Anton', sans-serif", font_body: "'Archivo', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "0px", extra: ".card,.tile{border-width:2px;box-shadow:5px 5px 0 #000}h1{text-transform:uppercase;transform:rotate(-0.6deg)}" },
    Theme { slug: "regional-gothic", name: "Regional Gothic", vibe: "Cornfield at dusk + police lights",
        bg: "#1A1A16", fg: "#E7E0C6", accent: "#D97706", surface: "#22221C", line: "#454338",
        font_display: "'Instrument Serif', Georgia, serif", font_body: "Georgia, serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "8px", extra: "body{background-image:radial-gradient(rgba(231,224,198,0.05) 1px,transparent 1px);background-size:5px 5px}" },
    Theme { slug: "soundcloud", name: "SoundCloud Rap", vibe: "Purple LED fog over parking garage",
        bg: "#12081F", fg: "#F1E9FF", accent: "#B26BFF", surface: "#1D1030", line: "#3B2757",
        font_display: "'Syne', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'Space Mono', ui-monospace, monospace", radius: "16px", extra: "h2{border-bottom:3px solid var(--accent);display:inline-block;padding-bottom:0.15em}" },
    Theme { slug: "y2k", name: "Y2K", vibe: "Silver mylar over blue sky gradient",
        bg: "#D9E6FF", fg: "#0B1E4B", accent: "#FF4D00", surface: "rgba(255,255,255,0.75)", line: "#9DB8EA",
        font_display: "'Orbitron', sans-serif", font_body: "'Space Grotesk', system-ui, sans-serif",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "22px", extra: "body{background:linear-gradient(180deg,#B8D2FF,#EAF2FF)}h1{text-shadow:0 2px 0 rgba(255,255,255,0.9)}" },
    Theme { slug: "templeos", name: "TempleOS", vibe: "Pure black void — monospace only. PRESS [S] TO SWITCH REALITY",
        bg: "#000000", fg: "#FFFFFF", accent: "#FFFFFF", surface: "#000000", line: "#FFFFFF",
        font_display: "'IBM Plex Mono', ui-monospace, monospace", font_body: "'IBM Plex Mono', ui-monospace, monospace",
        font_mono: "'IBM Plex Mono', ui-monospace, monospace", radius: "0px", extra: "*{box-shadow:none!important;text-shadow:none!important}a{color:#fff;text-decoration:underline}.card,.tile,.chip{border-width:1px}" },
];

// ---------------------------------------------------------------------------
// Shared CSS / JS / favicon assets
// ---------------------------------------------------------------------------

const FONTS_LINK: &str = "https://fonts.googleapis.com/css2?family=Anton&family=Archivo+Black&family=Archivo:wght@400;600&family=Courier+Prime:ital@0;1&family=IBM+Plex+Mono:wght@400;500;600&family=Instrument+Sans:wght@400;600&family=Instrument+Serif:ital@0;1&family=Inter:wght@400;600&family=JetBrains+Mono:wght@400;700&family=Newsreader:ital,opsz@0,6..72;1,6..72&family=Orbitron:wght@600;800&family=Space+Grotesk:wght@400;700&family=Space+Mono&family=Syne:wght@600;800&display=swap";

const BASE_CSS: &str = r##"
*{box-sizing:border-box}
html{scroll-behavior:smooth}
body{margin:0;background:var(--bg);color:var(--fg);font-family:var(--font-body);line-height:1.55;font-size:17px}
a{color:var(--accent)}
img{max-width:100%}
.wrap{max-width:1180px;margin:0 auto;padding:0 20px}
header.site{position:sticky;top:0;z-index:50;background:var(--bg);border-bottom:1px solid var(--line)}
.header-in{display:flex;align-items:center;gap:16px;flex-wrap:wrap;padding:10px 0}
.brand{display:flex;align-items:center;gap:10px;text-decoration:none;color:var(--fg);font-family:var(--font-display);font-size:22px}
.brand img{width:40px;height:40px;object-fit:contain}
nav.main{display:flex;gap:16px;flex-wrap:wrap;margin-left:auto;font-family:var(--font-mono);font-size:13px;text-transform:uppercase;letter-spacing:0.08em}
nav.main a{color:var(--fg);text-decoration:none}
nav.main a:hover{color:var(--accent)}
.switcher{display:flex;align-items:center;gap:8px;font-family:var(--font-mono);font-size:12px;text-transform:uppercase;letter-spacing:0.08em}
.switcher select{background:var(--surface);color:var(--fg);border:1px solid var(--line);border-radius:999px;padding:6px 10px;font-family:var(--font-mono);font-size:12px;max-width:190px}
.mono{font-family:var(--font-mono)}
.eyebrow{font-family:var(--font-mono);font-size:12px;letter-spacing:0.16em;text-transform:uppercase;opacity:0.65}
h1{font-family:var(--font-display);font-weight:800;font-size:clamp(44px,8vw,84px);line-height:0.95;letter-spacing:-0.02em;margin:0.25em 0}
h1 .serif-i{font-family:'Instrument Serif',Georgia,serif;font-style:italic;font-weight:400}
h2{font-family:var(--font-display);font-size:clamp(28px,4vw,42px);line-height:1.05;margin:0 0 0.4em}
h3{font-family:var(--font-display);font-size:21px;margin:0 0 0.35em}
section{padding:56px 0;border-bottom:1px solid var(--line)}
.lede{font-size:clamp(19px,2.4vw,24px);max-width:46em}
.chip{display:inline-flex;align-items:center;gap:8px;border:1px solid var(--line);border-radius:999px;padding:6px 14px;font-family:var(--font-mono);font-size:12px;margin:4px 6px 4px 0;background:var(--surface)}
.chip.solid{background:var(--fg);color:var(--bg);border-color:var(--fg)}
.cta-row{display:flex;gap:12px;flex-wrap:wrap;margin-top:22px}
.btn{display:inline-block;padding:13px 22px;border-radius:999px;border:1px solid var(--fg);text-decoration:none;font-family:var(--font-mono);font-size:14px;color:var(--fg);background:var(--surface)}
.btn.primary{background:var(--accent);border-color:var(--accent);color:#fff}
.grid{display:grid;gap:18px}
.grid.c2{grid-template-columns:repeat(auto-fit,minmax(300px,1fr))}
.grid.c3{grid-template-columns:repeat(auto-fit,minmax(250px,1fr))}
.card,.tile{border:1px solid var(--line);border-radius:var(--radius);background:var(--surface);padding:22px}
.card .tag,.tile .tag{font-family:var(--font-mono);font-size:11px;letter-spacing:0.14em;text-transform:uppercase;color:var(--accent)}
.card a.more{font-family:var(--font-mono);font-size:13px}
table{border-collapse:collapse;width:100%;font-size:15px}
th,td{border:1px solid var(--line);padding:10px 12px;text-align:left;vertical-align:top}
th{font-family:var(--font-mono);font-size:12px;text-transform:uppercase;letter-spacing:0.08em}
pre.copy{background:var(--surface);border:1px solid var(--line);border-radius:var(--radius);padding:18px;overflow:auto;white-space:pre-wrap;font-family:var(--font-mono);font-size:13.5px}
.win{border-color:var(--accent)}
.stat{font-family:var(--font-display);font-size:40px;line-height:1}
.stat small{display:block;font-family:var(--font-mono);font-size:11.5px;letter-spacing:0.06em;text-transform:uppercase;opacity:0.7;margin-top:6px;line-height:1.5}
.diagram{border:1px solid var(--line);border-radius:var(--radius);background:var(--surface);padding:20px;overflow-x:auto}
.d-row{display:flex;gap:10px;flex-wrap:wrap;justify-content:center;margin:10px 0}
.d-box{border:1.5px solid var(--fg);border-radius:12px;padding:10px 14px;font-family:var(--font-mono);font-size:12.5px;text-align:center;background:var(--bg)}
.d-box.core{background:var(--fg);color:var(--bg);font-size:15px;padding:16px 22px}
.d-box.zone{flex:1 1 180px;text-align:left}
.d-arrow{text-align:center;font-family:var(--font-mono);font-size:12px;opacity:0.75}
footer.site{padding:44px 0 90px;font-family:var(--font-mono);font-size:13px}
footer.site a{color:var(--fg)}
.sticky-cta{display:none}
@media (max-width:760px){
  .sticky-cta{display:block;position:fixed;left:12px;right:12px;bottom:12px;z-index:60;text-align:center;background:var(--accent);color:#fff;padding:14px;border-radius:999px;text-decoration:none;font-family:var(--font-mono);font-size:14px}
}
.hero-logo{width:min(210px,55vw);height:auto}
.texture{font-family:var(--font-mono);font-size:12px;opacity:0.7}
ul.tight{margin:0.4em 0;padding-left:1.15em}
ul.tight li{margin:0.28em 0}
"##;

const APP_JS: &str = r##"
// Authentic Human — style switcher ("Switch Reality")
// Theme choice persists in localStorage only. Pressing "s" cycles realities.
(function () {
  var THEME_KEY = "ah-theme";
  var link = document.getElementById("theme-css");
  var select = document.getElementById("theme-select");
  // Per-reality logos from the user's original artwork (chroma-keyed PNGs in
  // assets/logo/). Fallback: theme HEAD -> theme AH -> Minimalist HEAD.
  // retro: no live logo (the two retro artworks embed pre-2018 dates and are
  // archive-only). eurotrash/templeos: no logo supplied.
  var LOGOS = {
    "minimalist": {"head": "logo-minimalist-head.webp", "ah": "logo-minimalist-ah.webp"},
    "maximalist": {"head": "logo-maximalist-head.webp", "ah": "logo-maximalist-ah.webp"},
    "brutalist": {"head": "logo-brutalist-head.webp", "ah": "logo-brutalist-ah.webp"},
    "glassmorphism": {"head": "logo-glassmorphism-head.webp"},
    "neumorphism": {"head": "logo-neumorphism-head.webp", "ah": "logo-neumorphism-ah.webp"},
    "tumblr": {"head": "logo-tumblr-head.webp"},
    "alternative": {"head": "logo-alternative-head.webp", "ah": "logo-alternative-ah.webp"},
    "americana": {"head": "logo-americana-head.webp", "ah": "logo-americana-ah.webp"},
    "coquette": {"head": "logo-coquette-head.webp", "ah": "logo-coquette-ah.webp"},
    "glitter": {"head": "logo-glitter-head.webp", "ah": "logo-glitter-ah.webp"},
    "hyperpop": {"ah": "logo-hyperpop-ah.webp"},
    "punk": {"head": "logo-punk-head.webp", "ah": "logo-punk-ah.webp"},
    "regional-gothic": {"ah": "logo-regional-gothic-ah.webp"},
    "soundcloud": {"head": "logo-soundcloud-head.webp", "ah": "logo-soundcloud-ah.webp"},
    "y2k": {"ah": "logo-y2k-ah.png"}
  };
  var DEFAULT_LOGO = "logo-minimalist-head.webp";
  function swapLogo(slug) {
    var entry = LOGOS[slug] || {};
    var file = entry.head || entry.ah || DEFAULT_LOGO;
    var src = base() + "assets/logo/" + file;
    var brand = document.getElementById("brand-logo");
    if (brand) brand.setAttribute("src", src);
    var hero = document.getElementById("hero-logo");
    if (hero) hero.setAttribute("src", src);
  }
  function base() {
    return link.getAttribute("data-base") || "";
  }
  function slugs() {
    if (!select) return [];
    var out = [];
    for (var i = 0; i < select.options.length; i++) out.push(select.options[i].value);
    return out;
  }
  function apply(slug) {
    if (!slug) return;
    link.setAttribute("href", base() + "assets/themes/" + slug + ".css");
    document.documentElement.setAttribute("data-theme", slug);
    swapLogo(slug);
    try { localStorage.setItem(THEME_KEY, slug); } catch (e) {}
    if (select) select.value = slug;
  }
  var saved = null;
  try { saved = localStorage.getItem(THEME_KEY); } catch (e) {}
  if (saved && slugs().indexOf(saved) >= 0) apply(saved);
  if (select) {
    select.addEventListener("change", function () { apply(select.value); });
  }
  document.addEventListener("keydown", function (ev) {
    var t = ev.target;
    if (t && (t.tagName === "INPUT" || t.tagName === "TEXTAREA" || t.tagName === "SELECT")) return;
    if (ev.key !== "s" && ev.key !== "S") return;
    if (ev.metaKey || ev.ctrlKey || ev.altKey) return;
    var list = slugs();
    if (!list.length) return;
    var cur = document.documentElement.getAttribute("data-theme") || list[0];
    var next = list[(list.indexOf(cur) + 1) % list.length];
    apply(next);
  });
})();
"##;

// ---------------------------------------------------------------------------
// Services — 5 pages (content adapted from the user's service handoff;
// research-led platform model is the extra card on the home page)
// ---------------------------------------------------------------------------

struct Service {
    slug: &'static str,
    name: &'static str,
    tag: &'static str,
    title: &'static str,
    meta: &'static str,
    hook: &'static str,
    icp: &'static str,
    pain: &'static str,
    process: &'static [&'static str],
    pricing: &'static [&'static str],
    case: &'static str,
    links: &'static [(&'static str, &'static str)],
    cta: &'static str,
}

const SERVICES: &[Service] = &[
    Service {
        slug: "likeness-agent",
        name: "Likeness Agent — OIMO MAS",
        tag: "Flagship",
        title: "Likeness Agent — Verify, Protect, Monetize Your Face, Voice, Identity",
        meta: "Own your AI avatar. Authentic Human identifies where your likeness is used, protects it with watermark and proof, and monetizes it. Not a $29/mo rental.",
        hook: "Identify. Protect. Monetize. Your likeness, voice and creativity — verifiably yours, earning for you, revocable by you.",
        icp: "Influencers, musicians, filmmakers and podcasters — roughly 10k to 1M followers — who are starting to see deepfakes and clones of themselves.",
        pain: "Your face in an ad you never shot. Your voice cloned on a short-video app. No credit, no pay, no control.",
        process: &[
            "IDENTIFY — we scan the web and social platforms for uses of your face and voice.",
            "PROTECT — C2PA Content Credentials, watermarking and fingerprinting on every output, proof of ownership, and takedown monitoring.",
            "MONETIZE — a custom avatar and voice you actually own, deployed under your control, licensed to others on a revenue share.",
        ],
        pricing: &[
            "Build: from $5k",
            "Care plan: from $500/mo (monitoring, takedowns, updates)",
            "Licensing: 20% revenue share when others license your likeness — you keep 80%",
            "Compare: HeyGen at $29/mo and Synthesia at $19/mo are rentals — the platform owns the avatar. Here, you own it outright.",
        ],
        case: "WorldFest filmmakers: likeness protected and monetized through flix.worldfest.org, with provenance on every deliverable.",
        links: &[],
        cta: "Join the LMA beta",
    },
    Service {
        slug: "local-llm",
        name: "Local LLM — Replace-a-Team",
        tag: "Most requested",
        title: "Local LLM That Replaces Your Coding Team — On-Site, No Data Leak",
        meta: "Open models (Qwen, DeepSeek and peers) running on your own hardware, configured on your workflows. Your data never leaves, and is never trained on.",
        hook: "A private LLM that runs local. No data leaves your building, nothing is trained on your data — and it is watched and updated for you.",
        icp: "Manufacturers, labs and service companies (roughly 50–500 people) with internal development, support or documentation work — and IP they cannot leak to a cloud.",
        pain: "Paying enterprise SaaS rent every month while your documents, code and customer data flow into someone else's cloud models.",
        process: &[
            "Audit — we map the workflows worth automating and the data that must never leave.",
            "Build — open models (Qwen, DeepSeek and peers) served on your hardware, wired to your tools: code, lab notes, grant writing, support.",
            "Configure on your data — retrieval and tools connect the model to your documents. Configured on your data, never trained on it, and it never leaves your site.",
            "Watch — the node reports its health, we monitor it and push model and OS updates. You get a watched system, not a box that rots.",
        ],
        pricing: &[
            "Setup: from $15–40k depending on scope",
            "Care plan: from $2k/mo (monitoring, updates, support)",
            "Runs on your hardware or a node we configure — see the Platform offer on the home page for the smaller watched-node version.",
        ],
        case: "DBE_V0 taxonomy observatory: an autonomous agent running research workflows on local models.",
        links: &[("DBE_V0 — live", "https://pjrny.github.io/DBE_V0/")],
        cta: "Book an architecture call — Houston, TX",
    },
    Service {
        slug: "ethical-framework",
        name: "Ethical AI Framework",
        tag: "Governance",
        title: "Ethical AI Framework — Governance, Not Vibes",
        meta: "AI without governance amplifies risk. Authentic Human builds your governance practice, policy, training and monitoring — NIST AI RMF aligned, WCAG 2.2 AA.",
        hook: "Governance first: a model card, a C2PA policy, an AI disclosure TOS, training for your people, and a human in the loop for every release.",
        icp: "Universities, labs, HR teams and mid-size companies where people already use generative AI every week — with no policy behind it.",
        pain: "Lawsuit risk from unreviewed AI output, likeness misuse, inaccessible tools — and good people leaving because nobody will set a sane policy.",
        process: &[
            "Assess — where AI is already used, what data it touches, where the risk actually sits.",
            "Govern — governance practice built on OrgLogic principles, vendor-neutral evaluation (OAI.co mapping) and the NIST AI RMF: Map, Measure, Manage, Govern.",
            "Document — model card, C2PA provenance policy and an AI disclosure TOS your counsel can actually use (see our own on the Governance page).",
            "Train — your people go from never-users to power users, with monitoring that keeps it true after we leave.",
        ],
        pricing: &[
            "Workshop: from $3k",
            "Oversight: from $2k/mo (policy, audit, training, monitoring)",
        ],
        case: "Our own governance stack is public on the Governance page — C2PA policy, model card, ethics and disclosure TOS. We sell what we run.",
        links: &[],
        cta: "Download the free framework template",
    },
    Service {
        slug: "scientific-simulator",
        name: "Scientific Simulator",
        tag: "Grants",
        title: "Scientific Simulators for Fusion, Quantum, Policy — Forecasting Instability",
        meta: "We wrap your Fortran and Python physics in an AI agent with a dashboard and Monte Carlo runs — and co-write the grant that pays for it.",
        hook: "Fusion, quantum and policy simulations that forecast instability. Real data, real forecasting — not entertainment.",
        icp: "Early-career fusion and quantum researchers, university labs, and political campaigns that need to test decisions before they announce them.",
        pain: "Simulators take months to run and tune by hand. Papers sit in draft. Funding windows (DOE ECRP at $96M, Genesis at $159M) close while you wait.",
        process: &[
            "Bring the science — your existing Fortran/Python models and data stay yours.",
            "Wrap — we build the autonomous agent layer: parameter search, Monte Carlo runs, a dashboard a human can steer.",
            "Forecast — instability and sensitivity results you can publish and defend.",
            "Fund — we help co-write the grant (DOE ECRP and similar) that pays for the next round.",
        ],
        pricing: &[
            "Build: from $10k",
            "Grant co-write included in the engagement",
        ],
        case: "PoliSim-DBE-Game: a political and economic simulator running Monte Carlo policy tests. DBE_V0: the scientific taxonomy observatory behind it.",
        links: &[("DBE_V0 — live", "https://pjrny.github.io/DBE_V0/"), ("Political Economic Simulator — live", "https://poli-eco-sim.kimi.page/")],
        cta: "Co-write an ECRP grant with us",
    },
    Service {
        slug: "story-factory",
        name: "Story Factory",
        tag: "Content",
        title: "Story Factory — One Idea Into Social, PR, Ebook, Music and Video",
        meta: "One idea becomes six formats: social, PR release, ebook, audiobook, music and poster — with provenance on every asset.",
        hook: "One idea in, a full story package out. The storytelling pillar, productized.",
        icp: "Indie festivals, event organizers and campaigns with one story to tell everywhere — and one person to tell it.",
        pain: "You need ten formats and you have one person. So the story ships in one place and dies there.",
        process: &[
            "Script — we shape the one idea into a tight explainer script.",
            "Produce — an explainer (avatar or narrated) built with our home AI factory: coding and graphics stay in-house.",
            "Multiply — the same idea becomes social clips, a PR release, an ebook, an audiobook, music and a poster.",
            "Sign — every asset carries C2PA Content Credentials, so the story is verifiably yours wherever it travels.",
        ],
        pricing: &[
            "Per story package: from $2k",
            "Includes press-release template and a 60-second explainer script",
        ],
        case: "WorldFest on flix.worldfest.org: independent human stories, streamed free, each one a package instead of a single post.",
        links: &[("Make a Scene — free", "https://opal-flora-opal-forest.grok.me/")],
        cta: "Order a story",
    },
];

// ---------------------------------------------------------------------------
// Blogs — 6 niches, hero format (who / hook / problem / losers / approach /
// example / winners / free tools / CTA), adapted from the user's handoff
// ---------------------------------------------------------------------------

struct Blog {
    slug: &'static str,
    niche: &'static str,
    maker: &'static str,
    texture: &'static str,
    hook: &'static str,
    who: &'static str,
    problem: &'static str,
    losers: &'static str,
    approach: &'static str,
    example: &'static str,
    winners: &'static str,
    tools: &'static [(&'static str, &'static str)],
    links: &'static [(&'static str, &'static str)],
    cta: &'static str,
}

const BLOGS: &[Blog] = &[
    Blog {
        slug: "indie-film-festivals",
        niche: "Indie Film Festivals",
        maker: "Authentic Human x WorldFest-Houston",
        texture: "Wheat over thunder — Van Gogh festival poster",
        hook: "You won't believe how many festivals die because subtitling costs more than the prize money.",
        who: "Indie festival directors and curators with under $10k of tech budget, 50–300 films a year, drowning in subtitling costs.",
        problem: "AI dubbing went from roughly $100/min to $0.10/min in 18 months, and audiences now expect localized films. Festivals are still paying 2019 vendor rates — and losing international submissions because of it.",
        losers: "Festivals using burnt-in subtitles and missing the majority of non-English viewers; paying thousands per festival for dubbing; renting a per-film avatar tool and losing the IP; hosting on a generic video platform and keeping no audience data.",
        approach: "Own your pipeline. Open Whisper for transcription, open TTS for dubbing with human QC, poster generation in-house, and host free on flix.worldfest.org under your own branding — keeping IP and audience data.",
        example: "WorldFest: 20 languages of AI subtitles for effectively $0 against a $4k vendor quote — 2 hours instead of 2 weeks. The taxonomy behind it lives free at database.worldfest.org.",
        winners: "Winners get more international submissions (filmmakers love localized screenings), sponsors who love owned data, and an audience that stays — a festival that is global without losing its indie soul.",
        tools: &[
            ("Websites", "flix.worldfest.org, database.worldfest.org, pjrny.com/organizers"),
            ("Apps", "CapCut AI Dub, ElevenLabs Dubbing"),
            ("GitHub", "openai/whisper, coqui-ai/TTS, whisper-diarization"),
            ("AIs", "Whisper large-v3, Coqui XTTS, poster generation"),
            ("Hardware", "Any modern Mac or a home GPU box — $0 extra to start"),
            ("Databases", "Our Indie Film Tools DB (updated weekly)"),
        ],
        links: &[("Make a Scene — free scene builder", "https://opal-flora-opal-forest.grok.me/")],
        cta: "Want free hosting for your festival? Apply via flix.worldfest.org — or join the LMA beta to monetize filmmaker likeness.",
    },
    Blog {
        slug: "event-organizers",
        niche: "Event Organizers",
        maker: "Authentic Human x Patron Journey",
        texture: "Wheelchair courier over a festival stage — access at full speed",
        hook: "You will want to hear this if you've ever paid to sell your own tickets.",
        who: "Independent organizers running 50–500 person events, losing money per event to ticketing fees.",
        problem: "Ticketing monopolies take a percentage for a QR code, keep your attendee data, and now AI writes everyone's promo copy the same way. Local models can write 100 event descriptions in 10 minutes — in your voice, not the generic one.",
        losers: "Organizers paying percentage fees on every ticket, losing their attendee data to the platform, and running the same listing copy for three years.",
        approach: "pjrny.com/organizers: free ticketing, QR door check-in, organizer dashboard — and you own the data and the email list. Local-LLM promo copy configured on your voice.",
        example: "Atomika Fest and WorldFest run free ticketing with QR check-in and no fees. Before: per-event fees and no data. After: $0 fees and the full attendee list.",
        winners: "Winners keep the fee money on every event, own their list, and can actually retarget the people who came.",
        tools: &[
            ("Websites", "pjrny.com/organizers, authentichuman.co"),
            ("Apps", "QR code check-in, organizer dashboard"),
            ("GitHub", "cal.com, local LLM tooling"),
            ("AIs", "Local open models for promo copy in your voice"),
            ("Hardware", "Repurposed phones as door scanners — offline check-in that syncs later"),
            ("Databases", "Our Event Tools DB"),
        ],
        links: &[("Free ticketing — pjrny.com/organizers", "https://pjrny.com/organizers")],
        cta: "Create a free event on pjrny.com/organizers, or apply as a partner.",
    },
    Blog {
        slug: "fusion-clean-energy",
        niche: "Fusion / Clean Energy Researchers",
        maker: "Authentic Human x DOE Labs",
        texture: "Lava over snow — fusion core",
        hook: "You will want to hear this if your simulator still runs on Fortran and a prayer.",
        who: "Early-career fusion and quantum researchers, and university labs applying to DOE programs.",
        problem: "DOE put $96M into ECRP and $159M into Genesis for AI + fusion, quantum and chip design. Labs need simulation that runs in days, not months — miss the window and the funding goes to a lab that automated.",
        losers: "Labs with no AI agent layer around their physics code, tuning parameters by hand for months while papers sit in draft.",
        approach: "Build an AI agent that wraps your physics code: autonomous parameter search, Monte Carlo runs, a dashboard a human steers — forecasting, not guesswork.",
        example: "DBE_V0, our taxonomy observatory, models complex systems this way — the same approach behind the Scientific Simulator service.",
        winners: "Winners get funded, publish faster, and walk into the next call with results instead of intentions.",
        tools: &[
            ("Websites", "DBE_V0 — pjrny.github.io/DBE_V0, authentichuman.co/services/scientific-simulator, DOE user facilities"),
            ("Apps", "Simulation dashboards, Monte Carlo runners"),
            ("GitHub", "deepxde, fusion-data tooling"),
            ("AIs", "Local open models for code, lab notes and grant drafts"),
            ("Hardware", "DOE lab access, your own GPU node"),
            ("Databases", "Our Fusion Tools DB"),
        ],
        links: &[("DBE_V0 — live demo", "https://pjrny.github.io/DBE_V0/")],
        cta: "Co-write a DOE ECRP grant with us — see the Scientific Simulator service.",
    },
    Blog {
        slug: "political-campaigns",
        niche: "Political Campaigns",
        maker: "Authentic Human x PoliSim",
        texture: "A DON'T QUIT sign over the capitol",
        hook: "You will want to hear this if your canvassing list is from 2022.",
        who: "Local candidates and issue organizations in districts of roughly 5k–50k voters.",
        problem: "AI voter modeling and simulation changed campaigning. Campaigns that don't simulate their message against the economy and the district lose close races — sometimes by 2%.",
        losers: "Campaigns running on gut feeling and one message for every voter, with a stale list and no way to test a policy before announcing it.",
        approach: "PoliSim-DBE-Game: run the economic policy sim, forecast instability, and prep against a simulated opponent — with C2PA-verified comms so your media is provably yours.",
        example: "PoliSim tests central-bank rates against inflation and GDP with 10,000-run Monte Carlo simulations — 100 policy combinations tested before one is announced.",
        winners: "Winners announce the policy that already survived simulation, and spend their budget where the model says voters actually move.",
        tools: &[
            ("Websites", "Political Economic Simulator — poli-eco-sim.kimi.page, authentichuman.co (PoliSim project)"),
            ("Apps", "PoliSim game, policy simulator"),
            ("GitHub", "policysimulator"),
            ("AIs", "Local LLM voter personas — run locally, never uploaded"),
            ("Hardware", "A laptop is enough to start"),
            ("Databases", "Our Political Sim DB"),
        ],
        links: &[("Political Economic Simulator — live", "https://poli-eco-sim.kimi.page/")],
        cta: "Request the PoliSim template.",
    },
    Blog {
        slug: "influencers-creators",
        niche: "Influencers, Creators & Musicians",
        maker: "Authentic Human x OIMO MAS",
        texture: "Snow over sand dunes — clone vs real",
        hook: "You will want to hear this if you've seen an ad with 'you' in it that you never shot.",
        who: "Creators with 10k+ followers — and independent musicians and producers, roughly 1k–100k listeners — who are at real risk of deepfakes, unauthorized clones, and platforms owning their face or voice.",
        problem: "Avatar platforms made likeness rentable — but a rented face is a face the platform owns. Meanwhile your actual face is being scaled into ads without you. Musicians got hit first: from Jukebox in 2018 to today's AI music explosion, stems uploaded to random sites train other people's models for free, and voices get cloned with no pay.",
        losers: "Creators renting their face for $29/mo while the platform keeps the avatar, anyone who signed away likeness rights in a ToS they never read, and musicians uploading stems anywhere and signing away voice in the fine print.",
        approach: "OIMO MAS — the Likeness Agent: Identify where your likeness is used, Protect it with C2PA provenance and takedowns, Monetize it on a revenue share you control. For musicians: local stem separation and a voice likeness you own, verified the same way.",
        example: "WorldFest filmmakers already run this protection: verified likeness, provenance on deliverables, licensing instead of leaks. And the founder's Jukebox-era music experiments (2018–2020) drew 1M+ views — Authentic Human exists because that era also proved the rights problem.",
        winners: "Winners earn passive avatar income from a likeness they own and can revoke — and can prove the fakes are fakes. Musicians hold a verified voice that earns while they sleep.",
        tools: &[
            ("Websites", "authentichuman.co/services/likeness-agent"),
            ("Apps", "Face-verification tooling, takedown monitoring, Demucs / Spleeter for local stem separation"),
            ("GitHub", "face-verification projects, C2PA tooling, audiocraft, local RVC voice tooling"),
            ("AIs", "Local avatar builds and local voice models — ownable, not rented"),
            ("Hardware", "A phone scan is enough to start; your studio machine for music — nothing needs to leave it"),
            ("Databases", "Our Likeness Tools DB + Music AI Tools DB"),
        ],
        links: &[],
        cta: "Join the LMA beta — creators and musicians alike.",
    },
    Blog {
        slug: "universities-labs",
        niche: "Universities / Labs",
        maker: "Authentic Human x Patron Journey Labs",
        texture: "Hands opening a portal — simulation and TempleOS simplicity",
        hook: "AI without governance doesn't reduce risk. It amplifies it.",
        who: "Universities, labs and HR teams that need ethical AI their counsel can sign off on.",
        problem: "Most organizations now use generative AI weekly — many daily — with no ethics practice, no model card, and a policy that amounts to 'don't paste PII'. The stakes are lawsuits and lost talent.",
        losers: "Institutions that ban AI instead of governing it, run no training, and discover their exposure in a complaint instead of an audit.",
        approach: "Governance first: an ethical framework and board practice, transparent agent architecture, vendor-neutral evaluation (OAI.co), NIST AI RMF alignment — plus local LLMs on-prem so student and research data never leaks.",
        example: "Our own governance is public — read the C2PA policy, model card and disclosure TOS on the Governance page, and reuse the template.",
        winners: "Winners own their data, train their students on governed tools, and can show counsel a paper trail instead of a promise.",
        tools: &[
            ("Websites", "authentichuman.co/governance, OAI.co"),
            ("Apps", "Governance board tooling, audit logs"),
            ("GitHub", "CIRISAgent, O-SAIF, Ollama, vLLM"),
            ("AIs", "Local open models, on-prem"),
            ("Hardware", "Campus GPU node or a watched local node"),
            ("Databases", "Our Ethical AI Framework DB"),
        ],
        links: &[],
        cta: "Download the free framework template — see the Ethical AI Framework service.",
    },
];

// ---------------------------------------------------------------------------
// Contact CTAs — WhatsApp + templated email (owner instruction 2026-10-27:
// every blog and service page; the message pre-fills what they clicked on)
// ---------------------------------------------------------------------------

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

fn contact_cta(interest: &str, tag: &str) -> String {
    let wa = url_encode(&format!("Hi Authentic Human — I'm interested in: {}. ", interest));
    let subject = url_encode(&format!("Interested in: {}", interest));
    let body = url_encode(&format!("Hi Authentic Human,\n\nI'm interested in: {}\n\nWhat I want: \n\nMy budget: \n", interest));
    format!(
        r##"<div class="cta-row"><a class="btn primary" href="https://wa.me/17134109994?text={wa}" target="_blank" rel="noopener">Text me on WhatsApp &rarr;</a><a class="btn" href="mailto:message+{tag}@pjrny.com?subject={subject}&amp;body={body}">Email me &rarr;</a></div><p class="texture">Both go straight to the founder. WhatsApp opens with what you clicked on pre-filled. The email opens addressed to message+{tag}@pjrny.com with a template — just add what you want and your budget.</p>"##,
        wa = wa, tag = tag, subject = subject, body = body
    )
}

fn links_row(links: &[(&str, &str)]) -> String {
    if links.is_empty() { return String::new(); }
    let mut s = String::from(r##"<h3>Live / free — go there</h3><div class="cta-row">"##);
    for (label, url) in links {
        s.push_str(&format!(r##"<a class="btn" href="{}" target="_blank" rel="noopener">{} &rarr;</a>"##, url, label));
    }
    s.push_str("</div>");
    s
}

// ---------------------------------------------------------------------------
// Page scaffolding
// ---------------------------------------------------------------------------

fn theme_options() -> String {
    let mut s = String::new();
    for t in THEMES {
        s.push_str(&format!("<option value=\"{}\">{} — {}</option>", t.slug, t.name, t.vibe));
    }
    s
}

fn head(title: &str, desc: &str, canonical: &str, prefix: &str, jsonld: bool) -> String {
    let ld = if jsonld {
        r##"<script type="application/ld+json">{"@context":"https://schema.org","@type":"Organization","name":"Authentic Human","url":"https://authentichuman.co/","foundingDate":"2018","address":{"@type":"PostalAddress","addressLocality":"Houston","addressRegion":"TX","addressCountry":"US"},"sameAs":["https://github.com/pjrny","https://pjrny.com"]}</script>"##
    } else { "" };
    format!(
        r##"<!DOCTYPE html>
<html lang="en" data-theme="minimalist">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<meta name="description" content="{desc}">
<link rel="canonical" href="{canonical}">
<meta property="og:type" content="website">
<meta property="og:site_name" content="Authentic Human">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{desc}">
<meta property="og:url" content="{canonical}">
<link rel="icon" type="image/svg+xml" href="{prefix}assets/logo/favicon.svg">
<link rel="icon" type="image/png" sizes="32x32" href="{prefix}assets/logo/favicon-32.png">
<link rel="apple-touch-icon" sizes="180x180" href="{prefix}assets/logo/apple-touch-icon.png">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="{fonts}" rel="stylesheet">
<link rel="stylesheet" href="{prefix}assets/base.css">
<link id="theme-css" data-base="{prefix}" rel="stylesheet" href="{prefix}assets/themes/minimalist.css">
<script>try{{var t=localStorage.getItem('ah-theme');if(t){{document.getElementById('theme-css').href='{prefix}assets/themes/'+t+'.css';document.documentElement.setAttribute('data-theme',t);}}}}catch(e){{}}</script>
{ld}
</head>
"##,
        title = title, desc = desc, canonical = canonical, prefix = prefix, fonts = FONTS_LINK, ld = ld
    )
}

fn header(prefix: &str) -> String {
    format!(
        r##"<body>
<header class="site"><div class="wrap header-in">
<a class="brand" href="{p}"><img id="brand-logo" src="{p}assets/logo/logo-minimalist-head.webp" alt="Authentic Human blank-face logo"> Authentic&nbsp;Human</a>
<nav class="main">
<a href="{p}#value">Value</a>
<a href="{p}#diagram">System</a>
<a href="{p}blog/">Blogs</a>
<a href="{p}#projects">Projects</a>
<a href="{p}#services">Services</a>
<a href="{p}governance/">Governance</a>
</nav>
<label class="switcher">Switch Reality
<select id="theme-select" aria-label="Switch visual style (reality)">{options}</select>
</label>
</div></header>
"##,
        p = prefix, options = theme_options()
    )
}

fn footer(prefix: &str) -> String {
    format!(
        r##"<footer class="site"><div class="wrap">
<p><strong>Authentic Human</strong> &bull; Houston, TX &bull; authentichuman.co &bull; Since 2018 &bull; Verify. Protect. Monetize.</p>
<p>The human in the loop. Human-first AI that builds the tools that build you.</p>
<p><a href="{p}governance/">Governance</a> &bull; <a href="{p}blog/">Blogs by niche</a> &bull; <a href="{p}#services">Services</a> &bull; <a href="https://github.com/pjrny">github.com/pjrny</a></p>
<p>Projects &amp; partners include Patron Journey (<a href="https://pjrny.com">pjrny.com</a>) — free ticketing, free websites and free event tools.</p>
</div></footer>
<a class="sticky-cta" href="{p}services/likeness-agent/">Verify My Likeness &rarr;</a>
<script src="{p}assets/app.js" defer></script>
</body>
</html>
"##,
        p = prefix
    )
}

fn page(title: &str, desc: &str, canonical: &str, prefix: &str, jsonld: bool, body: &str) -> String {
    let mut s = head(title, desc, canonical, prefix, jsonld);
    s.push_str(&header(prefix));
    s.push_str(body);
    s.push_str(&footer(prefix));
    s
}

// ---------------------------------------------------------------------------
// Home page
// ---------------------------------------------------------------------------

fn home_body() -> String {
    let mut blog_cards = String::new();
    for b in BLOGS {
        blog_cards.push_str(&format!(
            r##"<article class="card"><div class="tag">{maker}</div><h3>{niche}</h3><p>{hook}</p><p class="texture">Hero: {texture}</p><p><strong>Free tools inside:</strong> {first_tool}</p><a class="more" href="blog/{slug}/">Read the {niche} playbook &rarr;</a></article>"##,
            maker = b.maker, niche = b.niche, hook = b.hook, texture = b.texture,
            first_tool = b.tools[0].1, slug = b.slug
        ));
    }
    let mut service_cards = String::new();
    for s in SERVICES {
        service_cards.push_str(&format!(
            r##"<article class="card"><div class="tag">{tag}</div><h3>{name}</h3><p>{hook}</p><p class="mono" style="font-size:13px">{price}</p><a class="more" href="services/{slug}/">{name} &rarr;</a></article>"##,
            tag = s.tag, name = s.name, hook = s.hook, price = s.pricing[0], slug = s.slug
        ));
    }

    let mut s = String::new();
    // 1 — Hero
    s.push_str(r##"<main>
<section style="border-bottom:none;padding-top:44px"><div class="wrap">
<img class="hero-logo" id="hero-logo" src="assets/logo/logo-minimalist-head.webp" alt="Authentic Human — blank face logo">
<div class="eyebrow">Human-first AI agency &bull; Houston, TX</div>
<h1>Authentic<br><span class="serif-i">Human</span></h1>
<p class="lede">Verify. Protect. Monetize. <strong>The human in the loop.</strong></p>
<p class="lede" style="font-size:19px">We build AI for the people AI will try to replace — private, local, ownable tools that make authentic humans impossible to replace.</p>
<div><span class="chip solid">Verify. Protect. Monetize.</span><span class="chip">The human in the loop</span><span class="chip">Since 2018 — Jukebox &rarr; Music AI</span></div>
<div class="cta-row"><a class="btn primary" href="#blogs">Use the free tools</a><a class="btn" href="#services">Verify / work with us</a></div>
</div></section>
"##);
    // 2 — Value prop
    s.push_str(r##"<section id="value"><div class="wrap">
<div class="eyebrow">Value proposition</div>
<h2>Free tools. Private AI. Pay when you sell.</h2>
<div class="grid c2">
<div class="card"><h3>Free platform tools</h3><p>Websites, ticketing, CRM and email on an open platform — free for ideal customers. The subscription starts when <em>you</em> start charging <em>your</em> customers. Until then, it costs you nothing.</p></div>
<div class="card"><h3>Private / local AI</h3><p>Models run on hardware you control. Your likeness, voice, documents and customer data are <strong>never trained on</strong> and never leave without you. That is the whole model card, and it is public on our Governance page.</p></div>
<div class="card"><h3>No enterprise SaaS rent</h3><p>The incumbent stack — Salesforce + HubSpot + Mailchimp at real scale — runs about <strong>$2,025/mo, roughly $24K a year</strong> (our market research, 2026). Our platform is the free alternative underneath, and the services are where we earn.</p></div>
<div class="card"><h3>Knowledge is everywhere</h3><p>Implementation is what gets you success. Every niche blog below gives away the DIY version — free tools, the actual stack, the mistakes. Original ideas and follow-through are the core value of an authentic human.</p></div>
</div>
</div></section>
"##);
    // 3 — About
    s.push_str(r##"<section id="about"><div class="wrap">
<div class="eyebrow">About — short version</div>
<h2>Human-first. Irreplaceable.</h2>
<p class="lede">Authentic Human is a creative AI agency in Houston, TX. Purpose: build tools that make authentic humans impossible to replace. Mission: verify, protect and monetize authentic human identity, while forecasting instability through storytelling and simulation.</p>
<p>It started in 2018 with music generation in the Jukebox era (1M+ views, 2018–2020) — and with the realization that the tools creating human culture would next come for human identity itself. From Jukebox to Music AI: since 2018.</p>
<div class="grid c3">
<div class="tile"><div class="tag">Pillar 01</div><h3>Authenticity</h3><p>To make human authenticity — physically and creatively — the most valuable currency there is. Verified, protected, and owned by the human it belongs to.</p></div>
<div class="tile"><div class="tag">Pillar 02</div><h3>Storytelling</h3><p>One idea becomes social, PR, ebook, audio, music and poster. Stories are how humans learn what the data means.</p></div>
<div class="tile"><div class="tag">Pillar 03</div><h3>Forecasting Instability &amp; Simulation Gameplay</h3><p>Simulation and research on what AI does to jobs, markets and politics — before it does it. Fusion, policy and political-economic simulators: real forecasting you can play with, not entertainment.</p></div>
</div>
</div></section>
"##);
    // 4 — Winners / Losers (sourced research only)
    s.push_str(r##"<section id="winners"><div class="wrap">
<div class="eyebrow">Winners / Losers — the research</div>
<h2>The market is splitting, not just growing</h2>
<p>Commodity AI output gets cheaper every month. What commands a premium is integration, compliance and private infrastructure — and the demand numbers say so (sources labeled inline):</p>
<div class="grid c3">
<div class="tile"><div class="stat">+178%<small>Growth in AI integration work, year over year — Upwork</small></div></div>
<div class="tile"><div class="stat">+329%<small>Growth in AI video generation work — Upwork</small></div></div>
<div class="tile"><div class="stat">+109%<small>Growth in demand for AI skills overall — Upwork</small></div></div>
<div class="tile"><div class="stat">#1 / #2<small>LinkedIn Jobs on the Rise 2026: #1 AI Engineers, #2 AI Consultants &amp; Strategists</small></div></div>
<div class="tile"><div class="stat">77%<small>Of employers plan AI training for their people — WEF Future of Jobs</small></div></div>
<div class="tile"><div class="stat">DIY + DFY<small>Learn it yourself from our free blogs — or have us build it. Both doors stay open.</small></div></div>
</div>
<div class="grid c2" style="margin-top:18px">
<div class="card win"><h3>Winners</h3><ul class="tight">
<li>Wire AI into a real workflow — ticketing, dubbing, documentation — instead of chatting at it.</li>
<li>Own the pipeline and the data: local models, own platform, provenance on every asset.</li>
<li>Train their people (77% of employers plan to — WEF) and keep the human in the loop.</li>
<li>Sell the thing AI cannot copy: verified, authentic human identity and judgment.</li>
</ul></div>
<div class="card"><h3>Losers</h3><ul class="tight">
<li>Rent everything — avatar, audience, data — from platforms that can change the terms or take the account.</li>
<li>Keep paying 2019 prices for work AI collapsed the cost of (dubbing, ticketing, first-draft code).</li>
<li>Paste sensitive data into cloud models with no governance, no model card, no policy.</li>
<li>Wait for a perfect plan while competitors ship an imperfect one.</li>
</ul></div>
</div>
</div></section>
"##);
    // 5 — Diagram (Master System Map) + other 9 captions
    s.push_str(r##"<section id="diagram"><div class="wrap">
<div class="eyebrow">How it connects</div>
<h2>One platform, watched end to end</h2>
<p>This is the master map of everything Authentic Human runs — the same architecture our client work is built on.</p>
<div class="diagram" role="img" aria-label="Master system map: actors around an Odoo core, with home AI factory, code and edge, communications, money, and client site zones, plus the watched update loop">
<div class="d-row">
<span class="d-box">OPERATOR<br>(Authentic Human)</span>
<span class="d-box">CLIENT ADMIN</span>
<span class="d-box">CLIENT STAFF</span>
<span class="d-box">END CUSTOMERS / ATTENDEES</span>
<span class="d-box">AI AGENTS / MACHINES</span>
</div>
<div class="d-arrow">&#8595; requests &bull; orders &bull; portal &bull; API calls &#8595;</div>
<div class="d-row"><span class="d-box core">ODOO CORE — many databases, connected by APIs<br>CRM &bull; Websites &bull; eCommerce &bull; Events &bull; Invoicing</span></div>
<div class="d-arrow">&#8595; the core talks to five zones &#8595;</div>
<div class="d-row">
<span class="d-box zone"><strong>HOME AI FACTORY</strong><br>Coding box: Qwen + DeepSeek (open, uncensored) &rarr; coding agents<br>Graphics box: Wan + image models &rarr; media</span>
<span class="d-box zone"><strong>CODE &amp; EDGE</strong><br>Multiple GitHub repos<br>Multiple Cloudflare accounts<br>Domains on GoDaddy</span>
<span class="d-box zone"><strong>COMMS</strong><br>Mailjet = primary email sender<br>Odoo = backup sender</span>
<span class="d-box zone"><strong>MONEY</strong><br>Stripe &bull; Authorize.net + high-risk MID &bull; PayPal<br>x402 for machine / AI-service metering<br>Crypto wallets &rarr; 3 banks &rarr; Odoo invoicing</span>
<span class="d-box zone"><strong>CLIENT SITE</strong><br>Watched local node + on-site database<br>Configured / repurposed hardware<br>Online hosted version</span>
</div>
<div class="d-arrow">WATCHED LOOP: node reports health &rarr; operator monitors &rarr; updates (OS, models, config) pushed back &rarr; node reports new version</div>
</div>
<h3 style="margin-top:28px">The full diagram set — our build guide</h3>
<ol>
<li><strong>Master System Map</strong> — what exists, and who talks to whom (above).</li>
<li><strong>Multi-Odoo Topology</strong> — many databases, one API contract; source of truth per data type; the Pi is backup only, never live failover.</li>
<li><strong>Payments</strong> — payer + risk decides the rail: Stripe, Authorize.net + MID, PayPal, x402, wallets, banks, Odoo reconciliation.</li>
<li><strong>Data Flow &amp; Residency</strong> — what stays on the client node, what syncs to Odoo, what goes public, what is never stored (card numbers: tokens only).</li>
<li><strong>Domains, DNS &amp; Email</strong> — GoDaddy registrar, Cloudflare DNS (ours vs. the client's own), Mailjet primary / Odoo backup, SPF/DKIM/DMARC per sending subdomain.</li>
<li><strong>Users &amp; Permissions</strong> — who can do what, in which database; machines get scoped API keys, never a UI.</li>
<li><strong>Home AI Factory</strong> — coding lane and graphics lane, separate computers, both ending at a human approval gate.</li>
<li><strong>Code, Repos &amp; Deployment</strong> — core fork, shared modules, per-pack modules, per-client config, infrastructure; no secrets in Git.</li>
<li><strong>Client Lifecycle</strong> — free tools &rarr; client starts charging &rarr; subscription triggers &rarr; node installed &rarr; watched &rarr; next pack.</li>
<li><strong>Backup &amp; Failure</strong> — what keeps running when power, a platform, or a processor fails — and the restore drill that decides if a backup earns its place.</li>
</ol>
</div></section>
"##);
    // 6 — Blogs
    s.push_str(&format!(r##"<section id="blogs"><div class="wrap">
<div class="eyebrow">Blogs by niche — free DIY tools, insights, case studies</div>
<h2>Learn it here. Free.</h2>
<p>Every niche gets the full playbook: the shift, the losers' mistakes, the approach, a real example, and the free tools list. <a href="blog/">All 6 playbooks &rarr;</a></p>
<div class="grid c3">{cards}</div>
</div></section>
"##, cards = blog_cards));
    // 7 — Partners / Projects
    s.push_str(r##"<section id="projects"><div class="wrap">
<div class="eyebrow">Partners / Projects — labeled honestly</div>
<h2>Hired work vs. our own R&amp;D</h2>
<p>Case studies are work the founder was hired to build. Owned projects are our internal R&amp;D — labeled as such, never dressed up as client work.</p>
<h3>HIRED &bull; Case studies &amp; partners</h3>
<div class="grid c3">
<article class="card"><div class="tag">Hired &bull; Festival + Archive</div><h3>WorldFest-Houston</h3><p>One festival, two systems: <strong>flix.worldfest.org</strong> — film database + streaming, independent films, AI subtitles and dubbing, provenance on deliverables; and <strong>database.worldfest.org</strong> — the taxonomy archive / scientific database behind the festival's decades of films.</p><a class="more" href="https://flix.worldfest.org">flix.worldfest.org &rarr;</a><br><a class="more" href="https://database.worldfest.org">database.worldfest.org &rarr;</a></article>
<article class="card"><div class="tag">Hired &bull; Event OS</div><h3>Patron Journey</h3><p>Free ticketing and event OS for organizers at <strong>pjrny.com/organizers</strong> — QR check-in, organizer dashboard, zero ticket fees, organizers own their data — plus free websites and free tools like the event simulator at <strong>patronjourney.com</strong>, built for organizers and venues.</p><a class="more" href="https://pjrny.com/organizers">Free ticketing — pjrny.com/organizers &rarr;</a><br><a class="more" href="https://patronjourney.com">patronjourney.com &rarr;</a></article>
<article class="card"><div class="tag">Hired &bull; Festival OS</div><h3>festival.marketing</h3><p>Festival OS work with the Houston WorldFest partnership — the operating layer under festival marketing.</p><a class="more" href="https://festival.marketing">festival.marketing &rarr;</a></article>
</div>
<h3 style="margin-top:30px">OWNED &bull; Internal R&amp;D</h3>
<div class="grid c3">
<article class="card"><div class="tag">Owned &bull; R&amp;D</div><h3>LMA — OIMO</h3><p>The likeness monetization agent: Identify, Protect, Monetize authentic human likeness. Our long-term product, sold as the Likeness Agent service.</p><a class="more" href="services/likeness-agent/">Likeness Agent &rarr;</a></article>
<article class="card"><div class="tag">Owned &bull; R&amp;D</div><h3>DBE</h3><p>The scientific simulator and taxonomy observatory (DBE_V0): complex-systems modeling that runs on local AI agents.</p><a class="more" href="https://pjrny.github.io/DBE_V0/">Open DBE_V0 &rarr;</a><br><a class="more" href="services/scientific-simulator/">Scientific Simulator &rarr;</a></article>
<article class="card"><div class="tag">Owned &bull; R&amp;D</div><h3>PoliSim</h3><p>A political / business / economic simulator: test policy and strategy against Monte Carlo forecasts of instability before you commit.</p><a class="more" href="https://poli-eco-sim.kimi.page/">Try the Political Economic Simulator &rarr;</a><br><a class="more" href="blog/political-campaigns/">PoliSim playbook &rarr;</a></article>
</div>
</div></section>
"##);
    // 8 — Services
    s.push_str(&format!(r##"<section id="services"><div class="wrap">
<div class="eyebrow">Services</div>
<h2>What you can hire — and what is free</h2>
<div class="grid c3">
<article class="card"><div class="tag">Platform &bull; Free until you sell</div><h3>The Platform</h3><p>Free Odoo-based tools — websites, ticketing, CRM, email via Mailjet — plus a watched private node, coding-agent builds, and education / training. Free until you charge your own customers; then a subscription, with x402 metering for machine and AI services.</p><p class="mono" style="font-size:13px">Free tools &rarr; subscription when you sell</p><a class="more" href="#value">How the model works &rarr;</a></article>
{cards}
</div>
</div></section>
"##, cards = service_cards));
    // 9 — Governance teaser
    s.push_str(r##"<section id="governance"><div class="wrap">
<div class="eyebrow">Governance</div>
<h2>Governance is a product feature here</h2>
<div class="grid c2">
<article class="tile"><h3>C2PA provenance policy</h3><p>Every asset we produce carries Content Credentials, embedded at export. If no manifest — we didn't make it.</p></article>
<article class="tile"><h3>Model Card</h3><p>Training data: none of yours. Ever. Local inference only for client work; opt-out is automatic.</p></article>
<article class="tile"><h3>Ethics — NIST AI RMF + WCAG 2.2 AA</h3><p>Map, Measure, Manage, Govern — plus digital accessibility as a requirement, not an afterthought.</p></article>
<article class="tile"><h3>AI Disclosure TOS</h3><p>A copyable disclosure you can put in your own contracts — the same one we run under.</p></article>
</div>
<p style="margin-top:18px"><a class="btn" href="governance/">Read the full Governance page &rarr;</a></p>
</div></section>
</main>
"##);
    s
}

// ---------------------------------------------------------------------------
// Governance page (no people, no placeholder signatures — policy only)
// ---------------------------------------------------------------------------

fn governance_body() -> String {
    r##"<main>
<section><div class="wrap">
<div class="eyebrow">Governance &bull; Authentic Human</div>
<h1 style="font-size:clamp(38px,6vw,60px)">Governance</h1>
<p class="lede">How we handle likeness, data and AI output — in public, in writing. No client data in training. Ever. A human in the loop for every release.</p>
<div><span class="chip">C2PA: policy on every asset</span><span class="chip">Local first: client data stays put</span><span class="chip">Train on client data: NEVER</span></div>
</div></section>
<section><div class="wrap">
<div class="eyebrow">B &bull; C2PA Manifesto</div>
<h2>If no manifest &rarr; we didn't make it</h2>
<p>Every asset we produce gets Content Credentials. Not a sticker: a cryptographic manifest bonded to the file at creation. We sign in Houston, anyone can verify anywhere. If an asset claiming to be ours has no manifest, we didn't make it.</p>
<p>Verification tooling is the open C2PA standard: <span class="mono">c2patool --verify asset.jpg</span> shows the manifest, the actions taken, and the issuer. This page states the policy; each asset carries its own proof.</p>
</div></section>
<section><div class="wrap">
<div class="eyebrow">C &bull; Model Card</div>
<h2>Training data: none of yours</h2>
<table>
<tr><th>Component</th><th>Purpose</th><th>Data handling</th></tr>
<tr><td>Whisper.cpp (local)</td><td>Transcription / dubbing</td><td>Self-hosted, no data leaves our machines, working audio held in memory only</td></tr>
<tr><td>pyannote (local fork)</td><td>Speaker diarization</td><td>Local inference; speaker embeddings are ephemeral and deleted after the job</td></tr>
<tr><td>CLIP / SigLIP (local)</td><td>Likeness search</td><td>Local inference only; no vector database retention without consent</td></tr>
<tr><td>Foundation models</td><td>Caption, alt-text, summarization</td><td>Inference only; no fine-tuning on client likeness, film or event data</td></tr>
</table>
<h3 style="margin-top:24px">Opt-out policy</h3>
<p>All clients are automatically opted out of training. Any model-improvement use requires written opt-in, compensated. We keep a do-not-train registry. No dark patterns.</p>
<h3>What we don't do</h3>
<ul class="tight">
<li>No scraping festival submissions for training.</li>
<li>No training on patron or customer emails.</li>
<li>No biometric storage without documented consent.</li>
<li>No offshore labeling of sensitive likeness data.</li>
</ul>
<h3>Retention &amp; locality</h3>
<ul class="tight">
<li>Self-hosted in Houston, TX where possible; US-based infrastructure otherwise.</li>
<li>Client data deleted per the retention policy in your agreement — default 30 days after delivery.</li>
<li>Embeddings are ephemeral and are not persisted across jobs without consent.</li>
<li>Audit logs are retained, access-controlled, so provenance claims can be checked later.</li>
</ul>
</div></section>
<section><div class="wrap">
<div class="eyebrow">D &bull; Ethics / OrgLogic</div>
<h2>NIST AI RMF + OAI.co + digital ADA</h2>
<div class="grid c3">
<div class="tile"><h3>NIST AI RMF</h3><p><strong>Map</strong> the controls for likeness, festival and patron data. <strong>Measure</strong> bias and error — e.g. transcription accuracy across accents. <strong>Manage</strong> with human-review triggers for low confidence and any likeness of minors. <strong>Govern</strong> with a quarterly governance review and a full audit trail.</p></div>
<div class="tile"><h3>OAI.co alignment</h3><p>Model: inference only, no fine-tuning on client work. Data: no client data in training, ever. Human oversight: a human in the loop for all public-facing likeness use.</p></div>
<div class="tile"><h3>Digital ADA — WCAG 2.2 AA</h3><p>Caption-first media, screen-reader testing, keyboard navigation, 200% zoom, reduced motion respected, alt text on imagery — including the free tools we ship for clients.</p></div>
</div>
<h3 style="margin-top:24px">Risk matrix</h3>
<table>
<tr><th>Risk level</th><th>Examples</th><th>Handling</th></tr>
<tr><td>Low &rarr; automated</td><td>Automated captioning, transcription</td><td>Runs automatically, spot-checked</td></tr>
<tr><td>High &rarr; human review</td><td>Likeness search, public distribution, minors</td><td>Nothing ships without a human review</td></tr>
</table>
<p>Data is processed in the US, self-hosted in Texas where possible. Labeling, when needed, is done by reviewers under NDA in Houston.</p>
</div></section>
<section><div class="wrap">
<div class="eyebrow">E &bull; AI Disclosure</div>
<h2>AI Disclosure &amp; Data Use — copy this</h2>
<p>Copy-paste for your own contracts and counsel (MSA appendix-ready):</p>
<pre class="copy">AI Disclosure &amp; Data Use: Authentic Human and its projects do not train any AI model on client data, likeness, film submissions, or patron data. Ever. All assets include C2PA Content Credentials. Client data is processed locally or in US-based infrastructure and deleted per the retention policy in your agreement. You may opt out of all analytics. Contact: governance@authentichuman.co</pre>
</div></section>
</main>
"##.to_string()
}

// ---------------------------------------------------------------------------
// Service + blog pages
// ---------------------------------------------------------------------------

fn service_body(s: &Service) -> String {
    let mut proc = String::new();
    for p in s.process { proc.push_str(&format!("<li>{}</li>", p)); }
    let mut price = String::new();
    for p in s.pricing { price.push_str(&format!("<li>{}</li>", p)); }
    format!(
        r##"<main><section><div class="wrap">
<div class="eyebrow">Services &bull; {tag}</div>
<h1 style="font-size:clamp(34px,5.4vw,56px)">{title}</h1>
<p class="lede">{hook}</p>
<h3>Who this is for</h3><p>{icp}</p>
<h3>The pain</h3><p>{pain}</p>
<h3>How it works</h3><ol>{proc}</ol>
<h3>Pricing — the model</h3><ul class="tight">{price}</ul>
<h3>Proof</h3><p>{case}</p>
{links}
<h3>Next step</h3><p>{cta} — or just reach out, it goes straight to the founder (Houston, TX):</p>
{contact}
<p><a href="../../#services">&larr; All services</a></p>
</div></section></main>
"##,
        tag = s.tag, title = s.title, hook = s.hook, icp = s.icp, pain = s.pain,
        proc = proc, price = price, case = s.case, cta = s.cta,
        links = links_row(s.links), contact = contact_cta(s.name, s.slug)
    )
}

fn blog_body(b: &Blog) -> String {
    let mut rows = String::new();
    for (k, v) in b.tools { rows.push_str(&format!("<tr><th>{}</th><td>{}</td></tr>", k, v)); }
    format!(
        r##"<main><section><div class="wrap">
<div class="eyebrow">Blog &bull; {niche} &bull; {maker}</div>
<h1 style="font-size:clamp(34px,5.4vw,56px)">{niche}</h1>
<p class="texture">Hero — drastic difference: {texture}</p>
<p class="lede">{hook}</p>
<h3>Who is this for?</h3><p>{who}</p>
<h3>Who makes this?</h3><p>{maker} — the team behind the projects in this niche, writing up what actually worked.</p>
<h3>Problem — the big shift</h3><p>{problem}</p>
<h3>Losers + common mistakes</h3><p>{losers}</p>
<h3>Recommended approach</h3><p>{approach}</p>
<h3>Example + comparison</h3><p>{example}</p>
<h3>Winners + benefits</h3><p>{winners}</p>
<h3>Actionable tip — free tools list</h3>
<table>{rows}</table>
{links}
<h3 style="margin-top:24px">Next step</h3><p>{cta}</p>
{contact}
<p><a href="../">All 6 niche playbooks &rarr;</a></p>
</div></section></main>
"##,
        niche = b.niche, maker = b.maker, texture = b.texture, hook = b.hook, who = b.who,
        problem = b.problem, losers = b.losers, approach = b.approach, example = b.example,
        winners = b.winners, rows = rows, cta = b.cta,
        links = links_row(b.links), contact = contact_cta(&format!("{} playbook", b.niche), b.slug)
    )
}

fn blog_index_body() -> String {
    let mut cards = String::new();
    for b in BLOGS {
        cards.push_str(&format!(
            r##"<article class="card"><div class="tag">{maker}</div><h3>{niche}</h3><p>{hook}</p><p class="texture">Hero: {texture}</p><a class="more" href="{slug}/">Read the playbook &rarr;</a></article>"##,
            maker = b.maker, niche = b.niche, hook = b.hook, texture = b.texture, slug = b.slug
        ));
    }
    format!(
        r##"<main><section><div class="wrap">
<div class="eyebrow">Blogs by niche</div>
<h1 style="font-size:clamp(38px,6vw,60px)">Free tools. Real playbooks.</h1>
<p class="lede">One playbook per niche — the big shift, who loses and why, the approach, a real example, and the free tools list. Knowledge is everywhere; implementation is what gets you success.</p>
<div class="grid c3">{cards}</div>
</div></section></main>
"##,
        cards = cards
    )
}

// ---------------------------------------------------------------------------
// Generator
// ---------------------------------------------------------------------------

fn theme_css(t: &Theme) -> String {
    format!(
        "/* Authentic Human — theme: {name} ({slug})\n   {vibe} */\n:root{{\n--bg:{bg};\n--fg:{fg};\n--accent:{accent};\n--surface:{surface};\n--line:{line};\n--font-display:{fd};\n--font-body:{fb};\n--font-mono:{fm};\n--radius:{radius};\n}}\n{extra}\n",
        name = t.name, slug = t.slug, vibe = t.vibe, bg = t.bg, fg = t.fg, accent = t.accent,
        surface = t.surface, line = t.line, fd = t.font_display, fb = t.font_body,
        fm = t.font_mono, radius = t.radius, extra = t.extra
    )
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).expect("create dirs"); }
    fs::write(path, content).unwrap_or_else(|e| panic!("write {:?}: {}", path, e));
}

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dist = root.join("dist");
    if dist.exists() { fs::remove_dir_all(&dist).expect("clean dist"); }
    fs::create_dir_all(&dist).expect("create dist");

    // Assets: base css, app js, themes, logos (copied from assets/logo/)
    write(&dist.join("assets/base.css"), BASE_CSS);
    write(&dist.join("assets/app.js"), APP_JS);
    for t in THEMES {
        write(&dist.join(format!("assets/themes/{}.css", t.slug)), &theme_css(t));
    }
    let logo_src = root.join("assets/logo");
    let logo_dst = dist.join("assets/logo");
    fs::create_dir_all(&logo_dst).expect("logo dir");
    let mut logo_count = 0;
    for entry in fs::read_dir(&logo_src).expect("read assets/logo") {
        let entry = entry.expect("logo entry");
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(".png") || name.ends_with(".svg") || name.ends_with(".webp") {
            fs::copy(entry.path(), logo_dst.join(&name)).expect("copy logo");
            logo_count += 1;
        }
    }

    // Pages
    let mut urls: Vec<String> = Vec::new();
    let add = |urls: &mut Vec<String>, rel: &str, html: String| {
        write(&dist.join(rel), &html);
        urls.push(rel.replace("index.html", "").replace('\\', "/"));
    };

    add(&mut urls, "index.html", page(
        "Authentic Human — Verify. Protect. Monetize.",
        "Human-first AI agency in Houston, TX. Free platform tools, private local AI, likeness protection, simulators and storytelling. Verify. Protect. Monetize.",
        "https://authentichuman.co/", "", true, &home_body()));

    add(&mut urls, "governance/index.html", page(
        "Governance — Authentic Human",
        "Authentic Human governance: C2PA provenance policy, model card (training data: none of yours), NIST AI RMF ethics, WCAG 2.2 AA, AI disclosure TOS.",
        "https://authentichuman.co/governance/", "../", false, &governance_body()));

    for s in SERVICES {
        add(&mut urls, &format!("services/{}/index.html", s.slug), page(
            &format!("{} — Authentic Human", s.title),
            s.meta,
            &format!("https://authentichuman.co/services/{}/", s.slug),
            "../../", false, &service_body(s)));
    }

    add(&mut urls, "blog/index.html", page(
        "Blogs by Niche — Authentic Human",
        "Free DIY tools, insights and case studies per niche: film festivals, event organizers, fusion researchers, campaigns, creators and musicians, universities.",
        "https://authentichuman.co/blog/", "../", false, &blog_index_body()));

    for b in BLOGS {
        add(&mut urls, &format!("blog/{}/index.html", b.slug), page(
            &format!("{} — Authentic Human Blog", b.niche),
            &format!("{} Free tools and the playbook inside.", b.hook),
            &format!("https://authentichuman.co/blog/{}/", b.slug),
            "../../", false, &blog_body(b)));
    }

    // Musicians blog merged into Influencers/Creators 2026-10-27 — keep the
    // old URL working with a redirect (not in the sitemap).
    write(&dist.join("blog/musicians/index.html"),
        "<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><title>Moved — Authentic Human</title><meta http-equiv=\"refresh\" content=\"0; url=../influencers-creators/\"><link rel=\"canonical\" href=\"https://authentichuman.co/blog/influencers-creators/\"></head><body><p>The Musicians playbook now lives inside <a href=\"../influencers-creators/\">Influencers, Creators &amp; Musicians</a>.</p></body></html>\n");

    // 404 (absolute asset paths: Pages serves it at any depth)
    let mut nf = head("Page not found — Authentic Human",
        "That page does not exist. Authentic Human — Verify. Protect. Monetize.",
        "https://authentichuman.co/404.html", "/", false);
    nf.push_str("<body><main><section><div class=\"wrap\"><div class=\"eyebrow\">404</div><h1 style=\"font-size:clamp(38px,6vw,60px)\">Not found.</h1><p class=\"lede\">Thank you for looking. This human couldn't find that page either.</p><p><a class=\"btn primary\" href=\"/\">Back to Authentic Human</a></p></div></section></main></body></html>");
    write(&dist.join("404.html"), &nf);

    // robots, sitemap, CNAME
    write(&dist.join("robots.txt"),
        "User-agent: *\nAllow: /\nSitemap: https://authentichuman.co/sitemap.xml\nHost: https://authentichuman.co\n");
    let mut sm = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for u in &urls {
        sm.push_str(&format!("<url><loc>https://authentichuman.co/{}</loc></url>\n", u));
    }
    sm.push_str("</urlset>\n");
    write(&dist.join("sitemap.xml"), &sm);
    write(&dist.join("CNAME"), "authentichuman.co\n");

    println!("Built {} pages, {} theme files, {} logo assets into dist/", urls.len(), THEMES.len(), logo_count);
}
