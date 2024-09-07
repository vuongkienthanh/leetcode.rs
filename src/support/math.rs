pub fn gcd(a: i32, b: i32) -> i32 {
    if b==0 {
        return a;
    }
    gcd(b, a % b)
}
pub fn lcm(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}
