use fake::decimal::Decimal;
use fake::{Dummy, Fake, Faker};
use pgrx::pg_sys::*;
use pgrx::{AnyNumeric, IntoDatum, PgBuiltInOids, PgOid};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_chacha;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use supabase_wrappers::interface::Cell;

pub type DataBuilder = dyn Fn(&mut ChaCha8Rng) -> Option<Datum>;

pub fn create_closure(oid: Oid) -> Box<DataBuilder> {
    let min = 10;
    let max = 1000;
    let max_len = 29;
    // Box::new(move |rng: &mut ThreadRng| -> Cell {
    //     Cell::I64(rng.gen_range(min..max))
    // })
    match oid {
        INT2OID => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> {
            rng.gen_range(min as i16..max as i16).into_datum()
        }),
        FLOAT8ARRAYOID => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> {
            let mut values = Vec::new();
            for _i in 0..1024 {
                values.push(rng.gen_range(-1 as f64..1 as f64))
            }
            values.into_datum()
        }),
        _ => Box::new(move |rng: &mut ChaCha8Rng| -> Option<Datum> { None }),
    }
}

pub fn apply_builder<F>(f: F, rng: &mut ChaCha8Rng) -> Option<Datum>
where
    F: Fn(&mut ChaCha8Rng) -> Option<Datum>,
{
    f(rng)
}
