import fs from 'node:fs';
import path from 'node:path';

const BACKUP_DIR = '.apply-backup';
const rootDir = process.cwd();

function resolveSafePath(relPath) {
  const fullPath = path.resolve(rootDir, relPath);
  if (!fullPath.startsWith(rootDir + path.sep) && fullPath !== rootDir) {
    throw new Error(`Unsafe path blocked: ${relPath}`);
  }
  return fullPath;
}

if (!fs.existsSync(BACKUP_DIR)) {
  console.log('Nothing to undo – no backups found.');
  process.exit(0);
}

const snapshots = fs.readdirSync(BACKUP_DIR)
  .filter((f) => f.startsWith('snapshot-') && f.endsWith('.json'))
  .sort();

if (snapshots.length === 0) {
  console.log('Nothing to undo – no backups found.');
  process.exit(0);
}

const latest = snapshots.at(-1);
const snapshotPath = path.join(BACKUP_DIR, latest);
const manifest = JSON.parse(fs.readFileSync(snapshotPath, 'utf8'));

for (const entry of manifest.files) {
  const fullPath = resolveSafePath(entry.path);
  if (entry.existed) {
    fs.mkdirSync(path.dirname(fullPath), { recursive: true });
    fs.writeFileSync(fullPath, entry.content, 'utf8');
    console.log(`Restored: ${entry.path}`);
  } else if (fs.existsSync(fullPath)) {
    fs.unlinkSync(fullPath);
    console.log(`Removed new file: ${entry.path}`);
  }
}

fs.unlinkSync(snapshotPath);
if (snapshots.length === 1) {
  fs.rmSync(BACKUP_DIR, { recursive: true, force: true });
}

console.log(`\nReverted ${manifest.files.length} file(s) to their previous state.`);