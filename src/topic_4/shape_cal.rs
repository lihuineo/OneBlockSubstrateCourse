use std::f32::consts::PI;

pub trait Geometry {
    fn cal_area(&self) -> f32; //定义可以计算面积的图形
}

pub struct ShapeCalculator {}

#[allow(dead_code)]
impl ShapeCalculator {
    /*
    计算面积
    */
    pub fn get_area<T: Geometry>(geo: T) -> f32 {
        geo.cal_area()
    }
}

//定义矩形
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl Geometry for Rectangle {
    fn cal_area(&self) -> f32 {
        &self.width * &self.height
    }
}

//定义正方形
pub struct Squard {
    pub width: f32,
    pub height: f32,
}

impl Geometry for Squard {
    fn cal_area(&self) -> f32 {
        assert_eq!(&self.width, &self.height);
        &self.width * &self.height
    }
}

//定义圆形
pub struct Circle {
    pub rad: f32,
}

impl Geometry for Circle {
    fn cal_area(&self) -> f32 {
        PI * self.rad * self.rad
    }
}
