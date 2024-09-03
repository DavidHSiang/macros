use std::vec;

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    let my_vec = my_vec![
        "123132213213213123213",
        "12312312343345435345",
        "534453645645645654",
    ];
    println!("{:?}", my_vec);
    println!("Hello, world!");

    let v1 = <[_]>::into_vec(Box::new([1, 2, 34, 5, 6, 7, 8, 9, 0]));
    println!("{:?}", v1);
}

// my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        Vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),+]))
        }
    };
}
