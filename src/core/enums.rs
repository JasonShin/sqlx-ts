use crate::common::lazy::{ CONFIG };

/// The module is used to find all known enums of DB connections that are provided via config


fn find_all_known_enums() {
    let connection_configs = &CONFIG.connections;
    let connection_keys = connection_configs.keys();

    for key in &connection_keys {

    }
}