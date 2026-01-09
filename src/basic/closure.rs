use std::thread;
pub fn test() {
    let num = 5;
    // decalre a closure, it is a function without a name
    let add_one = |x| x + 1;
    println!("{} add 1 equals: {}", num, add_one(num));

    // declare a closure with multiple parameters
    let calculate = |a, b, c| a * b + c;
    println!("1 + 2 + 3 equals: {}", calculate(1, 2, 3));

    // move the num variable into the closure, so that it can't be used after the current scope
    let take_num = move || println!("take num: {}", num);

    let double = |x| x * 2;
    let result = apply_to_value(5, double);
    println!("5 * 2 = {}", result);

    let add_five = make_adder(5);
    println!("4 + 5 = {}", add_five(4));

    let square = make_square();
    println!("4 squared = {}", square(4));

    // use closure to map a vector, and collect the result into a new vector
    let vec = vec![1, 2, 3, 4, 5];
    let squared_vec: Vec<i32> = vec.iter().map(|x| x * x).collect();
    println!("original vec: {:?}", vec);
    println!("squared vec: {:?}", squared_vec);

    let nums = vec![1, 2, 3, 4, 5];
    let handles = nums
        .into_iter()
        .map(|num| {
            thread::spawn(move || {
                thread::sleep(std::time::Duration::from_secs(3));
                num * 2
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        let result = handle.join().unwrap();
        println!("result is {}", result);
    }
}

// create a function that takes a closure as a parameter
fn apply_to_value<F>(val: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(val)
}

// create a function that returns a closure
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
// create a function that returns a closure, use Box to avoid the size limit
fn make_square() -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| y * y)
}
