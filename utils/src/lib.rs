use std::{io::{Read, Cursor, Bytes}, iter::Peekable};

mod error;
mod type_erase;

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

pub fn str_to_readable(s: &str) -> Peekable<Bytes<Cursor<&str>>> {
    Cursor::new(s).bytes().peekable()
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter, Error};
    use std::mem::{discriminant, transmute};

    const SHORT_STR_MAX: usize = 14;

    enum Value {
        Integer(i64),
        Float(f64),
        ShortStr(u8, [u8;SHORT_STR_MAX])
    }

    impl Display for Value {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self {
                Value::Integer(v) => {
                    write!(f, "Integer({:})", v)
                },
                Value::Float(v) => {
                    write!(f, "Float({:})", v)
                },
                Value::ShortStr(v, data) => {
                    write!(f, "ShortStr({:},{:?})", v, data)
                },
            }
        }
    }

    enum ExpDesc {
        Integer(fn (i64) -> Value),
        Float(fn (f64) -> Value)
    }

    #[test]
    fn test_discriminant() {
        println!("{:?}", discriminant(&Value::Integer(1)));
        println!("{:?}", discriminant(&Value::Float(1.0)));
        println!("{:?}", discriminant(&Value::ShortStr(1, [0;SHORT_STR_MAX])));
    }

    #[test]
    fn test_transmute() {
        let a: i64 = if let Value::Float(v) = Value::Float(1.234) {
            unsafe { transmute(v) }
        } else {
            2
        };
        println!("{:?}", a);
    }

    #[test]
    fn test_enum_fn() {
        let int_exp = ExpDesc::Integer(Value::Integer);
        let float_exp = ExpDesc::Float(Value::Float);
        if let ExpDesc::Integer(f) = int_exp {
            println!("{:}", f(1));
        }
        if let ExpDesc::Float(f) = float_exp {
            println!("{:}", f(1.3234));
        }
    }
}