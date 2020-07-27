impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let arrlen: usize = arr.len();
        if arrlen < 2 {return false;}
        let _diff: i32 = (arr[arrlen-1] - arr[arrlen-2]) * -1;
        for (pos, _elem) in arr.iter().enumerate() {
            if (pos as usize) < arrlen-1 {
                if *_elem - arr[pos+1] != _diff {
                    return false;
                }
            }
        }
        true
    }
}
