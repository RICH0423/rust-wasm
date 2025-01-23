use std::error::Error;

use wapc::WapcHost;
use wasmtime_provider::WasmtimeEngineProviderBuilder;

pub fn main() -> Result<(), Box<dyn Error>> {

  // Sample host callback that prints the operation a WASM module requested.
  let host_callback = |id: u64, bd: &str, ns: &str, op: &str, payload: &[u8]| {
    println!("Guest {} invoked '{}->{}:{}' with a {} byte payload",
    id, bd, ns, op, payload.len());

    if op == "pong" {
      Ok(b"PONG".to_vec())
    } else {
      Err("Unsupported host call!".into())
    }
  };

  let file = "../wapc-guest/target/wasm32-unknown-unknown/release/wapc_guest.wasm";
  let module_bytes = std::fs::read(file)?;

  let engine = WasmtimeEngineProviderBuilder::new()
    .module_bytes(&module_bytes)
    .build()?;
  let host = WapcHost::new(Box::new(engine), Some(Box::new(host_callback)))?;

  let res = host.call("ping", b"waPC")?;
  let s = std::str::from_utf8(&res)?;
  println!("{}", s);
  assert_eq!(res, b"PONG, waPC!");

  Ok(())
}
