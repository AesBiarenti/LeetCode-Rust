pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write_index = 0;
    let len = nums.len();

    for i in 0..len {
        if nums[i] != val {
            nums[write_index] = nums[i];
            write_index += 1;
        }
    }

    for i in write_index..len {
        nums[i] = -1;  
    }

    write_index as i32
}