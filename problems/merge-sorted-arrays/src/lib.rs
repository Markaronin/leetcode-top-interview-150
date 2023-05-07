#[allow(dead_code)]
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        for i in (0..(m + n)).rev() {
            if m != 0 && (n == 0 || nums1[m - 1] >= nums2[n - 1]) {
                nums1[i] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn empty_merge() {
        let mut arr_1 = vec![];
        let mut arr_2 = vec![];
        let m = 0;
        let n = 0;
        Solution::merge(&mut arr_1, m, &mut arr_2, n);
        assert_eq!(arr_1, vec![])
    }

    #[test]
    fn one_empty_merge() {
        let mut arr_1 = vec![1];
        let mut arr_2 = vec![];
        let m = 1;
        let n = 0;
        Solution::merge(&mut arr_1, m, &mut arr_2, n);
        assert_eq!(arr_1, vec![1])
    }

    #[test]
    fn merge_one_number() {
        let mut arr_1 = vec![1, 0];
        let mut arr_2 = vec![2];
        let m = 1;
        let n = 1;
        Solution::merge(&mut arr_1, m, &mut arr_2, n);
        assert_eq!(arr_1, vec![1, 2])
    }

    #[test]
    fn interleave() {
        let mut arr_1 = vec![1, 3, 0, 0];
        let mut arr_2 = vec![2, 4];
        let m = 2;
        let n = 2;
        Solution::merge(&mut arr_1, m, &mut arr_2, n);
        assert_eq!(arr_1, vec![1, 2, 3, 4])
    }
}
