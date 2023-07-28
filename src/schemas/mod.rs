use diesel::prelude::*;

table! {
    engages (id) {
        id -> Integer,
        code -> VarChar,
        name -> VarChar,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct Engage {
    id: i32,
    code: String,
    name: String,
}
