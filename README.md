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

作业五要求：  
1.实现存证模块的功能，包括：创建存证；撤销存证  
2.为存证模块添加新的功能，转移存证，接收两个参数，一个是包含的哈希值，另一个是存证的接收账户地址  

作业五解答：  
1.substrate-node-template/pallet/poe/src下的lib\mock\tests, 其中lib定义了存证模块的功能, mock用于定义模拟环境下运行时的配置，tests是存证的单元测试  
* create_claim()和revoke_claim()分别用于创建存证和撤销存证。分别对应的事件类型是ClaimCreated(),和ClaimRevoked()。错误类型有：创建错误ProofAlreadyExist（凭证已存在）， 撤销错误包括ClaimNotExist（凭证不存在）, NotClaimOwner（没有撤销权限）  
* 单元测试包括创建存证create_claim_test()，撤销存证revoke_claim_test()，测试结果截图：
  <img width="1440" alt="创建凭证单测" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/01263097-1198-4be7-b028-6231eeebca2f">
  <img width="1437" alt="删除凭证单测" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/074c76ed-dd57-427a-9c7a-35cfd32f7714">


* 创建存证的功能验证截图：
  <img width="1427" alt="创建凭证-1" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/39dfca32-2b73-4f67-af3e-8187b6bdd064">
  <img width="1438" alt="创建凭证-2" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/3ffae51c-282d-44fc-ae0b-a61061874473">
  <img width="1433" alt="创建凭证-3" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/1b1508bb-4715-40cb-a46a-b45a8f1239c6">

* 撤销存证的功能验证截图：
  <img width="1429" alt="删除凭证-1" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/d9abb3ae-6f1b-426e-aa4a-eca1ab65c211">
  <img width="1428" alt="删除凭证-2" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/90083ee0-c466-4bf2-a657-e792b17fd502">

2.substrate-node-template/pallet/poe/src下的lib\mock\tests, 其中lib定义了存证模块的功能, mock用于定义模拟环境下运行时的配置，tests是存证的单元测试  
* 转移存证transfer_claim()。对应的事件类型是ClaimTransferred()。错误类型有：NotNeedTransfer（重复转移）  
* 单元测试包括转移存证transfer_claim_test()，测试结果截图：
  <img width="1429" alt="转移凭证单测" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/bbc98541-2941-49ed-9a7f-df48bdeaad02">

* 转移存证的功能验证截图：
  <img width="1429" alt="转移凭证-1" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/46d51fb9-0a17-4688-a843-287ded0511db">
  <img width="1424" alt="转移凭证-2" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/610c56bb-6f73-4ad5-9797-44550cf2e31d">
  <img width="1441" alt="转移凭证-3" src="https://github.com/lihuineo/OneBlockSubstrateCourse/assets/161575076/058b4843-b5ca-4c02-b372-c9c81d63765c">

作业六要求：  
编写一个类型脚本程序来订阅template pallet中的值的更新（something）和event

作业六解答：  
client/query.ts, 其中subscribeSomething和subscribeSomethingStored定义订阅something和event的逻辑  
验证截图：  



