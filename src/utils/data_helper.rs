pub struct DataHelper {}

#[allow(dead_code)]
impl DataHelper {
    /*
    基础要求：固定类型（比如i32）的数组排序
    */
    pub fn sort_basic(nums: &mut Vec<i32>) {
        assert!(!nums.is_empty());
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] > nums[j] {
                    nums.swap(i, j);
                }
            }
        }
    }

    /*
    提高部分：能够使用范型和PartialOrd实现对任意类型的排序
     */
    pub fn sort_advanced<T: PartialOrd>(nums: &mut Vec<T>) {
        assert!(!nums.is_empty());
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] > nums[j] {
                    nums.swap(i, j);
                }
            }
        }
    }
}