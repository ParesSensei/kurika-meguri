-- Add migration script here
INSERT INTO hiragana (character, romaji, row_group) VALUES
-- Vowel
('あ', 'a', 'vowel'), ('い', 'i', 'vowel'), ('う', 'u', 'vowel'), ('え', 'e', 'vowel'), ('お', 'o', 'vowel'),
-- K-row
('か', 'ka', 'k'), ('き', 'ki', 'k'), ('く', 'ku', 'k'), ('け', 'ke', 'k'), ('こ', 'ko', 'k'),
-- S-row
('さ', 'sa', 's'), ('し', 'shi', 's'), ('す', 'su', 's'), ('せ', 'se', 's'), ('そ', 'so', 's'),
-- T-row
('た', 'ta', 't'), ('ち', 'chi', 't'), ('つ', 'tsu', 't'), ('て', 'te', 't'), ('と', 'to', 't'),
-- N-row
('な', 'na', 'n'), ('に', 'ni', 'n'), ('ぬ', 'nu', 'n'), ('ね', 'ne', 'n'), ('の', 'no', 'n'),
-- H-row
('は', 'ha', 'h'), ('ひ', 'hi', 'h'), ('ふ', 'fu', 'h'), ('へ', 'he', 'h'), ('ほ', 'ho', 'h'),
-- M-row
('ま', 'ma', 'm'), ('み', 'mi', 'm'), ('む', 'mu', 'm'), ('め', 'me', 'm'), ('も', 'mo', 'm'),
-- Y-row
('や', 'ya', 'y'), ('ゆ', 'yu', 'y'), ('よ', 'yo', 'y'),
-- R-row
('ら', 'ra', 'r'), ('り', 'ri', 'r'), ('る', 'ru', 'r'), ('れ', 're', 'r'), ('ろ', 'ro', 'r'),
-- W-row & N
('わ', 'wa', 'w'), ('を', 'wo', 'w'), ('ん', 'n', 'n_solo');