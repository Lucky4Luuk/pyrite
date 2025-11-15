use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Interface missing!")]
    InterfaceMissing,
    #[error("Failed to find entry point!")]
    EntryPointMissing,
    #[error("Unable to find function!")]
    FuncMissing,
}

pub fn run_task(bytes: &[u8]) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    let wasi = WasiCtx::builder().inherit_stdout().build();
    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
    };

    let mut store = Store::new(&engine, state);

    let component = Component::from_binary(&engine, bytes)?;
    let instance = linker.instantiate(&mut store, &component)?;
    let interface_idx = instance
        .get_export_index(&mut store, None, "wasi:cli/run@0.2.0")
        .ok_or(TaskError::InterfaceMissing)?;

    let parent_export_idx = Some(&interface_idx);
    let func_idx = instance
        .get_export_index(&mut store, parent_export_idx, "run")
        .ok_or(TaskError::EntryPointMissing)?;

    let func = instance
        .get_func(&mut store, func_idx)
        .ok_or(TaskError::FuncMissing)?;

    let typed = func.typed::<(), (Result<(), ()>,)>(&store)?;
    let (result,) = typed.call(&mut store, ())?;

    // The docs are making me do this (TypedFunc::call)
    typed.post_return(&mut store)?;

    result.map_err(|_| anyhow::anyhow!("error"))
}

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}
