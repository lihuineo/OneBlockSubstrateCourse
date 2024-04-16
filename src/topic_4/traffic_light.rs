#[allow(dead_code)]

pub enum TrafficLight {
    //枚举交通信号灯
    Red,
    Green,
    Yellow,
}

const PER_SEC: u32 = 1;
pub trait DurationTime {
    fn get_duration(&self) -> u32; //获取交通灯时长
}

impl DurationTime for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => PER_SEC * 90,
            TrafficLight::Green => PER_SEC * 60,
            TrafficLight::Yellow => PER_SEC * 3,
        }
    }
}
