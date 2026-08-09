# make-srv

A job server and dashboard that screenshots the make-srv UI and posts the
results as a comment on the pull request that changed it.

- `server/` — Rust (axum) job server. Accepts screenshot jobs over HTTP,
  runs several concurrently (bounded by `MAX_CONCURRENT_JOBS`), captures
  screenshots with Playwright, and publishes them to GitHub: it pushes the
  PNGs to the `screenshot-artifacts` branch and posts a PR comment
  embedding them.
- `ui/` — React + Vite dashboard (shadcn/ui, with the `theme-toggle`
  component pulled from the [ui.devtools.ltd](https://ui.devtools.ltd)
  registry) that lists jobs and their live status, and lets you enqueue one
  by hand.

## Running locally

```sh
# server
cargo run -p make-srv -- \
  --port 8787 \
  --ui-dist src/make-srv/ui/dist \
  --screenshot-script src/make-srv/ui/scripts/screenshot.mjs

# UI (dev mode, proxies /api to the server above)
cd src/make-srv/ui && npm install && npm run dev
```

Set `GITHUB_TOKEN` for the server to publish screenshots; without it, jobs
still capture screenshots but skip posting to GitHub.

## CI

`.github/workflows/ui-screenshot-job.yml` builds the server and UI, boots
both, and enqueues a job for the PR's own preview whenever `src/make-srv/**`
changes.
