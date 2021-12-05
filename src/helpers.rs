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

pub fn map_both<A, B, F: Fn(A) -> B>(f: F, (fst, snd): (A, A)) -> (B, B) {
    (f(fst), f(snd))
}
