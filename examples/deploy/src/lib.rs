#![no_std]

use hex_literal::hex;
use wasm0x_zkwasmsdk::evm_return;

const CONTRACT_BYTECODE: [u8; 320 / 2] = hex!("0061736d0100000001090260027f7f0060000002130103656e760b5f65766d5f72657475726e00000302010105030100110619037f01418080c0000b7f00418c80c0000b7f00419080c0000b072c04066d656d6f72790200046d61696e00010a5f5f646174615f656e6403010b5f5f686561705f6261736503020a0d010b00418080c000410c10000b0b150100418080c0000b0c48656c6c6f2c20576f726c64");

#[no_mangle]
pub extern "C" fn main() {
    evm_return(CONTRACT_BYTECODE.as_ptr(), CONTRACT_BYTECODE.len() as u32);
}