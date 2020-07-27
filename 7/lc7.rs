impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut neg: bool = false;
        let mut y = x.clone();
        if x < 0 {
            neg = true;
            y *= -1;
        }
        let z: String = y.to_string().chars().rev().collect::<String>();
        match z.parse() {
            Ok(val) => {y = val},
            Err(err) => {return 0;}
        }
        if y > i32::max_value() {
            return 0;
        }
        if neg {
            y *= -1;
        }
        y
    }
}
