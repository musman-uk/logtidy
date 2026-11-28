import fs from 'node:fs';
import path from 'node:path';
import { execSync } from 'node:child_process';

const BACKUP_DIR = '.apply-backup';
const rootDir = process.cwd();

function resolveSafePath(relPath) {
  const fullPath = path.resolve(rootDir, relPath);
  if (!fullPath.startsWith(rootDir + path.sep) && fullPath !== rootDir) {
    throw new Error(`Unsafe path blocked: ${relPath}`);
  }
  return fullPath;
}

const inputFile = process.argv[2];
const raw = inputFile
  ? fs.readFileSync(inputFile, 'utf8')
  : execSync(
      'powershell -NoProfile -Command "[Console]::OutputEncoding = [System.Text.Encoding]::UTF8; Get-Clipboard -Raw"',
      { encoding: 'utf8' }
    );

const pattern = /<<<FILE:\s*(.+?)>>>\r?\n([\s\S]*?)<<<END FILE>>>/g;
const matches = [...raw.matchAll(pattern)];

if (matches.length === 0) {
  console.log('No <<<FILE: ...>>> blocks found in clipboard. Nothing changed.');
  process.exit(0);
}

fs.mkdirSync(BACKUP_DIR, { recursive: true });
const seen = new Set();
const files = [];

for (const match of matches) {
  const relPath = match[1].trim();
  if (seen.has(relPath)) continue;
  seen.add(relPath);

  const fullPath = resolveSafePath(relPath);
  const existed = fs.existsSync(fullPath);
  files.push({
    path: relPath,
    existed,
    content: existed ? fs.readFileSync(fullPath, 'utf8') : null
  });
}

fs.writeFileSync(
  path.join(BACKUP_DIR, `snapshot-${Date.now()}.json`),
  JSON.stringify({ timestamp: new Date().toISOString(), files }, null, 2),
  'utf8'
);

for (const match of matches) {
  const relPath = match[1].trim();
  const fullPath = resolveSafePath(relPath);
  const content = match[2].replace(/\r?\n$/, '') + '\n';
  fs.mkdirSync(path.dirname(fullPath), { recursive: true });
  fs.writeFileSync(fullPath, content, 'utf8');
  console.log(`Updated: ${relPath}`);
}

console.log(`\nApplied ${matches.length} file(s). Run "node undo.mjs" anytime to revert.`);