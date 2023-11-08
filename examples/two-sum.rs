use std::collections::HashMap;

fn main() {
    println!("hello");
    println!("{:?}", two_sum([3,3, 7, 1].to_vec(), 8));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut head = 0;
    let mut tail= nums.len() -1;
    let mut result = Vec::new();
    let mut n = nums.clone();
    n.sort();

    while head < tail {
        if n[head] + n[tail] == target {
            // let mut result = Vec::new();
            for (i, num) in nums.iter().enumerate(){
                if n[head] == *num {
                    result.push(i as i32);
                    continue
                }
                if n[tail] == *num {
                    result.push(i as i32);
                    continue
                }
            };
            return result
            // return get_original_indexes(n[head], n[tail], nums)
        } else if n[head] + n[tail] > target {
            tail -= 1;
        } else {
            head += 1;
        }
    };
    return result
}