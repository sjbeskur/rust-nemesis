use std::io;
use std::time::Duration;
//use snmp::SyncSession;s
//use snmp::Value;
use snmp::{SyncSession, Value};
//use rayon::prelude::*;
use ipaddress::IPAddress;
use std::sync::{Arc, Mutex};
use std::vec::*;

extern crate snmp;
extern crate rayon;
extern crate ipaddress;

fn main() {
    println!("Hello, NemesisS!");

    println!("Enter an IPv4 subnet to poll (e.g. 192.168.0.0/24): ");
    let mut ip_addr = String::new();
    io::stdin().read_line(&mut ip_addr).expect("Failed to read line");
        
    let ip = IPAddress::parse(ip_addr).unwrap();
    let arr = Arc::new(Mutex::new(Vec::new()));
    ip.each_host(|i| arr.lock().unwrap().push(i.to_s()));
    
    let list = arr.lock().unwrap();


    // TODO:// need to parallelize this?
    // TODO:// need error checking
    for i in 0..list.len(){
        let  s = list[i].clone();
        let agent_addr = s + ":161";
        println!(" - {} -",agent_addr);
        let rslt = snmp_get(&agent_addr); 
        println!("{} ",rslt);
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


