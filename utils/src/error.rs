#[cfg(test)]
mod tests {
    use anyhow::{anyhow, Context, Result};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::fs::read;
    use std::path::PathBuf;

    #[derive(Debug)]
    enum MyErrors {
        NotFound,
    }

    impl Display for MyErrors {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                MyErrors::NotFound => write!(f, "not found"),
            }
        }
    }

    impl Error for MyErrors {}

    #[test]
    fn test_read() -> Result<()> {
        let mut path = PathBuf::new();
        path.push("./non_existent_file.txt");
        read(path).context(MyErrors::NotFound)?;
        Ok(())
    }

    fn to_option() -> Option<Vec<u8>> {
        let mut path = PathBuf::new();
        path.push("./non_existent_file.txt");
        read(path).context("file not exists").ok()
    }

    #[test]
    fn test_to_option_1() -> Result<()> {
        let option = to_option();
        option.ok_or_else(|| anyhow!("option none"))?;
        Ok(())
    }

    #[test]
    fn test_to_option_2() -> Result<()> {
        let option = to_option();
        option.context("option none context")?;
        Ok(())
    }

    #[test]
    fn test_custom_error() -> Result<()> {
        Err(anyhow!(MyErrors::NotFound))
    }

    #[test]
    fn downcast_err() {
        match test_read() {
            Ok(_) => println!("read ok"),
            Err(e) => {
                match e.downcast_ref() {
                    Some(MyErrors::NotFound) => {
                      println!("downcast to not found error");
                    }
                    None => {
                        println!("can not downcast to MyErrors");
                    }
                }
                match e.downcast_ref() {
                    Some(std::io::Error { .. }) => {
                        println!("downcast to io error {:?}", e);
                    }
                    None => {
                        println!("can not downcast to io error");
                    }
                }
            }
        }
    }
}
