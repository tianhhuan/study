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


}
