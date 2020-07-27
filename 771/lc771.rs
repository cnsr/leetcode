impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut res: i32 = 0;
        for jew in j.chars() {
            for stone in s.chars() {
                if stone == jew {
                    res += 1;
                }
            } 
        }
        res
    }
}
