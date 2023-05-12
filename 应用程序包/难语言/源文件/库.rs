macro_rules! 批量偷窃别人的模块并声明为自己的模块 {
    ($(
        $模块:tt :: { $($原名:tt $(::$原名加:tt)* $新名:ident),+ }
    )+) => {
        $(
            pub use $模块::{
                $($原名 $(::$原名加)* as $新名,)+
            };
        )+
    };
}

批量偷窃别人的模块并声明为自己的模块! {
    核心库::{
        assert 断言,
        assert_eq 断言相等,
        assert_ne 断言不等,
        debug_assert 调试断言,
        debug_assert_eq 调试断言相等,
        debug_assert_ne 调试断言不等,
        matches 匹配,
        panic 异常终止,
        panic 恐慌,
        todo 待办,
        unimplemented 无实现,
        unreachable 不可到达
    }

    标准库::{
        alloc 分配模块,
        dbg 调试,
        eprint 错误打印,
        eprintln 错误换行打印,
        format 格式化,
        fs 文件系统模块,
        mem 内存模块,
        path 路径模块,
        pin 内存固定模块,
        print 打印,
        println 换行打印,
        process 进程模块,
        result 结果模块模块,
        slice 切片模块模块,
        string 字符串模块模块,
        sync 同步模块,
        thread 线程模块,
        time 时间模块,
        vec 向量模块,
        write 写,
        writeln 换行写,
        os 系统模块,
        net 网络模块,
        option 可选模块,
        marker 标记模块,
        io 出入模块,
        ops 运算符模块,
        future 期货模块,
        fmt 格式模块,
        default 默认模块,
        convert 转换模块,
        collections 集合模块,
        cmp 比较模块,
        cell 容器模块,
        ffi	语言交互模块,
        hash 哈希模块,
        any 任意模块,
        borrow 借用,
        error 错误模块,
        clone 克隆模块,
        iter 迭代器模块
    }

    标准库::{
        any::Any 任意特性,
        borrow::Borrow 借用特性,
        borrow::BorrowMut 可变借用特性,
        borrow::ToOwned 变为己有特性,
        clone::Clone 克隆特性,
        cmp::Eq 相等特性,
        cmp::Ord 顺序特性,
        cmp::PartialEq 部分相等特性,
        cmp::PartialOrd 部分顺序特性,
        convert::AsMut 作为可变特性,
        convert::AsRef 作为引用特性,
        convert::From 从变换特性,
        convert::Into 变换到特性,
        convert::TryFrom 尝试从变换特性,
        convert::TryInto 尝试变换到特性,
        default::Default 默认特性,
        error::Error 错误特性,
        fmt::Binary 二进制特性,
        fmt::Debug 调试特性,
        fmt::Display 显示特性,
        fmt::Write 格式写特性,
        future::Future 期货特性,
        future::IntoFuture 转成期货特性,
        hash::Hash 哈希特性,
        hash::Hasher 哈希器特性,
        io::BufRead 缓冲区读取特性,
        io::IsTerminal 是否终止特性,
        io::Read 读取特性,
        io::Seek 可寻特性,
        io::Write 写特性,
        iter::Extend 延伸特性,
        iter::FromIterator 从迭代器特性,
        iter::IntoIterator 转成迭代器特性,
        iter::Iterator 迭代器特性,
        marker::Send 线程发送特性,
        marker::Sized 大小可知特性,
        marker::Sync 线程同步特性,
        ops::Deref 解引用特性,
        ops::DerefMut 可变解引用特性,
        ops::Fn 闭包特性,
        ops::FnMut 可变闭包特性,
        ops::FnOnce 一次性闭包特性,
        string::ToString 转字符串特性
    }
}

pub use core as 核心库;
pub use nan_lang_macros::*;
pub use std as 标准库;
pub use 标准库 as 爱死提地;
pub use 核心库 as 核;

