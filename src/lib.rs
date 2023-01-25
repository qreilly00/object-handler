mod tests {
    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}

pub mod objectHandler {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct ObjectHandler {
        object_options: Vec<(Box<dyn Fn(i32) -> i32>, String)>,
        object_store: Vec<Vec<Box<dyn Draw>>>,
    }

    /*impl ObjectHandler{
        pub fn new() -> Self {
            Self{}
        }
    }*/
}
