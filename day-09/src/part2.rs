pub fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut first_nums: Vec<i64> = Vec::new();
            let mut nums: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();

            first_nums.push(nums.first().unwrap().clone());

            while nums.iter().any(|n| *n != 0) {
                let mut num = nums.first().unwrap();

                let differences: Vec<i64> = nums[1..]
                    .iter()
                    .map(|n| {
                        let diff = n - num;
                        num = n;
                        diff
                    })
                    .collect();

                first_nums.push(differences.first().unwrap().clone());
                nums = differences;
            }

            first_nums
                .iter()
                .rev()
                .copied()
                .reduce(|acc, n| n - acc)
                .unwrap()
        })
        .sum()
}
