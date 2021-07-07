table! {
    cards (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        image -> Varchar,
        external_url -> Varchar,
        sign -> Varchar,
        mnemonic -> Varchar,
        kana -> Varchar,
        deck_id -> Int4,
    }
}
