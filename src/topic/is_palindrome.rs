// 回文数：https://leetcode.cn/problems/palindrome-number/description/

pub fn is_palindrome(x: i32) -> bool {
    let num_str = x.to_string();

    let n = num_str.len();

    if n <= 1 {
        return true;
    }

    if n == 2 {
        return num_str.chars().nth(0).unwrap() == num_str.chars().nth(1).unwrap();
    }

    for (i, c) in num_str.chars().enumerate() {
        if i >= n / 2 {
            break;
        }
        if c != num_str.chars().nth(n - i - 1).unwrap() {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod is_palindrome_test {
    struct testcases {
        x: i32,
        want: bool,
    }

    #[test]
    fn test_is_palindrome() {
        let test_cases = [
            testcases { x: 121, want: true },
            testcases {
                x: -121,
                want: false,
            },
            testcases { x: 10, want: false },
            testcases {
                x: -101,
                want: false,
            },
            testcases { x: 0, want: true },
            testcases { x: 101, want: true },
            testcases { x: 10, want: false },
        ];

        for tc in test_cases.iter() {
            assert_eq!(super::is_palindrome(tc.x), tc.want);
        }
    }
}
