#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const SOURCE: &str = "func main() {var a = 10; var b=20; return a + b}\0";

        unsafe {
            // // error reporting callback called by both compiler or VM
            // fn report_error (vm: *mut gravity_vm, error_type: error_type_t,
            //                   description: *const c_char, error_desc: error_desc_t, xdata: *mut c_void) {
            //     printf("%s\n", description);
            //     exit(0);
            // }
            // configure a VM delegate
            let mut delegate: gravity_delegate_t = std::mem::zeroed();

            // compile Gravity source code into bytecode
            let compiler = gravity_compiler_create(&mut delegate);
            let closure = gravity_compiler_run(
                compiler,
                SOURCE.as_ptr() as *const i8,
                (SOURCE.len() - 1) as _,
                0,
                true,
                true,
            );

            // sanity check on compiled source
            if closure.is_null() {
                // an error occurred while compiling source code and it has already been reported by the report_error callback
                gravity_compiler_free(compiler);
                panic!();
            }

            // create a new VM
            let vm = gravity_vm_new(&mut delegate);

            // transfer objects owned by the compiler to the VM (so they can be part of the GC)
            gravity_compiler_transfer(compiler, vm);

            // compiler can now be freed
            gravity_compiler_free(compiler);

            // run main closure inside Gravity bytecode
            if gravity_vm_runmain(vm, closure) {
                // print result (INT) 30 in this simple example
                let result = gravity_vm_result(vm);
                gravity_value_dump(vm, result, std::ptr::null_mut(), 0);
            }

            // free VM memory and core libraries
            gravity_vm_free(vm);
            gravity_core_free();
        }
    }
}
