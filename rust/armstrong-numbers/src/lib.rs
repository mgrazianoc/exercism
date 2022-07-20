pub fn is_armstrong_number(num: u32) -> bool {
    // 123 -> "123" -> ["", "1", "2", "3"] -> [1, 2, 3] -> [0, 1, 2, 3] -(reduce)-> 36

    let mut digits: Vec<u32> = num.to_string()
                                .split_terminator("")
                                .skip(1)
                                .map(|z| z.parse::<u32>().unwrap())
                                .collect();

    let len: u32 = digits.len() as u32;

    digits.insert(0, 0);
    let sum = digits.into_iter().reduce(|x, y| x + y.pow(len)).unwrap();

    sum == num
}
