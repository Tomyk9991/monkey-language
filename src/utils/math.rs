pub fn lowest_power_of_2_gt_n(n: usize) -> usize {
    let mut result: usize  = 1;
    while result < n {
        result <<= 1;
    }
    result
}