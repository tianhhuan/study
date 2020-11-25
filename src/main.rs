fn main() {
    println!("Hello, world!");
    let a= 123;
    //声明
    println!("a:{}",a);
    // 重影
    let  a="aaa";
    println!("a 重影:{}",a);
    let mut  a= 111;
    a+=2;
    a=112;
    println!("a 可变变量:{}",a);


    // 常量

    const  lll:i128=1111;

    ///interpreted as a constant pattern, not a new variable
    //    |         help: introduce a variable instead: `lll_var`
    //let lll=a;

    println!("常量 lll ：{}",lll);


    ///函数
    GetAdd10(11);


    ///函数的语句和表达式

    let a ={
        let a1=3;
        a1+11
    };
    fn fn1()->i32{
        5
    }
    println!("表达式a:{}, 函数表达式{}",a,fn1());

    println!("有返回值方法：{}",GetReturnInt(11)) ;



    ///条件语句

    let index = 3;
    if index>3 {
        println!("idnex大于3")
    }else if index==3 {
        println!("index等于3")
    }else {
        println!("index小于3")
    }

    //循环
    // while true {
    //     println!("循环")
    // }

    let mut  leng=10;
    while leng>1 {
        println!("leng:{}",leng);
        leng-=1;
    }

    let array=[1,2,3,4,5];
    for item in array.iter() {
        println!("for in:{}",item)
    }
    for  item in 0..10 {
        println!("for in:{}",item)
    }

    let mut  leng1=10;
    loop {
        leng1-=1;
        println!(" loop leng1:{}",leng1);
        if leng1<0 {
            break;
        }
    }
}
///无返回值
fn GetAdd10(x:i32){
    println!("x {}",x+10)
}
///有返回值
fn GetReturnInt(x:i32)-> i32{
    return  x+1;
}
