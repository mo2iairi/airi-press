import fs from 'node:fs';
import path from 'node:path';
import matter from 'front-matter';

export function generatePostsIndex() {
  const postsDir = path.resolve('public/posts');
  const outputFile = path.resolve('public/posts/index.json');

  if (!fs.existsSync(postsDir)) {
    console.log('Posts directory not found.');
    // Create it if it doesn't exist
    fs.mkdirSync(postsDir, { recursive: true });
    fs.writeFileSync(outputFile, JSON.stringify([], null, 2)); // Write empty array
    return;
  }

  const files = fs.readdirSync(postsDir).filter(file => file.endsWith('.md'));
  const posts = files.map(file => {
    const content = fs.readFileSync(path.join(postsDir, file), 'utf-8');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const parsed = matter(content) as any;
    const attrs = parsed.attributes || {};

    const idFromFilename = file.replace('.md', '');

    return {
      id: attrs.id || idFromFilename, // Allow ID to be explicitly set in frontmatter
      title: attrs.title || 'Untitled',
      date: attrs.date ? new Date(attrs.date).toISOString().split('T')[0] : '',
      description: attrs.description || '',
      tags: attrs.tags || [],
      category: attrs.category || 'General', // Default category
      imageUrl: attrs.imageUrl || attrs.cover || '', // Allow cover as alias
      pinnedWeight: Number(attrs.pinnedWeight) || 0 // Ensure number
    };
  });

  // Sort by pinnedWeight (desc) then by date (desc)
  posts.sort((a, b) => {
    const weightDiff = b.pinnedWeight - a.pinnedWeight;
    if (weightDiff !== 0) {
      return weightDiff;
    }
    return new Date(b.date).getTime() - new Date(a.date).getTime();
  });

  fs.writeFileSync(outputFile, JSON.stringify(posts, null, 2));
  console.log(`Generated posts index with ${posts.length} posts.`);
}

// If run directly (e.g. via 'npm run gen:posts')
generatePostsIndex();
