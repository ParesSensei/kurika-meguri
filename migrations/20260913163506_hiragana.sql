-- Add migration script here
CREATE TABLE IF NOT EXISTS hiragana (
    id SERIAL PRIMARY KEY,
    character VARCHAR(10) NOT NULL, -- あ, い, う, dll.
    romaji VARCHAR(10) NOT NULL,    -- a, i, u, dll.
    row_group VARCHAR(10) NOT NULL  -- 'vowel', 'k', 's', 't', 'n', 'h', 'm', 'y', 'r', 'w', 'n_solo'
    );

-- Index for query filter per row group
CREATE INDEX IF NOT EXISTS idx_hiragana_row_group ON hiragana(row_group);