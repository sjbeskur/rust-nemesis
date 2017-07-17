extern crate ipaddress;
//extern crate num;

#[cfg(test)]
mod tests {
    use ipaddress::IPAddress;

    #[test]
    fn it_works() {
        println!("hello nemesis")
    }


    #[test]
    fn test_ipv4(){
        let ip = IPAddress::parse("192.168.100.0/24").unwrap();            
        println!("{}", ip.to_s());
        assert_eq!("192.168.100.254", ip.last().to_s());
    }

    #[test]
    pub fn test_method_each() {
        let ip = IPAddress::parse("10.0.0.1/29").unwrap();
        let arr = Arc::new(Mutex::new(Vec::new()));
        ip.each(|i| arr.lock().unwrap().push(i.to_s()));
        assert_eq!(*arr.lock().unwrap().deref(),
                   ["10.0.0.0", "10.0.0.1", "10.0.0.2", "10.0.0.3", "10.0.0.4", "10.0.0.5",
                    "10.0.0.6", "10.0.0.7"]);
    }    
}