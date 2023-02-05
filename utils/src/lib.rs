mod error;

#[macro_export]
macro_rules! count_args {
    ($i:ident, $($other: ident),*) => {
        1 + count_args!($($other),*)
    };
    ($i:ident) => {
        1
    };
    () => {
        0
    };
}

#[macro_export]
macro_rules! flat_print {
    ($i:ident, $($other: ident),*) => {
        print!("{:?}={:?},", stringify!($i), $i);
        flat_print!($($other),*);
    };
    ($i:ident) => {
        print!("{:?}={:?},", stringify!($i), $i);
    };
    () => {
        println!();
    };
}