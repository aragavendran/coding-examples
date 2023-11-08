fn main() {
    println!("hello");
    let mut nums = [0,0,1,1,1,2,2,3,3,4].to_vec();
    println!("{:?}", remove_duplicates(&mut nums));
}
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut counter = 0;

    for i in 1..nums.len() {
            if nums[i- counter] == nums[i - counter -1] {
                nums.remove(i - counter);
                counter += 1;
            };
    }
    return nums.len() as i32
}