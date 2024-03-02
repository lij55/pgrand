use rand_chacha::ChaCha8Rng;
use std::ops::RangeInclusive;
use fake::Fake;

use fake::faker::internet;

use pgrx::pg_sys::Point;
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