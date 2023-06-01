
struct Solution;

impl Solution {

    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        if nums1.is_empty() || nums2.is_empty() {
            return Vec::new();
        }

        let mut map: HashMap<i32, i32>= HashMap::new();
        let mut stack: Vec<i32> = Vec::new();
        for i in (0..nums2.len()).rev() {
            while let Some(&a) = stack.last() {
                if a < nums2[i] {
                    stack.pop();
                } else {
                    break;
                }
            }

            let v = match stack.last() {
                Some(&a) => a,
                None => -1
            };
            map.insert(nums2[i], v);

            stack.push(nums2[i]);
        }


        let mut result = Vec::with_capacity(nums1.len());
        for i in nums1 {
            result.push(*map.get(&i).expect("uknown value"));
        }

        result

    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {

        let nums1 = [4,1,2];
        let nums2 = [1,3,4,2];

        let result = Solution::next_greater_element(nums1.to_vec(), nums2.to_vec());

        assert_eq!([-1, 3, -1].to_vec(), result);
    }

}