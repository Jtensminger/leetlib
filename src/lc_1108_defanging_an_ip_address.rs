/*  
    Given a valid (IPv4) IP address, return a defanged version of that IP address.
    A defanged IP address replaces every period "." with "[.]"
*/
pub fn defang_i_paddr(address: String) -> String {    
        address.replace(".", "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext1() {
        let address = String::from("1.1.1.1");
        let target = String::from("1[.]1[.]1[.]1");
        assert_eq!(target, defang_i_paddr(address));
    }
}
