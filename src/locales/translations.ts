export const translations = {
  en: {
    // Settings
    settings: 'Settings',
    general: 'General',
    language: 'Language',
    zh_cn: 'Simplified Chinese', // New key
    en_us: 'English',            // New key
    timezone: 'Timezone',
    appearance: 'Appearance',
    theme: 'Theme',
    light: 'Light',
    dark: 'Dark',
    glass: 'Glass',
    wallpaper: 'Wallpaper',
    about: 'About',
    version: 'Version',
    
    // Status Bar
    home: 'Home',
    
    // Apps (IDs)
    'clock': 'Clock',
    'posts': 'Posts',
    'github': 'GitHub',

    // Post List
    search_placeholder: 'Search posts...',
        filter_category: 'Category',
        filter_tag: 'Tags',
        filter_date: 'Date',
        all_categories: 'All Categories',
        all_tags: 'All Tags',
        start_date: 'Start Date',
        end_date: 'End Date',
        no_posts: 'No posts found.',
    
        // Post Editor
        post_title: 'Post Title',
        post_date: 'Date',
        post_category: 'Category',
        post_tags: 'Tags (comma separated)',
        post_cover: 'Cover Image URL',
        post_id: 'Post ID',
        post_description: 'Description',
            post_pinned_weight: 'Pinned Weight (Higher = More Pinned)',
            start_writing: 'Start writing...',
            edit_mode: 'Edit',
            preview_mode: 'Preview',
        
            // GitHub Config
            github_config: 'GitHub Config',
            gh_owner: 'Repo Owner',
            gh_repo: 'Repo Name',
            gh_branch: 'Branch',
            gh_token: 'Personal Access Token'
          },
          zh: {
            // Settings
            settings: '设置',
            general: '通用',
            language: '语言',
            zh_cn: '简体中文', // New key
            en_us: '英文',      // New key
            timezone: '时区',
            appearance: '外观',
            theme: '主题',
            light: '浅色',
            dark: '深色',
            glass: '毛玻璃',
            wallpaper: '壁纸',
            about: '关于',
            version: '版本',
            
            // Status Bar
            home: '主页',
            
            // Apps (IDs)
            'clock': '时钟',
            'posts': '文章',
            'github': 'GitHub',
        
            // Post List
            search_placeholder: '搜索文章...',
            filter_category: '分类',
            filter_tag: '标签',
            filter_date: '日期',
            all_categories: '所有分类',
            all_tags: '所有标签',
            start_date: '开始日期',
            end_date: '结束日期',
            no_posts: '没有找到文章。',
        
            // Post Editor
            post_title: '文章标题',
            post_date: '日期',
            post_category: '分类',
            post_tags: '标签 (逗号分隔)',
            post_cover: '封面图片 URL',
            post_id: '文章 ID',
            post_description: '简述',
            post_pinned_weight: '置顶权重 (数字越大越靠前)',
            start_writing: '开始写作...',
            edit_mode: '编辑',
            preview_mode: '预览',
        
            // GitHub Config
            github_config: 'GitHub 配置',
            gh_owner: '仓库所有者',
            gh_repo: '仓库名称',
            gh_branch: '分支',
            gh_token: '个人访问令牌 (PAT)'
          }
        };
        export type Language = keyof typeof translations;
export type TranslationKey = keyof typeof translations.en;
