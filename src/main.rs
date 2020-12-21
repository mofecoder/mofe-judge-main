extern crate diesel;

use cafecoder_rs::{db, model::*};
use diesel::{prelude::*, select};

fn main() {
    let db = db::establish_connection();

    use cafecoder_rs::schema::submits::dsl::submits.select()
}
