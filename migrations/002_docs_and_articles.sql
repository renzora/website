-- Documentation pages (admin-managed)
CREATE TABLE IF NOT EXISTS docs (
    id UUID PRIMARY KEY,
    slug VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    category VARCHAR(64) NOT NULL DEFAULT 'general',
    sort_order INT NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_docs_slug ON docs(slug);
CREATE INDEX idx_docs_category ON docs(category);

-- Community articles (user-submitted)
CREATE TABLE IF NOT EXISTS articles (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    summary TEXT NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    tags TEXT[] NOT NULL DEFAULT '{}',
    cover_image_url TEXT,
    published BOOLEAN NOT NULL DEFAULT false,
    likes INT NOT NULL DEFAULT 0,
    views INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_articles_slug ON articles(slug);
CREATE INDEX idx_articles_author ON articles(author_id);
CREATE INDEX idx_articles_tags ON articles USING GIN(tags);

-- Article likes (prevent duplicate likes)
CREATE TABLE IF NOT EXISTS article_likes (
    user_id UUID NOT NULL REFERENCES users(id),
    article_id UUID NOT NULL REFERENCES articles(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, article_id)
);

-- Article comments
CREATE TABLE IF NOT EXISTS article_comments (
    id UUID PRIMARY KEY,
    article_id UUID NOT NULL REFERENCES articles(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_article ON article_comments(article_id);
