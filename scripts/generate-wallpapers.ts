import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Configuration
const PUBLIC_DIR = path.resolve(__dirname, '../public');
const WALLPAPER_DIR = path.join(PUBLIC_DIR, 'wallpaper');
const SETTINGS_FILE = path.join(PUBLIC_DIR, 'settings.json');
const ALLOWED_EXTS = ['.webp', '.png', '.jpg', '.jpeg', '.gif'];

function getAllFiles(dirPath: string, arrayOfFiles: string[] = [], rootDir: string) {
  const files = fs.readdirSync(dirPath);

  files.forEach((file) => {
    const fullPath = path.join(dirPath, file);
    if (fs.statSync(fullPath).isDirectory()) {
      getAllFiles(fullPath, arrayOfFiles, rootDir);
    } else {
      const ext = path.extname(file).toLowerCase();
      if (ALLOWED_EXTS.includes(ext)) {
        // Create relative path from wallpaper dir, ensure forward slashes for web
        const relativePath = path.relative(rootDir, fullPath).replace(/\\/g, '/');
        arrayOfFiles.push(relativePath);
      }
    }
  });

  return arrayOfFiles;
}

function updateSettings() {
  if (!fs.existsSync(SETTINGS_FILE)) {
    console.error('Settings file not found!');
    process.exit(1);
  }

  try {
    const settings = JSON.parse(fs.readFileSync(SETTINGS_FILE, 'utf-8'));
    const wallpaperFiles = getAllFiles(WALLPAPER_DIR, [], WALLPAPER_DIR);

    console.log(`Found ${wallpaperFiles.length} wallpapers.`);

    let updated = false;

    // Find the appearance section -> wallpaper item
    for (const section of settings) {
      if (section.section_key === 'appearance') {
        for (const item of section.items) {
          if (item.id === 'wallpaper') {
            // Update path to root wallpaper dir
            item.path = '/wallpaper/';
            // Update file list
            item.files = wallpaperFiles;
            updated = true;
            break;
          }
        }
      }
    }

    if (updated) {
      fs.writeFileSync(SETTINGS_FILE, JSON.stringify(settings, null, 2));
      console.log('Updated settings.json with new wallpapers.');
    } else {
      console.warn('Wallpaper setting item not found in settings.json');
    }

  } catch (error) {
    console.error('Error processing settings:', error);
  }
}

updateSettings();
