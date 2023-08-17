use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
    is_true: Rc<RefCell<bool>>,
}

fn main() {
    let t = (12, "eggs"); // t is in the stack
    let b = Box::new(t); // b is in the stack, the data itself is in the heap
    println!("{:?}", b);

    let x = 5;
    let y = &x; // y is a reference
    assert_eq!(5, x);
    assert_eq!(5, *y); // desreferencing y

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    // reference counting of 3, all pointing to the same string
    println!("{}, {}, {}", s1, s2, s3);

    let flag = Flagger{is_true: Rc::new(RefCell::new(true))};

    let reference = Rc::new(flag.is_true.clone());
    println!("{:?}", reference);

    // change an inmutable value
    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;
    println!("{}", mut_ref);
}

