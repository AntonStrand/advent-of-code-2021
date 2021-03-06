pub fn to_columns<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let row_length = input.get(0).unwrap().len();
    let mut columns = vec![];

    for x in 0..row_length {
        let mut column = vec![];
        for y in 0..input.len() {
            column.push(input[y][x]);
        }
        columns.push(column);
    }

    columns
}

pub fn map_pair<A, B, F: Fn(A) -> B>(f: F, (fst, snd): (A, A)) -> (B, B) {
    (f(fst), f(snd))
}

pub fn map_fst<A, B, C, F: Fn(A) -> C>(f: F, (fst, snd): (A, B)) -> (C, B) {
    (f(fst), snd)
}

pub fn map_snd<A, B, C, F: Fn(B) -> C>(f: F, (fst, snd): (A, B)) -> (A, C) {
    (fst, f(snd))
}

pub fn map_both<A, B, C, D, F, G>(f: F, g: G, (fst, snd): (A, B)) -> (C, D)
where
    F: Fn(A) -> C,
    G: Fn(B) -> D,
{
    (f(fst), g(snd))
}

pub fn parse_number_string<N>(string: &String) -> Vec<N>
where
    N: std::str::FromStr,
{
    string
        .split("")
        .filter_map(|n| {
            if n.is_empty() {
                None
            } else {
                Some(unsafe_parse(n))
            }
        })
        .collect()
}

pub fn unsafe_parse<N: std::str::FromStr>(value: &str) -> N {
    value
        .parse::<N>()
        .ok()
        .unwrap_or_else(|| panic!("Failed to parse value: {}", value))
}

pub fn sort_desc<T: Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    let mut v = vec.clone();
    v.sort_by_key(|&w| std::cmp::Reverse(w));
    v
}

pub fn pipe<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

pub fn is_upper(string: &str) -> bool {
    string.to_uppercase() == string
}
