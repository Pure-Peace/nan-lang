nan_lang::很难很难的语言! {}

结构体! {
    公开
    测试结构体;
    测试结构体1;
}

结构体! {
    公开
    测试结构体2 {
        东西: i32,
    }
}

模块! {
    演示模块 {
        引用! {
            本仓库 测试结构体2
        }
    }
}

函数! {
    公开 你好世界<'a, 'b: 'static, T, D: Sized + 'a>
    参数 (传入参数1: 测试结构体2);
    返回值类型 有符号三十二位整数;
    泛型约束 <T: Sized>;
    函数体 {
        变量! {
            示例不可变变量 = 1
            示例不可变变量2 = 2
        }
        变量! { 可变 示例可变变量: 无符号六十四位整数 = 1 }
        可变量! { 示例可变变量2 = 2 }

        示例可变变量 += 强制转换! { 传入参数1.东西 => 无64 } + 示例不可变变量 + 示例不可变变量2 + 示例可变变量2;

        换行打印!("运算结果：{示例可变变量}");

        换行打印!("你好，世界！");

        变量! { _锁 = 原子引用计数::new(互斥锁::new(1)) }

        1 + 强制转换! { 示例不可变变量 => 有符号三十二位整数 => 有符号六十四位整数 => 有符号三十二位整数 }
    };
}

主函数!({
    换行打印!(
        "返回值：{}",
        你好世界::<i32, i32>(测试结构体2 { 东西: 222 })
    )
});
