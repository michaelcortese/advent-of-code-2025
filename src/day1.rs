pub fn solve(t: Vec<String>, j: usize) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    for i in t {
        let (rot, amount) = i.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        match rot {
            "L" => pos -= amount,
            "R" => pos += amount,
            _ => {}
        }

        if pos % 100 == 0 {
            count += 1;
        }
    }
    count
}
