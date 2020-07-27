impl Solution {
    pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for x in 0..a.len() {
            a[x].reverse();
            for y in 0..a[x].len() {
                let f = match a[x][y] {
                    1 => a[x][y] = 0,
                    0 => a[x][y] = 1,
                    _ => ()
                };
            }
        }
        a
    }
}
