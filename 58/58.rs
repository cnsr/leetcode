impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut x = s.split_whitespace().collect::<Vec<_>>();
        if x.len() > 0 {
            x[x.len() - 1].len() as i32
        } else{
            x.len() as i32
        }
    }
}
