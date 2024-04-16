fn main() {}

mod topic_4;
mod utils;
mod tests {

    #[test]
    fn sort_basic_test() {
        use crate::utils::data_helper::DataHelper;
        let mut vec1: Vec<i32> = vec![0, 1, 0, 12, -3];
        DataHelper::sort_basic(&mut vec1);
        assert_eq!(vec1, vec![-3, 0, 0, 1, 12]);

        let mut vec2: Vec<i32> = vec![9, 8, 7, 6, -5, 4, 3, 2, 1];
        DataHelper::sort_basic(&mut vec2);
        assert_eq!(vec2, vec![-5, 1, 2, 3, 4, 6, 7, 8, 9]);
    }

    #[test]
    fn sort_advanced_test() {
        use crate::utils::data_helper::DataHelper;
        let mut vec1: Vec<u32> = vec![0, 1, 0, 12, 3];
        DataHelper::sort_advanced(&mut vec1);
        assert_eq!(vec1, vec![0, 0, 1, 3, 12]);

        let mut vec2: Vec<i64> = vec![9, -8, 7, -6, 5, -4, 3, -2, 1];
        DataHelper::sort_advanced(&mut vec2);
        assert_eq!(vec2, vec![-8, -6, -4, -2, 1, 3, 5, 7, 9]);
    }

    #[test]
    fn traffic_light_test() {
        use crate::topic_4::traffic_light::{DurationTime, TrafficLight};

        let red_light = TrafficLight::Red;
        let yellow_light = TrafficLight::Yellow;
        let green_light = TrafficLight::Green;

        assert_eq!(red_light.get_duration(), 90);
        assert_eq!(green_light.get_duration(), 60);
        assert_eq!(yellow_light.get_duration(), 3);
    }
}
