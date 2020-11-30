


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
    ///所有权
    {
        let ownership=11;
        println!("ownership:{}",ownership);
        let ownership_string=String::from("hello world!");
        let ownership_string1=ownership_string;
        //println!("ownership_string:{}",ownership_string) //value borrowed here after move
        println!("ownership_string1:{}",ownership_string1);

        let ownership_string2= ownership_string1.clone();
        println!("ownership_string1:{}",ownership_string1);
        println!("ownership_string2:{}",ownership_string2);

        ///引用与租借

        let lease = String::from(" lease ");
        let lease1 = &lease;
        println!("lease is {} , lease1 is {}",lease,lease1);

        ///可变租借

        let mut  lease2= String::from("可变租借");
        let lease3 = &mut lease2;
        lease3.push_str("aaa");
        println!("可变租借：{}",lease3);




        //// Rust Slice切片
        let Slice = String::from("123456");

        let Slice1 = &Slice[0..5];
        let Slice=String::from("aaaaa");
        println!("切片：{}",Slice1);
        // for i in Slice1.iter() {
        //     println!("for i:{}",i)
        // }



        /// 结构体

        let site= Site{
            id: String::from("11111"),
            Name : String::from("aaaaa")
        };
        println!("结构体 site:{}",site.Name);

        site.priId();

        #[derive(Debug)]
        // ////枚举类
        // enum book{
        //     Open(),
        //     Close(String)
        // }
        // let  enum1= book::Open(String::from("open"));
        enum book{
            Open{name:String},
            Close{name:String}
        }
        let  enum1= book::Open{name:String::from("设计")};
        match enum1 {
            book::Open {name }=>{
                println!("{}",name)
            },
            book::Close {name}=>{
                println!("{}",name)
            }
        }
        //println!("enum1 :{:?}",enum1);
        enum Option<T> {
            Some(T),
            None,
        }
        let opt:Option<&str>=Option::None;
        match opt {
            Option::Some(somehing)=>{
                println!("{}",somehing)
            }
            Option::None=>{
                println!("opt os nothing")
            }
        }

        let match1=1;
        match match1 {
            10=>println!("10"),
            _=>println!("_")
        }
        if let 1 = match1{
            println!("1")
        }
    }
}
struct Site{
    id : String,
    Name : String,
}
impl Site{
    fn priId(&self){
        println!("this.Id:{}",self.id)
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
