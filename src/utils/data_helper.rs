pub struct DataHelper {}
use std::cmp;

#[allow(dead_code)]
impl DataHelper {
    /*
    固定i32类型的数组冒泡排序
    */
    pub fn sort_basic(nums: &mut Vec<i32>) {
        assert!(!nums.is_empty());
        for i in 0..nums.len() - 1 {
            let mut flag = false;
            for j in 0..nums.len() - i - 1 {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    flag = true;
                }
            }
            if !flag {
                break;
            }
        }
    }

    /*
    实现对任意类型的冒泡排序
     */
    pub fn sort_advanced<T: Copy + cmp::PartialOrd>(nums: &mut Vec<T>) {
        assert!(!nums.is_empty());
        for i in 0..nums.len() - 1 {
            let mut flag = false;
            for j in i..nums.len() - i - 1 {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    flag = true;
                }
            }
            if !flag {
                break;
            }
        }
    }
}
