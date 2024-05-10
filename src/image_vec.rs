use wasm_bindgen::Clamped;

pub fn blank(length: u32) -> Clamped<Vec<u8>> {
    let channels = length * 4;
    Clamped(vec![255; channels as usize])
}
