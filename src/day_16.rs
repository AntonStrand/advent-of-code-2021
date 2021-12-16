use crate::{helpers::map_fst, read, Solution};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Gt,
    Lt,
    Eq,
}

impl Operator {
    fn new(operator: &str) -> Self {
        match from_binary(operator) {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::Gt,
            6 => Self::Lt,
            _ => Self::Eq,
        }
    }

    fn apply(&self, values: Vec<u64>) -> u64 {
        let eval = |boolean| if boolean { 1 } else { 0 };
        let vals = values.clone().into_iter();
        println!("Apply {:?} to {:?}", self, values);
        match self {
            Operator::Sum => vals.sum(),
            Operator::Product => vals.product(),
            Operator::Minimum => vals.min().unwrap(),
            Operator::Maximum => vals.max().unwrap(),
            Operator::Gt => eval(values.get(0).unwrap() > values.get(1).unwrap()),
            Operator::Lt => eval(values.get(0).unwrap() < values.get(1).unwrap()),
            Operator::Eq => eval(values.get(0).unwrap() == values.get(1).unwrap()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct OperatorData {
    version: u64,
    operator: Operator,
    subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct LiteralData {
    version: u64,
    value: u64,
}

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Literal(LiteralData),
    Operator(OperatorData),
}

impl Packet {
    fn new_literal(version: u64, value: u64) -> Self {
        Packet::Literal(LiteralData { version, value })
    }

    fn new_operator(version: u64, operator: Operator, subpackets: Vec<Self>) -> Self {
        Packet::Operator(OperatorData {
            version,
            subpackets,
            operator,
        })
    }

    fn version(packet: Packet) -> u64 {
        match packet {
            Self::Literal(data) => data.version,
            Self::Operator(data) => {
                data.version
                    + data
                        .subpackets
                        .into_iter()
                        .map(Packet::version)
                        .sum::<u64>()
            }
        }
    }

    fn calculate(packet: Packet) -> u64 {
        match packet {
            Self::Literal(data) => data.value,
            Self::Operator(data) => {
                let values = data
                    .subpackets
                    .clone()
                    .into_iter()
                    .map(Packet::calculate)
                    .collect();

                data.operator.apply(values)
            }
        }
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(data) => write!(f, "(Literal {})", data.value),
            Self::Operator(data) => {
                write!(f, "(Operator {:?}, {:#?})", data.operator, data.subpackets)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TypeId {
    Literal,
    Operator(Operator),
}

impl TypeId {
    fn from_str(s: &str) -> Self {
        match s {
            "100" => Self::Literal,
            _ => Self::Operator(Operator::new(s)),
        }
    }
}

fn from_binary(binary: &str) -> u64 {
    u64::from_str_radix(binary, 2).unwrap()
}

fn usize_from_binary(binary: &str) -> usize {
    usize::from_str_radix(binary, 2).unwrap()
}

// This would have been better suited for a library but I want to do it myself
fn hex_to_binary(hex: &str) -> String {
    hex.to_uppercase()
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Not hex: {}", c),
        })
        .collect::<Vec<&str>>()
        .join("")
}

fn parse(max_packet: usize, mut start_binary: &str) -> (Vec<Packet>, &str) {
    let mut packets = vec![];

    loop {
        if start_binary.trim_end_matches('0').is_empty() || packets.len() == max_packet {
            break;
        }

        let (version, bin) = map_fst(from_binary, start_binary.split_at(3));
        let (type_id, mut binary) = map_fst(TypeId::from_str, bin.split_at(3));

        match type_id {
            TypeId::Literal => {
                let mut literal = "".to_string();
                let mut bin = binary;
                loop {
                    let (part, rest) = bin.split_at(5);
                    literal = literal + &part[1..];
                    bin = rest;
                    if part.starts_with("0") {
                        packets.push(Packet::new_literal(version, from_binary(literal.as_str())));
                        break;
                    }
                }
                binary = bin;
            }
            TypeId::Operator(operator) => {
                let (bits, bin) =
                    map_fst(|t| if t == "0" { 15usize } else { 11 }, binary.split_at(1));

                let (sub_len, rest) = map_fst(usize_from_binary, bin.split_at(bits));
                let (subpackets, remainer) = parse(sub_len, rest);

                packets.push(Packet::new_operator(version, operator, subpackets));

                binary = remainer;
            }
        }

        start_binary = binary;
    }

    (packets, start_binary)
}

/* Solutions */

fn part01(input: &String) -> u64 {
    parse(usize::MAX, &input)
        .0
        .into_iter()
        .map(Packet::version)
        .sum()
}

fn part02(input: &String) -> u64 {
    // parse(usize::MAX, &input)
    //     .0
    //     .into_iter()
    //     .for_each(|p| println!("{:?}", p));
    parse(usize::MAX, &input)
        .0
        .into_iter()
        .map(Packet::calculate)
        .sum::<u64>()
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
    fn test_hex_to_binary_D2FE28() {
        assert_eq!(
            hex_to_binary("D2FE28"),
            String::from("110100101111111000101000")
        );
    }

    #[test]
    fn test_hex_to_binary_38006F45291200() {
        assert_eq!(
            hex_to_binary("38006F45291200"),
            String::from("00111000000000000110111101000101001010010001001000000000")
        );
    }

    #[test]
    fn test_part_1_8A004A801A8002F478() {
        assert_eq!(part01(&hex_to_binary("8A004A801A8002F478")), 16)
    }

    #[test]
    fn test_part_1_620080001611562C8802118E34() {
        assert_eq!(part01(&hex_to_binary("620080001611562C8802118E34")), 12)
    }

    #[test]
    fn test_part_1_C0015000016115A2E0802F182340() {
        assert_eq!(part01(&hex_to_binary("C0015000016115A2E0802F182340")), 23)
    }

    #[test]
    fn test_part_1_A0016C880162017C3686B18A3D4780() {
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
        assert_eq!(part02(&input), 2914)
    }
}
