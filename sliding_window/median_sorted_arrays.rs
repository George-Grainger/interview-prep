fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let total = nums1.len() + nums2.len();
    nums1.push(i32::MAX);
    nums2.push(i32::MAX);

    let mut n1_idx = 0;
    let mut n2_idx = 0;
    while n1_idx + n2_idx < (total - 1) / 2 {
        if nums1[n1_idx] < nums2[n2_idx] {
            n1_idx += 1;
        } else {
            n2_idx += 1;
        }
    }

    let mut median_total = 0;
    let is_even = (total % 2 == 0) as usize;
    for i in 0..2 {
        if nums1[n1_idx] < nums2[n2_idx] {
            median_total += nums1[n1_idx];
            n1_idx += is_even;
        } else {
            median_total += nums2[n2_idx];
            n2_idx += is_even;
        }
    }

    median_total as f64 / 2.0
}
