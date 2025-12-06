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
    'calculator': 'Calculator',
    'music': 'Music',

    // Music App
    music_library: 'Library',
    music_no_song: 'Not Playing',
    music_select_playlist: 'Select a playlist',
    songs: 'Songs',
    play: 'Play',

    // Calculator App
    calculator_app: 'Calculator & Tools',
    calc_science: 'Scientific',
    calc_base64: 'Base64',
    calc_hash: 'Hash',
    calc_time: 'Timestamp',
    calc_base: 'Base Conv.',
    text_input: 'Input Text',
    enter_text: 'Type something...',
    calculate: 'Calculate',
    now: 'Current Time',
    refresh: 'Refresh',
    timestamp_to_date: 'Timestamp -> Date',
    date_to_timestamp: 'Date -> Timestamp',

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
                    'calculator': '计算器',
                    'music': '音乐',
                
                    // Music App
                    music_library: '音乐库',
                    music_no_song: '未播放',
                    music_select_playlist: '选择歌单',
                    songs: '首歌曲',
                    play: '播放全部',
                
                    // Calculator App
                    calculator_app: '全能计算器',                calc_science: '科学计算',
                calc_base64: 'Base64',
                calc_hash: '哈希计算',
                calc_time: '时间戳',
                calc_base: '进制转换',
                text_input: '输入文本',
                enter_text: '输入内容...',
                calculate: '计算',
                now: '当前时间',
                refresh: '刷新',
                timestamp_to_date: '时间戳 -> 日期',
                date_to_timestamp: '日期 -> 时间戳',
            
                // Post List
                search_placeholder: '搜索文章...',            filter_category: '分类',
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
