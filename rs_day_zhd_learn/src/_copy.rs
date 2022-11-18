#[test]
fn copy_mai() {
    let a = Box::new(std::cell::RefCell::new(1));
    let b = Box::new(std::cell::RefCell::new(15));

    *b.borrow_mut() = *a.borrow_mut();
    println!("b={:?}", b);

    let mut v = vec![];
    let vv = &v;
    v.push(1);

    {
        v.push(2);
    }
    println!("{:?}", v)
}

#[test]
fn do_chart1() {
    let s = "abl?opc?d";
    let mut chars = s.chars().collect::<Vec<char>>();

    for i in 0..s.len() {
        let mut words = ('a'..='z').into_iter();
        if chars[i] == '?' {
            let left = if i == 0 { None } else { Some(chars[i - 1]) };
            let right = if i == s.len() - 1 {
                None
            } else {
                Some(chars[i + 1])
            };
            chars[i] = words
                .find(|&w| Some(w) != left && Some(w) != right)
                .unwrap();
        }
    }
    let kk = chars.into_iter().collect::<String>();
    println!("{}", kk)
}
