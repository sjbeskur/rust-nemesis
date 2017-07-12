use std::io;
use std::time::Duration;
//use snmp::SyncSession;s
//use snmp::Value;
use snmp::{SyncSession, Value};
use rayon::prelude::*;
extern crate snmp;
extern crate rayon;

fn main() {
    println!("Hello, NemesisS!");

    println!("Enter an IPv4 address to poll: ");
    let mut ip_addr = String::new();
    io::stdin().read_line(&mut ip_addr).expect("Failed to read line");
    let agent_addr = ip_addr + ":161";

    let ips = vec!["10.10.1.254","10.10.1.253","10.10.1.252"];
    let test = ips.iter().map(|x| x).collect::<Vec<_>>();
    for n in test{
        println!("{}",n);
    }

    // need to parallelize this?
    for n in  1..10{     
        let rslt = snmp_get(&agent_addr); // I don't get why i have to do this to make it work
        println!("{} - {}",n,rslt);
    }
    
}

fn snmp_get(addr: &String) -> String {

    let sys_descr_oid = &[1,3,6,1,2,1,1,1,];
    let community  = b"public";  // byte syntax 
    let timeout    = Duration::from_secs(2);

    let mut sess = SyncSession::new(addr, community, Some(timeout), 0).unwrap();
    let mut response = sess.getnext(sys_descr_oid).unwrap();

    let mut result = String::new();
    if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
        result = format!("home router sysDescr {}", String::from_utf8_lossy(sys_descr));
    }

    result
}


