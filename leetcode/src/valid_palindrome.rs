
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let n = chars.len();
        for i in 0..(n/2) {
            if chars[i] != chars[n-i-1] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s = "A man, a plan, a canal: Panama";
        let r = Solution::is_palindrome(s.to_string());
        assert_eq!(true, r);

        let s = " ";
        let r = Solution::is_palindrome(s.to_string());
        assert_eq!(true, r);

        let s = "0P";
        let r = Solution::is_palindrome(s.to_string());
        assert_eq!(false, r)
    }
}