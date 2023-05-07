#[allow(dead_code)]
struct Solution {}
impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn empty_array() {
        let mut nums = vec![];
        let val = 0;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(nums, vec![]);
        assert_eq!(k, 0);
    }

    #[test]
    fn no_removals() {
        let mut nums = vec![1, 2, 3, 4];
        let val = 0;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(nums, vec![1, 2, 3, 4]);
        assert_eq!(k, 4);
    }

    #[test]
    fn interleaved_removals() {
        let mut nums = vec![1, 2, 3, 2, 4];
        let val = 2;
        let k = Solution::remove_element(&mut nums, val);
        assert_eq!(nums[0..k as usize], [1, 3, 4]);
        assert_eq!(k, 3);
    }
}
