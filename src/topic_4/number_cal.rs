pub struct Cal {}

#[allow(dead_code)]
impl Cal {
    /*
    u32类型的整数集合求和
    */
    pub fn get_sum(vec: &[u32]) -> Option<u32> {
        let mut sum_val: u32 = 0;
        for e in vec.iter() {
            match sum_val.checked_add(*e) {
                Some(result) => sum_val = result,
                None => return None,
            }
        }
        Some(sum_val)
    }
}
