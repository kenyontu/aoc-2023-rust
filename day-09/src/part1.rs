pub fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut last_nums: Vec<i64> = Vec::new();
            let mut nums: Vec<i64> = line.split(' ').map(|s| s.parse().unwrap()).collect();

            last_nums.push(nums.last().unwrap().clone());

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

                last_nums.push(differences.last().unwrap().clone());
                nums = differences;
            }

            let sum = last_nums.iter().sum::<i64>();
            sum
        })
        .sum()
}
