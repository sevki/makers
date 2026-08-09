#!/usr/bin/env node
// Captures a full-page PNG screenshot of each requested route against a
// running preview server, using the same file naming scheme the make-srv
// Rust job server expects (see Job::route_file_name).
import { mkdir } from "node:fs/promises";
import path from "node:path";
import { chromium } from "playwright";

function parseArgs(argv) {
  const args = { routes: [] };
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === "--base-url") args.baseUrl = argv[++i];
    else if (arg === "--routes") args.routes = argv[++i].split(",").filter(Boolean);
    else if (arg === "--out-dir") args.outDir = argv[++i];
    else throw new Error(`unknown argument: ${arg}`);
  }
  for (const required of ["baseUrl", "outDir"]) {
    if (!args[required]) throw new Error(`missing required --${required.replace(/[A-Z]/g, (c) => `-${c.toLowerCase()}`)}`);
  }
  if (args.routes.length === 0) args.routes = ["/"];
  return args;
}

function routeFileName(route) {
  const slug = route === "/" ? "root" : route.replace(/^\/+|\/+$/g, "").replaceAll("/", "-");
  return `${slug}.png`;
}

async function main() {
  const { baseUrl, routes, outDir } = parseArgs(process.argv.slice(2));
  await mkdir(outDir, { recursive: true });

  const browser = await chromium.launch({
    executablePath: process.env.PLAYWRIGHT_CHROMIUM_PATH || undefined,
  });
  try {
    const page = await browser.newPage({ viewport: { width: 1280, height: 800 } });
    for (const route of routes) {
      const url = new URL(route, baseUrl).toString();
      console.log(`capturing ${url}`);
      await page.goto(url, { waitUntil: "networkidle" });
      const outPath = path.join(outDir, routeFileName(route));
      await page.screenshot({ path: outPath, fullPage: true });
    }
  } finally {
    await browser.close();
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
