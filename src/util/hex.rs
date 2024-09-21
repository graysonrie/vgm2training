pub fn to_u32(hex: &str) -> u32 {
    u32::from_str_radix(hex, 16).expect("Invalid hex input")
}
pub fn to_hex(num:u32) -> String{
    format!("{:X}", num)
}