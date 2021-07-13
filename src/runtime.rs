use crate::builtins;

use wasmi::FuncRef;
use wasmi::ValueType;
use wasmi::RuntimeValue;

use wasmi::Externals;
use wasmi::ImportResolver;

use std::collections::BTreeMap;


pub type HostFunc = fn(wasmi::RuntimeArgs<'_>) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap>;

pub struct Global {
    func_id: usize,
    modules: BTreeMap<&'static str, BTreeMap<&'static str, FuncRef>>,
    funcs: BTreeMap<usize, HostFunc>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            func_id: 1000,
            modules: BTreeMap::new(),
            funcs: BTreeMap::new(),
        }
    }

    pub fn add_func(
        &mut self, 
        module_name: &'static str, 
        func_name: &'static str, 
        func_params: &'static [wasmi::ValueType], 
        func_ret: Option<wasmi::ValueType>,
        func: HostFunc, 
    ) {
        if !self.modules.contains_key(module_name) {
            self.modules.insert(module_name, BTreeMap::new());
        }

        let module = self.modules.get_mut(module_name).unwrap();

        self.func_id += 1;
        let func_signature = wasmi::Signature::new(func_params, func_ret);
        let func_ref = wasmi::FuncInstance::alloc_host(func_signature, self.func_id);

        module.insert(func_name, func_ref);

        self.funcs.insert(self.func_id, func);
    }

    pub fn host_fn_call(&self, func_id: usize, args: wasmi::RuntimeArgs<'_>) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        if let Some(func) = self.funcs.get(&func_id) {
            func(args)
        } else {
            Err(wasmi::Trap::new(wasmi::TrapKind::Unreachable))
        }
    }
}

#[allow(unused_variables)]
impl ImportResolver for Global {
    fn resolve_func(&self, module_name: &str, field_name: &str, signature: &wasmi::Signature) -> Result<wasmi::FuncRef, wasmi::Error> {
        if let Some(module) = self.modules.get(module_name) {
            if let Some(func_ref) = module.get(field_name) {
                Ok(func_ref.clone())
            } else {
                Err(wasmi::Error::Instantiation(String::from("ops")))
            }
        } else {
            Err(wasmi::Error::Instantiation(String::from("ops")))
        }
    }

    fn resolve_global(&self, module_name: &str, field_name: &str, descriptor: &wasmi::GlobalDescriptor) -> Result<wasmi::GlobalRef, wasmi::Error> {
        Err(wasmi::Error::Instantiation(String::from("ops")))
    }

    fn resolve_memory(&self, module_name: &str, field_name: &str, descriptor: &wasmi::MemoryDescriptor) -> Result<wasmi::MemoryRef, wasmi::Error> {
        Err(wasmi::Error::Instantiation(String::from("ops")))
    }

    fn resolve_table(&self, module_name: &str, field_name: &str, descriptor: &wasmi::TableDescriptor) -> Result<wasmi::TableRef, wasmi::Error> {
        Err(wasmi::Error::Instantiation(String::from("ops")))
    }
}

impl Externals for Global {
    fn invoke_index(&mut self, index: usize, args: wasmi::RuntimeArgs<'_>) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        self.host_fn_call(index, args)
    }
}


pub fn load_global_builtins() -> Global {
    let mut global = Global::new();

    // NOTE: 注入 宿主 环境里面的函数，使 WASM CODE 可以直接调用该函数代码，以达到较好的性能。
    global.add_func("env", "fil_mul", &[ValueType::I32, ValueType::I32], Some(ValueType::I32), |args| {
        let a: u32 = args.nth_checked(0)?;
        let b: u32 = args.nth_checked(1)?;

        let ret = builtins::fil_mul(a, b);

        Ok(Some(RuntimeValue::from(ret)))
    });

    global
}


pub struct Vm {
    builtins: Global,
    module: wasmi::Module,
}

impl Vm {
    pub fn new(builtins: Global, wasm_code: &[u8]) -> Result<Self, wasmi::Error> {
        let module = wasmi::Module::from_buffer(wasm_code)?;

        Ok(Self { builtins, module })
    }

    pub fn exec_test_fn_mul(&mut self, a: i32, b: i32) -> Result<Option<wasmi::RuntimeValue>, wasmi::Error> {
        let instance = wasmi::ModuleInstance::new(&self.module, &self.builtins)?.assert_no_start();
        let args = [ RuntimeValue::I32(a), RuntimeValue::I32(b), ];

        instance.invoke_export("mul", &args, &mut self.builtins)
    }

    pub fn start(&mut self) -> Result<Option<wasmi::RuntimeValue>, wasmi::Error> {
        let instance = wasmi::ModuleInstance::new(&self.module, &self.builtins)?.assert_no_start();
        instance.invoke_export("fil_start", &[], &mut self.builtins)
    }
}
