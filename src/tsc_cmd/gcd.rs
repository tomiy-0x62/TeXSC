pub fn gcd(mut nums: Vec<u64>) -> u64 {
    assert!(nums.len() > 1);
    nums.sort_by(|a, b| b.cmp(a));
    let mut a = nums.pop().unwrap();
    for n in nums.iter().rev() {
        a = calc_gcd(*n, a);
    }
    a
}

fn calc_gcd(a: u64, b: u64) -> u64 {
    assert!(a >= b);
    if b == 0 {
        a
    } else {
        calc_gcd(b, a % b)
    }
}
