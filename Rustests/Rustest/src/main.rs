use std::env;

use std::ffi::OsStr;
use windows_service::service::ServiceAccess;
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

// let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;
// let my_service = manager.open_service("my_service", ServiceAccess::START)?;
// my_service.start(&[OsStr::new("Started from Rust!")])?;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }
}

fn main() -> windows_service::Result<()> {
    // use std::env;
    // use windows_service::{
        // service::ServiceAccess,
        // service_manager::{ServiceManager, ServiceManagerAccess},
    // }; 

    let service_name = env::args().nth(1).unwrap_or("netlogon".to_owned());

    let manager_access = ServiceManagerAccess::CONNECT;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service = service_manager.open_service(service_name, ServiceAccess::QUERY_CONFIG)?;

    let config = service.query_config()?;
    println!("{:#?}", config);
    Ok(())
}

