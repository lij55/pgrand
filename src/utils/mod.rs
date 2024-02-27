#![allow(dead_code)]

mod data;
pub mod guc;

use data::*;
use pgrx::pg_sys::*;
use pgrx::{AnyNumeric, Date, Inet, IntoDatum, Time, Uuid};
use std::str::FromStr;

use fake::faker;
use fake::{Fake, Faker};
use guc::PARADE_GUC;
use rand::Rng;
use rand_chacha;
use rand_chacha::ChaCha8Rng;

pub type DataBuilder = dyn Fn(&mut ChaCha8Rng) -> Option<Datum>;

pub fn create_closure(oid: Oid) -> Box<DataBuilder> {
    let min = 10;
    let max = 1000;
    let _max_len = 29;
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
        _ => Box::new(move |_rng: &mut ChaCha8Rng| -> Option<Datum> { None }),
    }
}

pub fn apply_builder<F>(f: F, rng: &mut ChaCha8Rng) -> Option<Datum>
where
    F: Fn(&mut ChaCha8Rng) -> Option<Datum>,
{
    f(rng)
}

pub fn generate_random_data_for_oid(oid: Oid, rng: &mut ChaCha8Rng) -> Option<Datum> {
    let min_int = PARADE_GUC.min_integer.get() as i16;
    let max_int = PARADE_GUC.max_integer.get() as i16;

    let min_text_len = PARADE_GUC.min_text_length.get() as usize;
    let max_text_len = PARADE_GUC.max_text_length.get() as usize;
    let array_len = PARADE_GUC.array_length.get() as u32;
    let float_factor: u32 = PARADE_GUC.float_scale.get() as u32;
    // log!("{oid}");
    match oid {
        INT2OID => rng.gen_range(min_int / 2 as i16..max_int).into_datum(),
        INT4OID => rng.gen_range(min_int..max_int).into_datum(),
        INT8OID => rng
            .gen_range(min_int as i64..max_int as i64 * 2)
            .into_datum(),
        FLOAT4ARRAYOID => {
            let mut values = Vec::new();
            for _i in 0..array_len {
                values.push(rng.gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32))
            }
            values.into_datum()
        }
        FLOAT8ARRAYOID => {
            let mut values = Vec::new();
            for _i in 0..array_len {
                values.push(rng.gen_range(-1.0 * float_factor as f64..1.0 * float_factor as f64))
            }
            values.into_datum()
        }
        BOOLOID => (rng.gen_range(0..=1) != 0).into_datum(),
        //CHAROID => Some(3.into()),
        FLOAT4OID => rng
            .gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32)
            .into_datum(),
        FLOAT8OID => rng
            .gen_range(-1.0 * float_factor as f64..1.0 * float_factor as f64)
            .into_datum(),

        NUMERICOID => AnyNumeric::try_from(
            rng.gen_range(-1.0 * float_factor as f32..1.0 * float_factor as f32),
        )
        .unwrap_or_default()
        .into_datum(),

        TEXTOID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        VARCHAROID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        BPCHAROID => (min_text_len..max_text_len)
            .fake_with_rng::<std::string::String, ChaCha8Rng>(rng)
            .into_datum(),

        DATEOID => {
            let s: std::string::String = faker::time::en::Date().fake_with_rng(rng);
            Date::from_str(s.as_str()).unwrap().into_datum()
        }
        TIMEOID => {
            let s: std::string::String = random_time(rng);

            Time::from_str(s.as_str()).unwrap().into_datum()
        }
        TIMESTAMPOID => {
            Timestamp::from(rng.gen_range(i64::MIN / 128..i64::MAX / 1024)).into_datum()
        }
        UUIDOID => {
            let bytes = Faker.fake::<[u8; 16]>();
            Uuid::from_bytes(bytes).into_datum()
        }

        INETOID => {
            let addr = random_ip(rng);
            Inet::from(addr).into_datum()
        }
        _ => None,
    }
}
