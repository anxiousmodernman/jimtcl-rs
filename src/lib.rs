pub extern crate jimtcl_sys;

use jimtcl_sys::*;

struct Interpreter<T> {
    inner: Box<T>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interp = unsafe { Jim_CreateInterp() };
        Interpreter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let i = Interpreter::new();
    }
}
