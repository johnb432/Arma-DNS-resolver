use arma_rs::{arma, Extension};
use trust_dns_resolver::Resolver;

// To build, cd <directory>, then "cargo build"

#[arma]
fn init() -> Extension {
    Extension::build()
        .version("1.0.0".to_string())
        .command("get_ip", get_ip_from_dns)
        .finish()
}
// The command is what has to be called in Arma
// "arma_dns_resolver" callExtension ["get_ip", ["www.example.com"]] returns ["93.184.216.34",0,301]

pub fn get_ip_from_dns(hostname: String) -> String {
    // Use the host OS'es `/etc/resolv.conf`
    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(hostname).unwrap();

    // There can be many addresses associated with the name; This can return IPv4 and/or IPv6 addresses
    let address = response.iter().next().expect("no addresses returned!");

    if address.is_ipv4() {
        format!("{:?}", address)
    } else {
        format!("{:?}", "")
    }
}

#[cfg(test)]
mod tests {
    use super::init;

    #[test]
    fn get_ip_from_dns() {
        let extension = init().testing();
        let (result, _) = unsafe { extension.call("get_ip", Some(vec!["localhost".to_string()])) };
        assert_eq!(result, "127.0.0.1");
    }

    #[test]
    fn get_ip_from_dns2() {
        let extension = init().testing();
        let (result, _) =
            unsafe { extension.call("get_ip", Some(vec!["www.example.com".to_string()])) };
        assert_eq!(result, "93.184.216.34");
    }
}

// Only required for cargo, don't include in your library
//fn main() {}
