use smartstring::alias::String;
#[inline]
pub(crate) unsafe fn read_null_terminated_string(base_address: usize) -> Option<String> {
    let mut name: Vec<u8> = Vec::with_capacity(20);
    for i in 0.. {
        let char_as_u8 = *(base_address as *const u8).offset(i);
        if char_as_u8 == 0 {
            return Some(name.iter().map(|&f| f as char).collect::<String>());
        }
        name.push(char_as_u8);
    }
    None
}
