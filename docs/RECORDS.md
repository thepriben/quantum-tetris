# Records and leaderboards

## Score formula

```
score = time_left_ms × purity_multiplier
purity_multiplier = 1.0 + 0.1 × (6 - forced_observations)
```

- **time_left_ms** — milliseconds remaining when you exit the Municipal Gate (base timer: 180_000 ms)
- **forced_observations** — times the player used "Observe" on an Imp

Lower observations and faster clears yield higher scores.

## Local storage

File: `records/local.json`

```json
{
  "entries": [
    {
      "seed": "2026-05-30-la-default",
      "time_left_ms": 84200,
      "score": 117880,
      "shards": 6,
      "observations": 1,
      "backend": "qip",
      "version": "0.1.0",
      "played_at": "2026-05-30T12:00:00Z"
    }
  ]
}
```

Keep top **10** entries per seed, sorted by `score` descending.

## Daily seed

```
seed = first 8 hex chars of SHA256( UTC_date + anchor_lat + anchor_lon )
```

Same seed for all players on a given day enables fair comparison.

## Web leaderboard (Sprint 8)

- WASM client exports run JSON
- Optional POST to a minimal API (Cloudflare Worker or static regeneration)
- Display top 10 in egui main menu

## Anti-cheat (MVP)

Lightweight input hash + seed checksum. Pedagogical project, not esports-grade validation.
