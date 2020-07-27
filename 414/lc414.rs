impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<_> = nums.drain(..).collect(); // dedup
        nums.extend(set.into_iter());
        nums.sort();
        nums.reverse();
        return if nums.len() < 3 {nums[0]} else {nums[2]}
    }
}
