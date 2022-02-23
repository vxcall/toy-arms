use std::str::Utf8Error;

pub(crate) unsafe fn read_null_terminated_string(base_address: usize) -> Result<String, Utf8Error> {
    let mut name: Vec<u8> = Vec::new();
    let mut i: isize = 0;
    loop {
        let char_as_u8 = *(base_address as *const u8).offset(i);
        if char_as_u8 == 0 {
            return Ok(std::str::from_utf8(&name[..])?.to_owned());
        }
        name.push(char_as_u8);
        i += 1;
    }
}
