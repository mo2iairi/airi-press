import { defineConfig, type ViteDevServer } from 'vite'
import vue from '@vitejs/plugin-vue'
import fs from 'node:fs'
import path from 'node:path'
import { exec } from 'node:child_process'

// Custom plugin for local post management
const localPostManager = () => {
  return {
    name: 'local-post-manager',
    configureServer(server: ViteDevServer) {
      if (process.env.VITE_LOCAL_MODE !== 'true') return;

      server.middlewares.use('/api/save-post', async (req: any, res: any, next: any) => {
        if (req.method === 'POST') {
          let body = '';
          req.on('data', (chunk: any) => {
            body += chunk.toString();
          });

          req.on('end', () => {
            try {
              const { filename, content } = JSON.parse(body);
              const filePath = path.resolve(__dirname, 'public/posts', filename);

              // Write file
              fs.writeFileSync(filePath, content);

              // Regenerate index
              exec('npm run gen:posts', (err, stdout, stderr) => {
                if (err) {
                  console.error('Failed to regenerate posts:', stderr);
                  res.statusCode = 500;
                  res.end(JSON.stringify({ error: 'Failed to regenerate index' }));
                  return;
                }
                console.log('Posts regenerated:', stdout);
                res.statusCode = 200;
                res.end(JSON.stringify({ success: true }));
              });

            } catch (e) {
              console.error(e);
              res.statusCode = 500;
              res.end(JSON.stringify({ error: 'Failed to save file' }));
            }
          });
        } else {
          next();
        }
      });
    }
  }
}

// https://vite.dev/config/
export default defineConfig({
  base: process.env.VITE_DEPLOY_TARGET === 'github' 
    ? process.env.BASE_URL || '/airi-press/' // Fallback or set via env
    : '/',
  plugins: [
    vue(),
    localPostManager()
  ],
})
