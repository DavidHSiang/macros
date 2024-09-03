use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // f3(f2(f1("Hello, world!")?)?)?;
    my_try!(f3(my_try!(f2(my_try!(f1("Hello, world!"))))));
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1: {}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2: {}", s.as_ref()))
}

fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3: {}", s.as_ref()))
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
