// Render the authored storyboard HTML to the atlas PNG with headless Chromium at
// 2× so each 406×776 screen is an 812×1552 crop. Usage:
//   node render_atlas.mjs <storyboard.html> <atlas.png> <width> <height>
// Uses the workspace Playwright install (Octosense-website/node_modules).
import {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
const [html, png, width, height] = process.argv.slice(2);
const website = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const {chromium} = createRequire(path.join(website, 'package.json'))('playwright');
const browser = await chromium.launch();
try {
  const page = await browser.newPage({viewport: {width: Number(width) / 2, height: Number(height) / 2}, deviceScaleFactor: 2});
  await page.goto('file://' + path.resolve(html), {waitUntil: 'load'});
  await page.evaluate(() => document.fonts.ready);
  await page.screenshot({path: png, fullPage: false, animations: 'disabled'});
  console.log(JSON.stringify({png, chromium: browser.version()}));
} finally { await browser.close(); }
