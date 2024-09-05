use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Derection {
    Up(DerectionUp),     // will generate impl From<DerectionUp> for Derection
    Down { speed: u32 }, // will not generate impl From<u32> for Derection
    Left(u32),           // will generate impl From<u32> for Derection
    Right(u32, u32),     // will not generate impl From<(u32, u32)> for Derection
}

#[allow(unused)]
#[derive(Debug)]
struct DerectionUp {
    speed: u32,
}
fn main() {
    // let up = Derection::Up(DerectionUp { speed: 10 });
    let up: Derection = DerectionUp::new(10).into();
    let left: Derection = 10.into();

    print!("{:#?}", up);
    print!("{:#?}", left);
}

impl DerectionUp {
    fn new(speed: u32) -> Self {
        DerectionUp { speed }
    }
}
