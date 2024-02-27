use rand_chacha::ChaCha8Rng;
use std::ops::RangeInclusive;
use fake::Fake;

use fake::faker::internet;


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