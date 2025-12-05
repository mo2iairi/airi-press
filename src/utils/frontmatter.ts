export interface FrontMatterResult {
  attributes: Record<string, any>;
  body: string;
}

export function parseFrontMatter(text: string): FrontMatterResult {
  const pattern = /^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/;
  const match = text.match(pattern);

  if (!match) {
    return {
      attributes: {},
      body: text
    };
  }

  const yaml = match[1];
  const body = match[2] || ''; // Ensure string
  const attributes: Record<string, any> = {};

  if (yaml) {
    yaml.split('\n').forEach(line => {
      const parts = line.split(':');
      if (parts.length >= 2) {
        const key = parts[0]?.trim();
        if (!key) return; // Skip if no key
        
        let value = parts.slice(1).join(':').trim();
        
        // Simple array parsing [a, b]
        if (value.startsWith('[') && value.endsWith(']')) {
          // @ts-ignore
          value = value.slice(1, -1).split(',').map(v => v.trim());
        }
        
        attributes[key] = value;
      }
    });
  }

  return { attributes, body };
}
