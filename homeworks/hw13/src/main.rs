use std::rc::Rc;
fn main() {
    let x = 5;
    let xh = Box::new(x);
    println!("{}", x * *xh);

    let rc_value = String::from("RC Value");
    {
        let rc = Rc::new(rc_value); // rc_value is moved into rc, rc_value can't be used anymore (should have used clone)
        println!("{}", Rc::strong_count(&rc)); // 1
        {
            let rc2 = Rc::clone(&rc); // another reference, both counts are 2 now
            println!("{}", Rc::strong_count(&rc)); // 2
            println!("{}", Rc::strong_count(&rc2)); // 2
        }// rc2 is dropped, the count is 1
        println!("{}", Rc::strong_count(&rc)); // 1
    } // rc is dropped
    // println!("{}", rc_value); // this doesn't work because rc_value was moved into rc
}
