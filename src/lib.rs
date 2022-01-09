//#![allow(unused)]

use rand::Rng;
#[cfg(test)]
mod tests;

fn generate_random_from_seeds(seeds: [u16; 4]) -> u8 {
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    let mut seed: u64 = 0;
    for i in 0..4 {
        seed |= (seeds[i] as u64) << (16 * i);
    }

    return Pcg64::seed_from_u64(seed).next_u32() as u8;
}

pub fn generate_random_seed_and_its_hash() -> (u16, [u8; 64]) {
    use sha2::{Digest, Sha512};
    let seed: u16 = rand::thread_rng().gen();

    let hash: [u8; 64] = Sha512::digest(&seed.to_le_bytes()).as_slice().try_into().unwrap();

    return (seed, hash);
}

fn validate_seed(seed: u16, hash: [u8; 64]) -> bool {
    use sha2::{Digest, Sha512};
    return hash == Sha512::digest(&seed.to_le_bytes()).as_slice();
}

pub fn check_and_generate_random(seeds: [(u16, [u8; 64]); 4]) -> Result<u8, Box<dyn std::error::Error>> {
    for (s, h) in seeds {
        if !validate_seed(s, h) {
            return Err(format!("invalid hash or seed: sha512({}) != {:?}", s, h).into());
        }
    }
    
    // this try_into() cannot fail
    return Ok(generate_random_from_seeds(seeds.into_iter().map(|(s, _)| s).collect::<Vec<u16>>().try_into().unwrap()));
}

