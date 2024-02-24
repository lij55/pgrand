#![allow(dead_code)]

use std::str::FromStr;
use fake::Fake;
use pgrx::pg_sys::*;
use pgrx::{AnyNumeric, Date, GucContext, GucFlags, GucRegistry, GucSetting, IntoDatum, Time};

use rand::Rng;
use rand_chacha;
use rand_chacha::ChaCha8Rng;

use fake::faker;

pub struct RandomGUC {
    pub min_integer: GucSetting<i32>,
    pub max_integer: GucSetting<i32>,
    pub min_text_length: GucSetting<i32>,
    pub max_text_length: GucSetting<i32>,
    pub array_length: GucSetting<i32>,
    pub float_scale: GucSetting<i32>,
}

impl RandomGUC {
    pub const fn new() -> Self {
        Self {
            min_integer: GucSetting::<i32>::new(-10000),
            max_integer: GucSetting::<i32>::new(10000),
            min_text_length: GucSetting::<i32>::new(30000),
            max_text_length: GucSetting::<i32>::new(50000),
            array_length: GucSetting::<i32>::new(1024),
            float_scale: GucSetting::<i32>::new(1),
        }
    }

    pub fn init(&self) {
        GucRegistry::define_int_guc(
            "random.min_int",
            "",
            "",
            &self.min_integer,
            i32::MIN,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.max_int",
            "",
            "",
            &self.max_integer,
            i32::MIN,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.min_text_length",
            "",
            "",
            &self.min_text_length,
            3,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.max_text_length",
            "",
            "",
            &self.max_text_length,
            3,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.array_length",
            "",
            "",
            &self.array_length,
            1,
            16384,
            GucContext::Userset,
            GucFlags::default(),
        );

        GucRegistry::define_int_guc(
            "random.float_scale",
            "",
            "",
            &self.float_scale,
            1,
            i32::MAX,
            GucContext::Userset,
            GucFlags::default(),
        );
    }
}

pub static PARADE_GUC: RandomGUC = RandomGUC::new();

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

        DATEOID => unsafe {
            Date::from_pg_epoch_days(rng.gen_range(1 * 360..50 * 360)).into_datum()
        },
        TIMEOID => {
            let s = faker::time::en::Date().fake_with_rng(rng);
            Time::from_str(&s).unwrap().into_datum()
        },
        TIMESTAMPOID => None,
        UUIDOID => None,
        _ => None,
    }
}