pub type 无符号八位整数 = u8;
pub type 无符号十六位整数 = u16;
pub type 无符号三十二位整数 = u32;
pub type 无符号六十四位整数 = u64;
pub type 无符号一百二十八位整数 = u128;
pub type 无符号操作系统位整数 = usize;

pub type 有符号八位整数 = i8;
pub type 有符号十六位整数 = i16;
pub type 有符号三十二位整数 = i32;
pub type 有符号六十四位整数 = i64;
pub type 有符号一百二十八位整数 = i128;
pub type 有符号操作系统位整数 = isize;

pub type 三十二位浮点数 = f32;
pub type 六十四位浮点数 = f64;

pub type 无8 = u8;
pub type 无16 = u16;
pub type 无32 = u32;
pub type 无64 = u64;
pub type 无128 = u128;
pub type 无系统位 = usize;

pub type 有8 = i8;
pub type 有16 = i16;
pub type 有32 = i32;
pub type 有64 = i64;
pub type 有128 = i128;
pub type 有系统位 = isize;

pub type 浮32 = f32;
pub type 浮64 = f64;

pub type 向量<类型> = Vec<类型>;
pub type 动态数组<类型> = 向量<类型>;

pub type 字符 = char;
pub type 字符串 = String;
pub type 字符串切片 = str;

pub type 可选值<类型> = Option<类型>;
pub type 可选<类型> = 可选值<类型>;
pub type 不一定有<类型> = 可选值<类型>;
pub type 可能有<类型> = 可选值<类型>;
pub type 说不定有<类型> = 可选值<类型>;

pub type 结果<对, 错> = Result<对, 错>;
pub type 是非<是, 非> = 结果<是, 非>;
pub type 是非对错<是, 非> = 结果<是, 非>;

pub type 布尔值 = bool;
pub type 布尔 = 布尔值;
pub type 布尔林 = 布尔值;
pub type 真假 = 布尔值;
pub type 真假与否 = 布尔值;
pub type 二值 = 布尔值;
pub type 要么是要么不是 = 布尔值;
pub type 要么不是要么是 = 布尔值;

pub type 空值 = ();
pub type 虚无 = 空值;
pub type 没有 = 空值;
pub type 没 = 空值;
pub type 空 = 空值;
pub type 无 = 空值;
pub type 虚空 = 空值;
pub type 空元组 = 空值;

pub type 盒子指针<类型> = Box<类型>;
pub type 盒子<类型> = 盒子指针<类型>;
pub type 箱子<类型> = 盒子指针<类型>;
pub type 箱子指针<类型> = 盒子指针<类型>;
pub type 指针<类型> = 盒子指针<类型>;

pub type 线程安全指针<类型> = 同步模块::Arc<类型>;
pub type 多线程引用计数<类型> = 线程安全指针<类型>;
pub type 括弧<类型> = 线程安全指针<类型>;
pub type 原子引用计数指针<类型> = 线程安全指针<类型>;
pub type 原子引用计数<类型> = 线程安全指针<类型>;
pub type 原子计数指针<类型> = 线程安全指针<类型>;
pub type 跨线程<类型> = 线程安全指针<类型>;
pub type 跨线程指针<类型> = 线程安全指针<类型>;

pub type 线程引用计数<类型> = 标准库::rc::Rc<类型>;
pub type 单线程引用计数<类型> = 线程引用计数<类型>;
pub type 引用计数<类型> = 线程引用计数<类型>;
pub type 引用计数指针<类型> = 线程引用计数<类型>;
pub type 单线程引用计数指针<类型> = 线程引用计数<类型>;
pub type 智能指针<类型> = 线程引用计数<类型>;

pub type 互斥锁<类型> = 同步模块::Mutex<类型>;
pub type 锁<类型> = 互斥锁<类型>;
pub type 互斥<类型> = 互斥锁<类型>;

pub type 读写锁<类型> = 同步模块::RwLock<类型>;
pub type 读写<类型> = 读写锁<类型>;

pub type 数组<类型, const 大小: 无符号操作系统位整数> = [类型; 大小];
pub type 固定大小数组<类型, const 大小: 无符号操作系统位整数> = [类型; 大小];

