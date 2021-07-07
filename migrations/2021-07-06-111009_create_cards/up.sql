-- Creating the cards table 
CREATE TABLE cards (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR NOT NULL,
    external_url VARCHAR NOT NULL,
    sign VARCHAR NOT NULL,
    mnemonic VARCHAR NOT NULL,
    kana VARCHAR NOT NULL,     
    deck_id INTEGER NOT NULL
)