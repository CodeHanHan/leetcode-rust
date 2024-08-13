use std::collections::HashMap;

// 两数之和: https://leetcode.cn/problems/two-sum/description/
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut want = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        match want.get(num) {
            Some(&i) => return vec![i as i32, index as i32],
            None => {
                want.insert(target - num, index);
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod test_two_sum {
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        want: Vec<i32>,
    }

    #[test]
    fn test_two_sum() {
        let test_cases = [
            TestCase {
                nums: vec![2, 7, 11, 15],
                target: 9,
                want: vec![0, 1],
            },
            TestCase {
                nums: vec![3, 2, 4],
                target: 6,
                want: vec![1, 2],
            },
            TestCase {
                nums: vec![3, 3],
                target: 6,
                want: vec![0, 1],
            },
        ];
        for tc in test_cases.iter() {
            assert_eq!(super::two_sum(tc.nums.clone(), tc.target), tc.want);
        }
    }
}
