#![allow(unreachable_code)]
pub extern crate jimtcl_sys;

use std::ffi::*;
use std::os::raw::*;

use jimtcl_sys::*;

#[derive(Debug, PartialEq)]
pub enum JimVal {
    Str(String),
    Number(f64),
    // Dict
    // Bool
}
/// A wrapper around the Jim Tcl interpeter. Not thread safe.
pub struct Interpreter {
    inner: *mut Jim_Interp,
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

    pub fn add_command(&self, _cmd: Box<Command>) {}

    pub fn get_val(&self, name: &str) -> Option<JimVal> {
        // get by name
        let length = name.len();
        let cname = CString::new(name).unwrap();

        // Allocate an object to hold the value returned from the interpreter.
        let jim_obj = unsafe {
            Jim_NewStringObj(self.inner, cname.as_ptr() as *const c_char, length as c_int)
        };

        // Jim_GetVariable returns NULL if var name not found.
        let res = unsafe { Jim_GetVariable(self.inner, jim_obj, JIM_NONE as i32) };
        if res.is_null() {
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
        // jim.c - look for `static const Jim_ObjType` declarations
        match type_name {
            // scriptline
            // script
            // command
            // variable
            // reference
            // int
            // coerced-double
            // double
            // list
            // dict
            // index
            // return-code
            // expression
            // scanformatstring
            // get-enum
            // dict-substitution
            // interpolated
            // string
            // compared-string
            // source
            "source" => unsafe {
                let b = (*res).bytes;
                let s = CStr::from_ptr(b).to_str().unwrap().to_owned();
                Jim_Free(jim_obj as *mut c_void);
                return Some(JimVal::Str(s));
            },
            _ => unimplemented!("got type: {:?}", type_name),
        }

        unsafe { Jim_Free(jim_obj as *mut c_void) };
        None
    }
}

pub trait Command {
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MyCmd;
    impl Command for MyCmd {
        fn name(&self) -> &str {
            "mycmd"
        }
    }
    #[test]
    fn simple_get_val() {
        let i = Interpreter::new();
        let code = "
        # This is Tcl code
        set a 69
        set b 420
        if 0 { puts hello }
        proc procName { argument } {
            # script contents
            return
        }
        set d [dict create foo baz]
        ";
        i.eval(code);
        assert_eq!(Some(JimVal::Str("69".to_string())), i.get_val("a"));
        assert_eq!(Some(JimVal::Str("420".to_string())), i.get_val("b"));
        assert_eq!(None, i.get_val("c"));
        assert_eq!(None, i.get_val("d"));
    }
}
