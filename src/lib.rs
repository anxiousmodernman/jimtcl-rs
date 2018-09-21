pub extern crate jimtcl_sys;

use std::ffi::*;
use std::os::raw::*;

use jimtcl_sys::*;

pub struct Interpreter {
    inner: *mut Jim_Interp,
}

#[derive(Debug, PartialEq)]
pub enum JimVal {
    Str(String),
    Number(f64),
}

impl Interpreter {
    pub fn new() -> Self {
        let interp = unsafe { Jim_CreateInterp() };
        unsafe { Jim_RegisterCoreCommands(interp) };
        Interpreter { inner: interp }
    }
    pub fn eval(&self, code: &str) {
        let c = CString::new(code).unwrap();
        unsafe {
            Jim_Eval(self.inner, c.as_ptr() as *const c_char);
        }
    }

    pub fn get_val(&self, name: &str) -> Option<JimVal> {
        // get by name
        let length = name.len();
        let cname = CString::new(name).unwrap();

        // Allocate an object to hold the value returned from the interpreter.
        let jim_obj = unsafe {
            Jim_NewStringObj(self.inner, cname.as_ptr() as *const c_char, length as c_int)
        };

        // Jim_GetVariable:
        //     returns NULL if var name not found
        let res = unsafe { Jim_GetVariable(self.inner, jim_obj, JIM_NONE as i32) };
        if res.is_null() {
            println!("returning none now!");
            return None;
        }
        let type_name = unsafe {
            // *const i8
            let ptr = (*(*res).typePtr).name;
            CStr::from_ptr(ptr as *const c_char)
                .to_str()
                .expect("could not access typeName of Jim_Obj")
        };
        // look up type: where are these enumerated?
        match type_name {
            "source" => unsafe {
                let s = (*res).bytes;
                let ss = CStr::from_ptr(s).to_str().unwrap().to_owned();
                return Some(JimVal::Str(ss));
            },
            _ => unimplemented!("got type: {:?}", type_name),
        }
        // return appropriate JimVal for type

        unsafe { Jim_Free(jim_obj as *mut c_void) };
        None
    }
}

pub trait Command {
    // init
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let i = Interpreter::new();
        let code = "set a 69";
        i.eval(code);
        assert_eq!(Some(JimVal::Str("69".to_string())), i.get_val("a"));
    }
}
