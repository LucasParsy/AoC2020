pub static IMPLEMENTED: bool = true;
pub static INTERACTIVE: (bool, bool) = (false, false);

pub fn p1(input: &[String], _interactive: bool) -> i64 {
    let mut nums: Vec<u64> = input.iter().map(|f| f.parse::<u64>().unwrap()).collect();
    nums.sort_unstable();
    let last_elem = *nums.last().unwrap();
    nums.push(last_elem + 3);
    let mut joltage = 0;
    let mut adaptators = vec![0; 3];
    for elem in nums.iter() {
        let n_jolt = elem - joltage;
        adaptators[(n_jolt - 1) as usize] += 1;
        joltage += n_jolt;
    }

    //println!("{:#?}", adaptators);
    adaptators[0] * adaptators[2]
}

pub fn p2(input: &[String], _interactive: bool) -> i64 {
    let mut nums: Vec<u64> = input.iter().map(|f| f.parse::<u64>().unwrap()).collect();
    nums.sort_unstable();
    let last_elem = *nums.last().unwrap();
    nums.push(last_elem + 3);
    nums.insert(0, 0);
    //let mut mult_vec = vec![1;nums.len()];
    let magic_quotients = vec![1,1,1,2,4,7,1,1];

    //println!("{:#?}", nums);
    let mut combs = 1;
    let mut prev = nums[0] + 10;
    let mut streak = 1;
    for n in nums {
        if n != prev+1 {
            //println!("{}", streak);
            combs *= magic_quotients[streak];
            streak = 0;
        }
        streak += 1;
        prev = n;
    }
    combs
}

use crate::myTest;
myTest!();
