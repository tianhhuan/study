pub mod trai{

        pub trait Des{
              fn pri(&self) -> &String;
        }
        pub struct Person{
            pub name : String,
            pub age : u8
        }
        impl  Des for Person{
             fn pri(&self) -> &String {
                &self.name
            }
        }
    
}