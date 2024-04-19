作业三要求：  
使用Rust语言写一个冒泡排序的算法  
基础要求：固定类型（比如i32）的数组排序  
提高部分：能够使用范型和PartialOrd实现对任意类型的排序  

作业三解答：  
基础要求：data_helper::sort_basic()  
提高部分：data_helper::sort_advanced()  
单元测试部分：tests::sort_basic_test(), tests::sort_advanced_test()  
运行截图：  
<img width="1430" alt="WeChat4d5237ebfb2016b91c47fc2db1628382" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/631d826e-a9a0-497a-b469-369ecbbc2ac8">

作业四要求：  
1.为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
2.实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
3.实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

作业四解答：  
1.topic_4/traffic_light, 分别定义枚举TrafficLight和特性DurationTime。DurationTime的get_duration获取时间。单元测试traffic_light_test()  
<img width="1418" alt="交通灯测试" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/6c892582-03f3-4a37-b2fe-aadc358487fb">

2.topic_4/number_cal, get_sum()用于计算u32整数求和。单元测试get_sum_test()  
<img width="1416" alt="求和测试" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/19b02fb3-b391-4c4f-9fba-1ff20ffe3532">

3.topic_4/shape_cal, get_area()用于计算面积，接收geo参数，该参数服从Geometry泛型约束。Geometry定义了可以计算面积的图形，它需要实现cal_area方法。分别定义了Rectangle、Squard、Circle服从Geometry特性，实现了cal_area()方法。单元测试get_area_test()  
<img width="1438" alt="求面积测试" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/487b19d0-ec4d-4a1f-91d4-2ea3f2081676">
