struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut result = 0_i32;
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        for k in 0..(n*2-1) {
            let mut i = k / 2;
            let mut j = k / 2 + k % 2;
            while chars[i] == chars[j] {
                result += 1;

                if i > 0 {
                    i -= 1;
                } else {
                    break;
                }

                if j < (n-1) {
                    j += 1;
                } else {
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let s = "aaa".to_string();
        let r = Solution::count_substrings(s);
        assert_eq!(6, r)
    }

}