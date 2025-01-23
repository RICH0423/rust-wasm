use wapc_guest as wapc;

#[no_mangle]
pub fn wapc_init() {
  wapc::register_function("ping", ping);
}

fn ping(msg: &[u8]) -> wapc::CallResult {
  wapc::console_log(&format!(
    "IN_WASM: Received request for `ping` operation with payload : {}",
    std::str::from_utf8(msg).unwrap()
  ));

  let payload = std::str::from_utf8(msg)?;
  let _res = wapc::host_call("binding", "sample:namespace", "pong", msg)?;
  let pong_res = std::str::from_utf8(&_res)?;
  let output = format!("{}, {}!", pong_res, payload);
  
  Ok(output.as_bytes().to_vec())
}
