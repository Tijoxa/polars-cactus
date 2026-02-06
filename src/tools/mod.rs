// This script is a Rust package adaptation of the Cactus Kev Perfect Hash algorithm
// which evaluates a poker hand by giving it a score from 1 (best) to 7462 (worst)
// https://suffe.cool/poker/evaluator.html

// +--------+--------+--------+--------+
// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
// +--------+--------+--------+--------+

pub mod arrays;
pub mod card_and_repr;

pub fn find_fast(mut u: u32) -> u32 {
    u += 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= u >> 4;
    let b: u32 = (u >> 8) & 0x1ff;
    let a: u32 = (u + (u << 2)) >> 19;
    let hash: u32 = arrays::HASH_ADJUST[b as usize];
    let c: u32 = a ^ hash;

    c
}

pub fn eval_5cards(c1: u32, c2: u32, c3: u32, c4: u32, c5: u32) -> u32 {
    let q: u32 = (c1 | c2 | c3 | c4 | c5) >> 16;

    // This checks for Flushes and Straight Flushes
    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return arrays::FLUSHES[q as usize];
    }

    // This checks for Straights and High Card hands
    let unique5: u32 = arrays::UNIQUE_5[q as usize];
    if unique5 != 0 {
        return unique5;
    }

    // This performs a perfect-hash lookup for remaining hands
    let q2: u32 = (c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff);
    let result: u32 = arrays::HASH_VALUES[find_fast(q2) as usize];

    result
}

#[cfg(test)]
mod lib_tests {
    use super::eval_5cards;

    #[test]
    fn eval_5cards_tests() {
        let res_1 = eval_5cards(8398611, 4204049, 2106637, 1057803, 16787479);
        assert_eq!(res_1, 5);

        let res_2 = eval_5cards(268471337, 295429, 557831, 16812055, 268446761);
        assert_eq!(res_2, 3484);
    }
}
