use core::num;


trait PrintLol {
    fn print_lol(&self) {
        println!("LOL!");
    }
}

trait PrintHello {
    fn print_hello(&self) {
        println!("Hello!");
    }
}

struct Mgd {
  data: String,
}

struct Dnn {
  num: i32,
}

impl PrintLol for Mgd {}
impl PrintLol for Dnn {}
impl PrintHello for Mgd {}
impl PrintHello for Dnn {}

fn main(){
  // let data = Mgd {
  //   data: "Hello".to_string(),
  // };

  // let num = Dnn {
  //   num: 42,
  // };

  // // data.print_lol();
  // get_object(data);
  // get_object(num);

  let data = return_object();
  data.print_lol();
}

// fn get_object(obj: impl PrintLol) {
//     obj.print_lol();
// }
// fn get_object<T: PrintLol + PrintHello>(obj: T) {
//     obj.print_lol();
//     obj.print_hello();
// }
// fn get_object<T>(obj: T) 
// where T: PrintLol + PrintHello,
// {
//     obj.print_lol();
//     obj.print_hello();
// }

fn return_object() -> impl PrintLol {
    Mgd {
        data: "Hello from Mgd".to_string(),
    }
}