use rand_chacha::ChaCha8Rng;
use std::ops::RangeInclusive;
use fake::Fake;
use num_traits::pow;
use fake::faker::internet;
use pgrx::{IntoDatum, pg_sys, PgMemoryContexts};

use pgrx::pg_sys::{float8, Point, log};
use rand::Rng;


const RANGE12: RangeInclusive<u32> = 0..=11;
const RANGE24: RangeInclusive<u32> = 0..=23;
const RANGE60: RangeInclusive<u32> = 0..=59;
const RANGEFF: RangeInclusive<u32> = 0..=255;

pub fn random_time(rng: &mut ChaCha8Rng) -> String {
    let hour = RANGE24.fake_with_rng::<u32, ChaCha8Rng>(rng);
    let minute = RANGE60.fake_with_rng::<u32, ChaCha8Rng>(rng);
    let second = RANGE60.fake_with_rng::<u32, ChaCha8Rng>(rng);
    format!("{hour}:{minute}:{second}")

}

pub fn random_ip(rng: &mut ChaCha8Rng) -> String {
    internet::en::IPv4().fake_with_rng(rng)
}

pub fn random_numeric(rng: &mut ChaCha8Rng, scale: u8, precision: u8) -> f32 {
    let s:u64 = rng
        .gen_range( 0..pow(10, scale as usize)) as u64;

    let p:u64 = rng.gen_range(0..pow(10, precision as usize)) as u64;
    log!("{s}.{p}");
    s as f32 + p as f32 / pow(10, precision as usize) as f32



}

pub fn random_point(rng: &mut ChaCha8Rng, range: u32) -> Point {
    // 100 to keep 2 precision
    let c = 100.0 * range as f64;
    Point {
        x: rng
            .gen_range(-1.0 * c..1.0 * c).floor() / 100.0,
        y: rng
            .gen_range(-1.0 * c..1.0 * c).floor() / 100.0
    }
}

pub struct RndCIRCLE {
    pub center: Point,
    pub radius: float8,
}

impl RndCIRCLE {
    pub fn new(rng: &mut ChaCha8Rng) -> Self {
        RndCIRCLE {
            center: random_point(rng, 100),
            radius: rng
                .gen_range(0.0..10.0f64)
        }
    }
}

impl IntoDatum for RndCIRCLE {
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        unsafe {
            let ptr = PgMemoryContexts::CurrentMemoryContext
                .copy_ptr_into(&mut self, std::mem::size_of::<pg_sys::CIRCLE>());
            Some(ptr.into())
        }
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::CIRCLEOID
    }
}