pub type 切片<类型> = [类型];

pub type 裸指针<类型> = *const 类型;
pub type 可变裸指针<类型> = *mut 类型;

pub type 引用<'生命周期, 类型> = &'生命周期 类型;
pub type 可变引用<'生命周期, 类型> = &'生命周期 mut 类型;

#[macro_export]
macro_rules! 很难很难的语言 {
    () => {
        #[macro_use]
        extern crate nan_lang;
        use nan_lang::*;
    };
}

#[macro_export]
macro_rules! 主函数 {
    ($代码块:block) => {
        fn main() $代码块
    };
    () => {
        $crate::主函数!({});
    };
}

#[macro_export]
macro_rules! 模块 {
    (
        公开;
        $(
            $模块名:ident { $($模块体:item)* }
        )+
    ) => {
        pub $crate::模块!(
            $(
                $模块名 {
                    $($模块体)*
                }
            )+
        );
    };
    ($(
        $模块名:ident { $($模块体:item)* }
    )+) => {
        $(
            mod $模块名 {
                use nan_lang::*;
                $($模块体)*
            }
        )+
    };
}

#[macro_export]
macro_rules! 结构体 {
    (
        公开
        $(
            $(#[$元属性:meta])*
            $结构体名:ident$(< $( $生命周期或类型:tt $( : $生命周期或类型1:tt $(+ $生命周期或类型2:tt )* )? ),+ >)?
            $(泛型约束$(:)? < $( $泛型:tt : $类型:tt $(+ $额外:tt )* ),+ >;)?
            {
                $(
                    $(#[$字段元属性:meta])*
                    $字段名:ident: $字段类型:ty $(,)?
                )*
            }
        )+
    ) => {
        $crate::结构体! {
            $(
                $(#[$元属性])*
                pub $结构体名 $(< $( $生命周期或类型 $( : $生命周期或类型1 $(+ $生命周期或类型2 )* )? ),+ >)?
                $(where $($泛型: $类型 $(+ $额外 )*,)+)?
                {
                    $(
                        $(#[$字段元属性])*
                        $字段名: $字段类型,
                    )*
                }
            )+
        }
    };
    (
        $(
            $(#[$元属性:meta])*
            $可见性:vis $结构体名:ident$(< $( $生命周期或类型:tt $( : $生命周期或类型1:tt $(+ $生命周期或类型2:tt )* )? ),+ >)?
            $(泛型约束$(:)? < $( $泛型:tt : $类型:tt $(+ $额外:tt )* ),+ >;)?
            {
                $(
                    $(#[$字段元属性:meta])*
                    $字段名:ident: $字段类型:ty $(,)?
                )*
            }
        )+
    ) => {
        $(
            $(#[$元属性])*
            $可见性 struct $结构体名 $(< $( $生命周期或类型 $( : $生命周期或类型1 $(+ $生命周期或类型2 )* )? ),+ >)?
            $(where $($泛型: $类型 $(+ $额外 )*,)+)?
            {
                $(
                    $(#[$字段元属性])*
                    $字段名: $字段类型,
                )*
            }
        )+
    };
    (
        公开
        $(
            $(#[$元属性:meta])*
            $结构体名:ident;
        )*
    ) => {
        $crate::结构体! { $( $(#[$元属性])* pub $结构体名; )* }
    };
    (
        $(
            $(#[$元属性:meta])*
            $可见性:vis $结构体名:ident;
        )*
    ) => {
        $( $(#[$元属性])* $可见性 struct $结构体名; )*
    };
}

#[macro_export]
macro_rules! 函数 {
    (
        公开
        $(
            $(#[$元属性:meta])*
            $函数名:ident$(< $( $生命周期或类型:tt $( : $生命周期或类型1:tt $(+ $生命周期或类型2:tt )* )? ),+ >)?  $(参数$(:)?)? $(($($参数:ident : $参数类型:ty),*))?;
            $(返回值类型$(:)? $返回值类型:ty;)?
            $(泛型约束$(:)? < $( $泛型:tt : $类型:tt $(+ $额外:tt )* ),+ >;)?
            $(函数体)?$(:)? $函数体:block;
        )+
    ) => {
        $crate::函数!(
            $(
                $(#[$元属性])*
                pub $函数名$(< $( $生命周期或类型 $( : $生命周期或类型1 $(+ $生命周期或类型2 )* )? ),+ >)? $(($($参数 : $参数类型)*))?;
                $(返回值类型: $返回值类型;)?
                $(泛型约束: < $( $泛型: $类型 $(+ $额外 )* ),+ >;)?
                函数体: $函数体;
            )+
        );
    };
    (
        $(
            $(#[$元属性:meta])*
            $可见性:vis $函数名:ident$(< $( $生命周期或类型:tt $( : $生命周期或类型1:tt $(+ $生命周期或类型2:tt )* )? ),+ >)? $(参数$(:)?)? $(($($参数:ident : $参数类型:ty)*))?;
            $(返回值类型$(:)? $返回值类型:ty;)?
            $(泛型约束$(:)? < $( $泛型:tt : $类型:tt $(+ $额外:tt )* ),+ >;)?
            $(函数体)?$(:)? $函数体:block;
        )+
    ) => {
        $(
            $可见性 fn $函数名 $(< $( $生命周期或类型 $( : $生命周期或类型1 $(+ $生命周期或类型2 )* )? ),+ >)?
            ($($($参数 : $参数类型,)*)?) $(-> $返回值类型)? $(where $($泛型: $类型 $(+ $额外 )*,)+)? $函数体
        )+
    };
}

#[macro_export]
macro_rules! 引用 {
    ($模块路径:pat) => {
        use $模块路径;
    };
    (本仓库 $($模块路径:tt $(:: $子路径:tt)*)+) => {
        use crate::{
            $($模块路径 $(:: $子路径)* )+
        };
    };
}

#[macro_export]
macro_rules! 公开引用 {
    ($模块路径:pat) => {
        pub use $模块路径;
    };
    (本仓库 $($模块路径:tt $(:: $子路径:tt)*)+) => {
        pub use crate::{
            $($模块路径 $(:: $子路径)* )+
        };
    };
}

#[macro_export]
macro_rules! 变量 {
    (
        可变
        $(
            $变量名:ident $(: $类型:ty)? = $表达式:expr$(;)?
        )+
    ) => {
        $(let mut $变量名 $(: $类型)? = $表达式;)+
    };
    (
        $( $变量名:ident $(: $类型:ty)? = $表达式:expr$(;)? )+
    ) => {
        $(let $变量名 $(: $类型)? = $表达式;)+
    };
}

#[macro_export]
macro_rules! 可变量 {
    (
        $( $变量名:ident $(: $类型:ty)? = $表达式:expr$(;)? )+
    ) => {
        $(let mut $变量名 $(: $类型)? = $表达式;)+
    };
}

#[macro_export]
macro_rules! 实现 {
    (
        $($类型:tt)+
    ) => {
        $(impl $类型)+
    };
}

#[macro_export]
macro_rules! 强制转换 {
    ($变量名:expr $(=> $类型:ty)+) => {
        $变量名 $(as $类型)+
    };
}

#[macro_export]
macro_rules! 强转 {
    ($变量名:expr $(=> $类型:ty)+) => {
        $变量名 $(as $类型)+
    };
}

#[macro_export]
macro_rules! 向量 {
    ($参数:tt) => {
        vec![$参数];
    };
}

#[macro_export]
macro_rules! 动态数组 {
    ($参数:tt) => {
        vec![$参数];
    };
}

#[macro_export]
macro_rules! 环境变量 {
    ($参数:tt) => {
        env!($参数);
    };
}

#[macro_export]
macro_rules! 可选环境变量 {
    ($参数:tt) => {
        option_env!($参数);
    };
}
