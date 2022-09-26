#[test]
fn try_parse() {
    let strings = &vec!["3", "93", "18"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", numbers);

    let new_number: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().err())
        .collect();
    println!("Results by Filter: {:?}", new_number);

    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    let mut kjd: Vec<u8> = Default::default();
    kjd.extend(vec![2]);
    println!("{:?}", kjd)
}
