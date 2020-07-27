impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for element in 0..nums.len() {
            for e in 0..nums.len() {
                if e != element {
                    if nums[e] + nums[element] == target {
                        return vec!(element as i32,  e as i32);
                    }
                }
            }
        }
        vec!(0, 0)
    }

}
