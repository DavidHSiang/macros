use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Derection<T> {
    Up(DerectionUp<T>),  // will generate impl From<DerectionUp> for Derection
    Down { speed: u32 }, // will not generate impl From<u32> for Derection
    Left(u32),           // will generate impl From<u32> for Derection
    Right(u32, u32),     // will not generate impl From<(u32, u32)> for Derection
}

#[allow(unused)]
#[derive(Debug)]
struct DerectionUp<T> {
    speed: T,
}
fn main() {
    // let up = Derection::Up(DerectionUp { speed: 10 });
    let up: Derection<i32> = DerectionUp::new(10).into();
    let left: Derection<u32> = 10.into();

    print!("{:#?}", up);
    print!("{:#?}", left);
}

impl<T> DerectionUp<T> {
    fn new(speed: T) -> Self {
        DerectionUp { speed }
    }
}
