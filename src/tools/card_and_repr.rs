use super::arrays;
use std::str::Chars;

pub fn card_to_repr(card: Option<&str>) -> Option<u32> {
    let card = card?;
    let mut iterator: Chars = card.chars();
    let symbol: Option<char> = iterator.next();
    let color: Option<char> = iterator.next();

    let cdhs: u32;
    let rrrr: u32;
    let pppppp: u32;
    let bbbbbbbbbbbbb: u32;

    match symbol {
        Some('2') => {
            rrrr = 0x0 << 8;
            pppppp = arrays::PRIMES[0];
            bbbbbbbbbbbbb = 0x1 << 16
        }
        Some('3') => {
            rrrr = 0x1 << 8;
            pppppp = arrays::PRIMES[1];
            bbbbbbbbbbbbb = 0x1 << 17
        }
        Some('4') => {
            rrrr = 0x2 << 8;
            pppppp = arrays::PRIMES[2];
            bbbbbbbbbbbbb = 0x1 << 18
        }
        Some('5') => {
            rrrr = 0x3 << 8;
            pppppp = arrays::PRIMES[3];
            bbbbbbbbbbbbb = 0x1 << 19
        }
        Some('6') => {
            rrrr = 0x4 << 8;
            pppppp = arrays::PRIMES[4];
            bbbbbbbbbbbbb = 0x1 << 20
        }
        Some('7') => {
            rrrr = 0x5 << 8;
            pppppp = arrays::PRIMES[5];
            bbbbbbbbbbbbb = 0x1 << 21
        }
        Some('8') => {
            rrrr = 0x6 << 8;
            pppppp = arrays::PRIMES[6];
            bbbbbbbbbbbbb = 0x1 << 22
        }
        Some('9') => {
            rrrr = 0x7 << 8;
            pppppp = arrays::PRIMES[7];
            bbbbbbbbbbbbb = 0x1 << 23
        }
        Some('T') => {
            rrrr = 0x8 << 8;
            pppppp = arrays::PRIMES[8];
            bbbbbbbbbbbbb = 0x1 << 24
        }
        Some('J') => {
            rrrr = 0x9 << 8;
            pppppp = arrays::PRIMES[9];
            bbbbbbbbbbbbb = 0x1 << 25
        }
        Some('Q') => {
            rrrr = 0xa << 8;
            pppppp = arrays::PRIMES[10];
            bbbbbbbbbbbbb = 0x1 << 26
        }
        Some('K') => {
            rrrr = 0xb << 8;
            pppppp = arrays::PRIMES[11];
            bbbbbbbbbbbbb = 0x1 << 27
        }
        Some('A') => {
            rrrr = 0xc << 8;
            pppppp = arrays::PRIMES[12];
            bbbbbbbbbbbbb = 0x1 << 28
        }
        _ => return None,
    };

    match color {
        Some('C') => cdhs = 0x8 << 12,
        Some('D') => cdhs = 0x4 << 12,
        Some('H') => cdhs = 0x2 << 12,
        Some('S') => cdhs = 0x1 << 12,
        _ => return None,
    };

    Some(cdhs + rrrr + pppppp + bbbbbbbbbbbbb)
}

pub fn repr_to_card(card_repr: Option<u32>) -> Option<String> {
    let card_repr = card_repr?;
    let color: char;
    let symbol: char;

    match (card_repr & 0xf000) >> 12 {
        0x8 => color = 'C',
        0x4 => color = 'D',
        0x2 => color = 'H',
        0x1 => color = 'S',
        _ => return None,
    };

    match (card_repr & 0xf00) >> 8 {
        0x0 => symbol = '2',
        0x1 => symbol = '3',
        0x2 => symbol = '4',
        0x3 => symbol = '5',
        0x4 => symbol = '6',
        0x5 => symbol = '7',
        0x6 => symbol = '8',
        0x7 => symbol = '9',
        0x8 => symbol = 'T',
        0x9 => symbol = 'J',
        0xa => symbol = 'Q',
        0xb => symbol = 'K',
        0xc => symbol = 'A',
        _ => return None,
    };

    let card = format!("{}{}", symbol, color);
    Some(card)
}

// Tests of card_and_repr
#[cfg(test)]
mod tests {
    use super::card_to_repr;

    #[test]
    fn card_to_repr_tests() {
        let res = card_to_repr(Some("TH"));
        assert_eq!(res, Some(16787479));
    }

    #[test]
    fn card_to_repr_none_1() {
        let res = card_to_repr(Some("Z"));
        assert_eq!(res, None);
    }

    #[test]
    fn card_to_repr_none_2() {
        card_to_repr(Some("TT"));
        assert_eq!(card_to_repr(Some("TT")), None);
    }
}
