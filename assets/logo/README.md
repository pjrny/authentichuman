# Authentic Human — logo assets

The live logos are the user's ORIGINAL artworks (uploaded 2026-10-27),
chroma-keyed to transparency with Python PIL (corner-sampled key color,
distance-based feathered alpha, cropped to content). Untouched originals
are archived in `originals/` under their original filenames.

The theme switcher (assets/app.js) swaps the header + hero logo with the
theme: HEAD variant first, then the AH monogram, then the Minimalist HEAD
default. The Minimalist HEAD is the site's default logo.

## Mapping (original -> live file)

| Original (in originals/) | Reality | Variant | Live file | Treatment |
|---|---|---|---|---|
| e209d282….jpg | Minimalist | Head (line profile, AH + stars inside) | logo-minimalist-head.webp | keyed (green) |
| c9d88372….jpg | Minimalist | AH (plain black serif) | logo-minimalist-ah.webp | keyed (blue) |
| 84f4f2cf….jpg | Maximalist | Head (gold baroque filigree profile + wordmark) | logo-maximalist-head.webp | keyed (blue) |
| 2a26549d….jpg | Maximalist | AH (gold filigree head, AH inside) | logo-maximalist-ah.webp | keyed (green) |
| 21f5ba11….jpg | Brutalist | Head (concrete block face + bold wordmark) | logo-brutalist-head.webp | keyed (orange) |
| 377633b4….jpg | Brutalist | AH (stone frame, AH inside) | logo-brutalist-ah.webp | keyed (magenta) |
| 13262b75….jpg | Glassmorphism | Head (nebula head in frosted ring, AH inside) | logo-glassmorphism-head.webp | keyed (green) |
| aca800d6….jpg | Neumorphism | Head (embossed head tile + puffy wordmark) | logo-neumorphism-head.webp | keyed (magenta) |
| f46985aa….jpg | Neumorphism | AH (cream puffy 3D) | logo-neumorphism-ah.webp | keyed (magenta) |
| f44ca045….jpg | 2014 Tumblr | Head (grainy bust + "t" + wordmark) | logo-tumblr-head.webp | keyed (blue) |
| 72377d28….jpg | Alternative | Head (pompadour silhouette + wordmark) | logo-alternative-head.webp | keyed (blue) |
| aa30d9dd….jpg | Alternative | AH (distressed grunge) | logo-alternative-ah.webp | keyed (orange) |
| e07ac835….jpg | Americana | Head (denim profile in wheat swirl + western wordmark) | logo-americana-head.webp | keyed (magenta) |
| 59c5a60d….jpg | Americana | AH (denim AH in wheat wreath) | logo-americana-ah.webp | keyed (magenta) |
| 02926151….jpg | Americana (uncertain — see below) | AH alt (vintage parchment western) | logo-americana-ah-alt.webp | keyed (blue); processed but NOT wired as the primary AH |
| 7fbc3a66….jpg | Coquette | Head (pink silhouette with bows + wordmark) | logo-coquette-head.webp | keyed (green) |
| 6c1b8b8d….jpg | Coquette | AH (pink serif with ribbon bows) | logo-coquette-ah.webp | keyed (green) |
| 3ce4e2d8….jpg | Glitter Graphics | Head (pink glitter head + glitter wordmark) | logo-glitter-head.webp | keyed (orange) |
| 092672a4….jpg | Glitter Graphics | AH (pink glitter letters) | logo-glitter-ah.webp | keyed (orange) |
| 6a0c6c07….jpg | Hyperpop | AH (iridescent acid chrome) | logo-hyperpop-ah.webp | keyed (magenta) |
| 6372f34e….jpg | Punk Derivatives | Head (distressed silhouette + grunge wordmark) | logo-punk-head.webp | keyed (orange) |
| 138dd296….jpg | Punk Derivatives | AH (safety pins + ransom note) | logo-punk-ah.webp | keyed (green) |
| 50d3b73d….jpg | Regional Gothic | AH (blackletter AH in swamp-vine wreath) | logo-regional-gothic-ah.webp | keyed (magenta) |
| b33c575a….jpg | SoundCloud Rap | Head (white line head, galaxy inside + wordmark) | logo-soundcloud-head.webp | keyed (magenta) |
| 26a89bbe….jpg | SoundCloud Rap | AH (starfield bust outline, AH on chest) | logo-soundcloud-ah.webp | keyed (magenta) |
| 93a84f2e….jpg | Y2K | AH (silver chrome balloons) | logo-y2k-ah.png | keyed (blue) |

## Archive-only — NOT in the live rotation

| Original | Why |
|---|---|
| 3c68fcab….jpg (Retro visitor-counter badge, "SINCE 1997") | Embedded date contradicts the confirmed "Since 2018" lineage (user decision 2026-10-27). Archived only; the Retro theme falls back to the Minimalist HEAD via the standard fallback chain. |
| a778d438….jpg (Retro visitor-counter badge, "EST. 1995") | Same as above. |

## No logo supplied

- Eurotrash — no artwork could be confidently assigned; falls back to Minimalist HEAD.
- TempleOS — no logo by design; the site favicon.svg (blank-face motif) stands in.

## Icons

- favicon.svg — blank-face motif, remains the primary icon.
- favicon-32.png / apple-touch-icon.png — derived from the Minimalist HEAD
  mark. The PNG reads cleanly on light tab bars but is low-contrast on dark
  ones, so it is wired as a secondary icon + touch icon; the SVG stays primary.

## Uncertain assignments (flagged, not guessed silently)

- 02926151….jpg (parchment western AH): Americana already has a confident AH
  (59c5a60d, same denim/wheat motif as its head), so this was processed as an
  alternate Americana AH and is not wired into the switcher.
- b33c575a + 26a89bbe (white-line cosmic pair): assigned to SoundCloud Rap by
  pairing (magenta/purple starfield); no other reality was unclaimed for them.
- 6a0c6c07 (acid chrome AH): assigned to Hyperpop (acid-green/pink chrome
  matches the Hyperpop palette); Eurotrash is the other chrome-styled reality
  and remains without a logo.
- 6372f34e (distressed head): assigned Punk Derivatives head (its AH partner
  138dd296 is unambiguous); Alternative was the other candidate, and
  Alternative's head is the pompadour silhouette (72377d28).
