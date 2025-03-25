use std::sync::LazyLock;
use threadpool::ThreadPool;

static TABLE: LazyLock<[u32; 256]> = std::sync::LazyLock::new(|| {
    let mut table = [0; 256];
    for n in 0..256 {
        let mut c = n as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xedb88320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        table[n] = c;
    }
    table
});

static ALPHABET: LazyLock<Vec<char>> = std::sync::LazyLock::new(|| {
    std::fs::read_to_string("alphabet.txt")
        .expect("File alphabet.txt not found")
        .chars()
        .collect()
});

static HASHES: LazyLock<Vec<u32>> = std::sync::LazyLock::new(|| {
    let mut list: Vec<u32> = std::fs::read_to_string("hashes.txt")
        .expect("File hashes.txt not found")
        .split("\n")
        .map(|x| u32::from_str_radix(x.trim(), 16).expect(&format!("Invalid hash {x}")))
        .collect();
    list.sort();
    list
});

fn crc32(str: &str) -> u32 {
    let mut crc = !0;

    for c in str.chars() {
        crc = (crc >> 8) ^ TABLE[(crc ^ c as u32) as usize & 0xff];
    }

    !(crc ^ !0)
}

fn main() {
    let pool = ThreadPool::new(4);

    for i in &*ALPHABET {
        for j in &*ALPHABET {
            pool.execute(move || {
                for x in &*ALPHABET {
                    for y in &*ALPHABET {
                        for a in &*ALPHABET {
                            for b in &*ALPHABET {
                                for c in &*ALPHABET {
                                    let str = format!("{}{}{}{}{}{}{}", i, j, x, y, a, b, c);
                                    let hash = crc32(&str);
                                    if HASHES.contains(&hash) {
                                        println!("Hash: 0x{:x}  {}", hash, &str);
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }
    }
    pool.join(); // Wait for all threads to finish

    println!("Press any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
