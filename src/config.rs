use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
    tls_key: String,
    tls_cer: String,
}

impl Config {
    pub fn new() -> Self {
        let vals = Self::init();
        Self {
            port: vals.get("port").unwrap().parse::<u16>().unwrap(),
            host: vals.get("host").cloned().unwrap(),
            tls_cer: vals.get("tls_cer").cloned().unwrap(),
            tls_key: vals.get("tls_key").cloned().unwrap(),
        }        
    }
    
    pub fn print_config(&self) {
        println!("{:?}", &self);
    }

    fn init() -> HashMap<String, String> {
        let mut map = HashMap::new();    
        //set default
        map.insert("port".to_string(), "443".to_string());
        map.insert("host".to_string(), "127.0.0.1".to_string());
        map.insert("tls_key".to_string(), "srv.key".to_string());
        map.insert("tls_cer".to_string(), "srv.cer".to_string());        
        //set from file
        dotenv().ok();
        //todo set from vault 
        //set from env
        for (key, value) in env::vars() {            
            map.insert(key.to_lowercase(), value); 
        }
        
        return map;
    }
}

#[test]
fn test_new() {
    let cfg = Config::new();
    assert_eq!(cfg.port, 443);
    assert_eq!(cfg.host, "127.0.0.1");
    assert_eq!(cfg.tls_cer, "srv.cer");
    assert_eq!(cfg.tls_key, "srv.key");
}

#[test]
fn test_env() {
    env::set_var("port", "80");
    env::set_var("host", "0.0.0.0");
    let cfg = Config::new();
    assert_eq!(cfg.port, 80);
    assert_eq!(cfg.host, "0.0.0.0");    
}



#[test]
fn test_print() {
    let cfg = Config::new();
    cfg.print_config();
}
