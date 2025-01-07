use std::fmt::{self, Display, Formatter};

struct CustomVec(Vec<u8>);

impl Display for CustomVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Values:\n")?;
        for v in &self.0 {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    let vec = CustomVec(vec![0, 0, 0, 0, 0]);
    println!("Vec: {}", vec);
}