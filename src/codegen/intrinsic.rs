use crate::ast::ScalarType;
use inkwell::context::Context;
use inkwell::module::Module;
use std::collections::HashMap;

pub enum Intrinsic {
    Builtin(),
    FunctionPointer(),
}

pub struct Intrinsics {
    registry: HashMap<String, Intrinsic>,
}

impl Intrinsics {
    pub fn new() -> Self {
        Intrinsics {
            registry: HashMap::new(),
        }
    }

    pub fn llvm_numeric<T: AsRef<str>, S: ScalarType>(name: T, ty: S) -> String {
        format!("llvm.{}.{}", name.as_ref(), ty.scalar_repr())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::F32;
    use crate::Result;
    use inkwell::execution_engine::JitFunction;
    use inkwell::OptimizationLevel;

    type PowFunc = unsafe extern "C" fn(f32, f32) -> f32;

    #[test]
    fn test_intrinsic_pow() -> Result<()> {
        let ctx = Context::create();
        let module = ctx.create_module("pow");
        let builder = ctx.create_builder();
        let exec = module.create_jit_execution_engine(OptimizationLevel::None)?;
        let pow = Intrinsics::llvm_numeric("pow", F32);
        let f32_type = ctx.f32_type();
        let fn_type = f32_type.fn_type(&[f32_type.into(), f32_type.into()], false);
        let builtinpow = module.add_function(&pow, fn_type, None);
        let pow = module.add_function("pow", fn_type, None);
        let basic_block = ctx.append_basic_block(pow, "entry");
        builder.position_at_end(basic_block);
        let x = pow.get_nth_param(0).unwrap();
        let y = pow.get_nth_param(1).unwrap();
        let r = builder.build_call(builtinpow, &[x, y], "call");
        let basic_r = r.try_as_basic_value().left().unwrap();
        builder.build_return(Some(&basic_r));

        let f: JitFunction<PowFunc> = unsafe { exec.get_function("pow")? };

        let x = 5.0_f32;
        let y = 7.0_f32;

        unsafe {
            let r = f.call(x, y);
            println!("result = {}", r);
        }
        Ok(())
    }
}
