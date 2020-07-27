impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut res: String = String::new();
        for c in address.chars() {
            let f = match c {
                '.' => {
                    res.push('[');
                    res.push('.');
                    res.push(']');
                },
                _ => res.push(c)
            };
        }
        res
    }
}
