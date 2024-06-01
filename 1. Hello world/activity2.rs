use std::fmt;


struct Structure(i8);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn main() {
    println!("This struct `{}` won't print...", Structure(3));
}
