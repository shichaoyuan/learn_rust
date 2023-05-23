struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<u8, i32> = HashMap::new();
        let mut l = 0;
        let mut r = 0;
        let mut longest = 0;

        let bytes = s.as_bytes();
        loop {
            if r >= bytes.len() {
                break;
            }
            let a = bytes[r];
            map.entry(a).and_modify(|c| *c += 1).or_insert(1);
            r += 1;

            while map.get(&a).map_or(false, |c| *c > 1) {
                let b = bytes[l];
                map.entry(b).and_modify(|c| *c -= 1);
                l += 1;
            }

            if r - l > longest {
                longest = r - l;
            }
        }

        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let r = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(3, r);
    }
}
