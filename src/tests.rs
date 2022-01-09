
#[test]
fn hash_gen_test() {
    use sha2::Digest;
    use sha2::Sha512;

    let (seed, hash) = super::generate_random_seed_and_its_hash();
    let test_hash: [u8; 64] = Sha512::digest(&seed.to_le_bytes()).as_slice().try_into().unwrap();

    assert!(hash == test_hash);

    for h in hash {
        print!("{:x}", h);
    }
    print!("\n");
    println!("{}", seed);
}

#[test]
fn test_stuff() {
    use super::generate_random_seed_and_its_hash;
    use sha2::{Digest, Sha512};

    type Client = (u16, [u8; 64]);

    let mut clients: [Client; 4] = [(0, [0; 64]); 4];

    for c in &mut clients {
        *c = generate_random_seed_and_its_hash();
    }

    let mut temp: [u8; 64];
    for c in clients {
        for test_c in clients {
            if test_c == c {
                continue;
            }
            temp = Sha512::digest(&test_c.0.to_le_bytes()).as_slice().try_into().unwrap();
            assert!(temp == test_c.1, "failed hash check");
        }
    }

    let rand_val = super::generate_random_from_seeds(clients.into_iter().map(|x| x.0).collect::<Vec<u16>>().try_into().unwrap()) % 7;
    println!("{}", rand_val);
}

