pub struct Solution;

impl Solution {
    // #1
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n;
        let mut digits = vec![];
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        let mut now = 1;
        digits
            .into_iter()
            .rev()
            .map(|m| {
                let ans = m * now;
                now *= -1;
                ans
            })
            .sum()
    }

    // #2
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score = score;
        score.sort_by(|a, b| b[k as usize].cmp(&a[k as usize]));
        score
    }

    // #3
    fn count_ones(s: String) -> usize {
        s.chars().filter(|c| *c == '1').count()
    }

    pub fn make_strings_equal(s: String, target: String) -> bool {
        let s = Solution::count_ones(s);
        let target = Solution::count_ones(target);
        if s == 0 {
            target == 0
        } else {
            target >= 1
        }
    }

    // #4
    pub fn min_cost(nums: Vec<i32>, k: i32) -> i32 {
        let l = nums.len();
        let mut costs = vec![vec![0; l]; l];

        for i in 0..l {
            let mut seen = vec![0; l];
            seen[nums[i] as usize] += 1;
            costs[i][i] = 0;
            for j in i + 1..l {
                let now = nums[j] as usize;
                seen[now] += 1;
                match seen[now] {
                    1 => costs[i][j] = costs[i][j - 1],
                    2 => costs[i][j] = costs[i][j - 1] + 2,
                    _ => costs[i][j] = costs[i][j - 1] + 1,
                }
            }
        }

        let mut dp = vec![0];

        for i in 1..=l {
            dp.push((0..i).map(|j| dp[j] + costs[j][i - 1]).min().unwrap() + k);
        }

        *dp.last().unwrap()
    }
}
