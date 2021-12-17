use crate::{read, Solution};

fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        // :04b turn number to binary and pad with preceding 0 to a total width of 4
        .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect()
}

fn eval(op: &str, literals: Vec<u64>) -> u64 {
    match op {
        "000" => literals.iter().sum(),
        "001" => literals.iter().product(),
        "010" => *literals.iter().min().unwrap(),
        "011" => *literals.iter().max().unwrap(),
        "101" => (literals[0] > literals[1]) as u64,
        "110" => (literals[0] < literals[1]) as u64,
        "111" => (literals[0] == literals[1]) as u64,
        _ => unreachable!(),
    }
}

fn parse_literal(binary: &str, mut pos: usize) -> (u64, usize) {
    let mut value = String::new();
    loop {
        let bits = &binary[pos..pos + 5];
        pos += 5;
        value.push_str(&bits[1..]);

        if bits.starts_with("0") {
            return (u64::from_str_radix(&value, 2).unwrap(), pos);
        }
    }
}

fn parse_by_sub_count(binary: &str, mut pos: usize) -> (usize, u64, Vec<u64>) {
    let num_sub_packets = usize::from_str_radix(&binary[pos + 1..pos + 12], 2).unwrap();
    pos += 12;
    let mut version = 0;
    let results = (0..num_sub_packets)
        .into_iter()
        .map(|_| {
            let (p, v, r) = parse(binary, pos);
            pos = p;
            version += v;
            r
        })
        .collect();

    (pos, version, results)
}

fn parse_by_bits(binary: &str, mut pos: usize) -> (usize, u64, Vec<u64>) {
    let bits = usize::from_str_radix(&binary[pos + 1..pos + 16], 2).unwrap();
    pos += 16;
    let stop_at = pos + bits;
    let mut version = 0;
    let mut results = vec![];

    while pos < stop_at {
        let (p, v, r) = parse(binary, pos);
        pos = p;
        version += v;
        results.push(r);
    }

    (pos, version, results)
}

fn parse(binary: &str, mut pos: usize) -> (usize, u64, u64) {
    // No more packet can be parsed
    if pos + 6 > binary.len() {
        return (binary.len(), 0, 0);
    }
    let mut version = u64::from_str_radix(&binary[pos..pos + 3], 2).unwrap();
    let type_id = &binary[pos + 3..pos + 6];
    pos += 6;

    let result = if type_id == "100" {
        let (literal, p) = parse_literal(binary, pos);
        pos = p;
        literal
    } else {
        // operator
        let (p, v, results) = if &binary[pos..pos + 1] == "1" {
            parse_by_sub_count(binary, pos)
        } else {
            parse_by_bits(binary, pos)
        };

        pos = p;
        version += v;
        eval(type_id, results)
    };
    (pos, version, result)
}

/* Solutions */

fn part01(input: &String) -> u64 {
    parse(&input, 0).1
}

fn part02(input: &String) -> u64 {
    parse(&input, 0).2
}

pub fn day_16() -> Solution {
    let input = hex_to_binary(&read("./input/day_16.txt"));
    let timer = std::time::Instant::now();
    Solution::new(16, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hex_to_binary_d2fe28() {
        assert_eq!(
            hex_to_binary("D2FE28"),
            String::from("110100101111111000101000")
        );
    }

    #[test]
    fn test_hex_to_binary_38006f45291200() {
        assert_eq!(
            hex_to_binary("38006F45291200"),
            String::from("00111000000000000110111101000101001010010001001000000000")
        );
    }

    #[test]
    fn test_part_1_8a004a801a8002f478() {
        assert_eq!(part01(&hex_to_binary("8A004A801A8002F478")), 16)
    }

    #[test]
    fn test_part_1_620080001611562c8802118e34() {
        assert_eq!(part01(&hex_to_binary("620080001611562C8802118E34")), 12)
    }

    #[test]
    fn test_part_1_c0015000016115a2e0802f182340() {
        assert_eq!(part01(&hex_to_binary("C0015000016115A2E0802F182340")), 23)
    }

    #[test]
    fn test_part_1_a0016c880162017c3686b18a3d4780() {
        assert_eq!(part01(&hex_to_binary("A0016C880162017C3686B18A3D4780")), 31)
    }

    #[test]
    fn test_part_2_sum() {
        assert_eq!(part02(&hex_to_binary("C200B40A82")), 3)
    }

    #[test]
    fn test_part_2_product() {
        assert_eq!(part02(&hex_to_binary("04005AC33890")), 54)
    }

    #[test]
    fn test_part_2_min() {
        assert_eq!(part02(&hex_to_binary("880086C3E88112")), 7)
    }

    #[test]
    fn test_part_2_max() {
        assert_eq!(part02(&hex_to_binary("CE00C43D881120")), 9)
    }

    #[test]
    fn test_part_2_lt() {
        assert_eq!(part02(&hex_to_binary("D8005AC2A8F0")), 1)
    }

    #[test]
    fn test_part_2_gt() {
        assert_eq!(part02(&hex_to_binary("F600BC2D8F")), 0)
    }

    #[test]
    fn test_part_2_eq() {
        assert_eq!(part02(&hex_to_binary("9C005AC2F8F0")), 0)
    }

    #[test]
    fn test_part_2_sum_prod_eq() {
        assert_eq!(part02(&hex_to_binary("9C0141080250320F1802104A08")), 1)
    }

    #[test]
    fn test_part01() {
        let input = &hex_to_binary(&read("./input/day_16.txt"));
        assert_eq!(part01(&input), 886)
    }

    #[test]
    fn test_part02() {
        let input = &hex_to_binary(&read("./input/day_16.txt"));
        assert_eq!(part02(&input), 184487454837)
    }
}
