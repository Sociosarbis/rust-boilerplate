use anyhow::Result;
use once_cell::sync::OnceCell;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex}, time::Instant,
};
use wasmer::{
    imports, Cranelift, Engine, Function, FunctionEnv, FunctionEnvMut, FunctionType, Instance,
    Memory, Module, Store, Type, Value,
};
use wasmer_wasi::{Stderr, Stdout, WasiState};

pub struct QuickJS {
    engine: Engine,
    module: Module,
}

impl Default for QuickJS {
    fn default() -> Self {
        let engine = Engine::from(Cranelift::default());
        let module =
            Module::from_binary(&engine, include_bytes!("../../quickjs-wasm/quickjs.wasm"))
                .unwrap();
        Self { engine, module }
    }
}

impl TryFrom<PathBuf> for QuickJS {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let engine = Engine::from(Cranelift::default());
        let module = Module::from_file(&engine, path)?;
        Ok(Self { engine, module })
    }
}

impl QuickJS {
    pub fn try_execute(
        &self,
        script: &str,
        data: Option<&str>,
        inherit_stdout: bool,
        inherit_stderr: bool,
    ) -> Result<Option<String>> {
        let input = script.as_bytes().to_vec();
        let input_size = input.len() as i32;
        let data = data
            .map(|data| data.as_bytes().to_vec())
            .unwrap_or_default();
        let data_size = data.len() as i32;
        let output = Arc::new(Mutex::new(None));
        let mut store = Store::new(self.engine.clone());

        let mut wasi_state = WasiState::new("quickjs_runner");
        if inherit_stdout {
            wasi_state.stdout(Box::new(Stdout::default()));
        }
        if inherit_stderr {
            wasi_state.stderr(Box::new(Stderr::default()));
        }
        let mut wasi_env = wasi_state.finalize(&mut store)?;

        let get_input_size_signature = FunctionType::new(vec![], vec![Type::I32]);

        let get_input_size = Function::new(&mut store, get_input_size_signature, move |_args| {
            Ok(vec![Value::I32(input_size)])
        });

        let get_data_size_signature = FunctionType::new(vec![], vec![Type::I32]);

        let get_data_size = Function::new(&mut store, get_data_size_signature, move |_args| {
            Ok(vec![Value::I32(data_size)])
        });

        struct MyEnv {
            memory: OnceCell<Memory>,
            input: Vec<u8>,
            data: Vec<u8>,
            output: Arc<Mutex<Option<String>>>,
        }

        let my_env = FunctionEnv::new(
            &mut store,
            MyEnv {
                memory: OnceCell::new(),
                input: input,
                data: data,
                output: output.clone(),
            },
        );

        fn get_input(env: FunctionEnvMut<MyEnv>, ptr: i32) {
            if let Some(mem) = env.data().memory.get() {
                let view = mem.view(&env);
                view.write(ptr as u64, &env.data().input)
                    .expect("can not get input");
                return;
            }
            panic!("failed to find host memory");
        }

        let get_input_typed = Function::new_typed_with_env(&mut store, &my_env, get_input);

        fn get_data(env: FunctionEnvMut<MyEnv>, ptr: i32) {
            if let Some(mem) = env.data().memory.get() {
                let view = mem.view(&env);
                view.write(ptr as u64, &env.data().data)
                    .expect("can not get data");
                return;
            }
            panic!("failed to find host memory");
        }
        let get_data_typed = Function::new_typed_with_env(&mut store, &my_env, get_data);

        fn set_output(env: FunctionEnvMut<MyEnv>, ptr: i32, capacity: i32) {
            let mut output = env.data().output.lock().unwrap();
            *output = if capacity == 0 {
                None
            } else {
                if let Some(mem) = env.data().memory.get() {
                    let mut buffer: Vec<u8> = vec![0; capacity as usize];
                    mem.view(&env)
                        .read(ptr as u64, &mut buffer)
                        .expect("can not read output buffer");
                    Some(String::from_utf8(buffer).expect("buffer can not form a utf8 string"))
                } else {
                    panic!("failed to find host memory")
                }
            }
        }

        let set_output_typed = Function::new_typed_with_env(&mut store, &my_env, set_output);

        let mut import_object = imports! {
            "host" => {
                "get_input" => get_input_typed,
                "get_input_size" => get_input_size,
                "get_data" => get_data_typed,
                "get_data_size" => get_data_size,
                "set_output" => set_output_typed
            }
        };

        import_object.extend(
            wasi_env
                .import_object(&mut store, &self.module)?
                .into_iter(),
        );
        let instance = Instance::new(&mut store, &self.module, &import_object)?;
        let m = instance.exports.get_memory("memory")?;
        wasi_env.initialize(&mut store, &instance)?;
        my_env.as_mut(&mut store).memory.set(m.clone()).unwrap();
        let start = instance.exports.get_function("_start")?;
        let run_start = Instant::now();
        start.call(&mut store, &[])?;
        let output = output.lock().unwrap();
        println!("elapsed: {:?}", run_start.elapsed());
        Ok(output.to_owned())
    }
}

