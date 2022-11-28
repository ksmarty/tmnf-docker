use exile::{error::Result, Document, Element};
use phf::phf_map;
use std::{
    env, fs,
    process::{Command, Stdio},
};
use strip_bom::StripBom;

fn get_env(key: &str) -> Option<String> {
    const ENVS: phf::Map<&str, &str> = phf_map! {
        "SERVER_NAME" => "TMNF Docker Server",
        "SERVER_PASS" => "PASSWORD123",
        "NATION" => "",
        "ADMINS" => "",
        "HOST_USER" => "",
        "HOST_PASS" => "",
        "MYSQL_DATABASE" => "aseco",
        "MYSQL_USER" => "tmf",
        "MYSQL_PASSWORD" => "password",
        "SERVER_PORT" => "2350",
        "P2P_PORT" => "3450",
        "RPC_PORT" => "5000",
    };

    match env::var(key) {
        Ok(val) => Some(val),
        Err(_e) => match ENVS.get(key) {
            Some(val) => Some(val.to_string()),
            None => {
                println!("Requested key \"{key}\" not found!");
                None
            }
        },
    }
}

fn load_file(file: &str) -> Result<Document> {
    let dedicated_cfg = fs::read_to_string(file).expect("File not found");

    exile::parse(dedicated_cfg.strip_bom())
}

fn set_text<S: AsRef<str>>(el: &mut Element, node: &str, text: S) {
    el.child_mut(node).unwrap().set_text(text.as_ref()).unwrap();
}

fn authorization_levels(authorization_levels: &mut Element) {
    for level in authorization_levels.children_mut() {
        set_text(level, "password", get_env("HOST_PASS").unwrap());
    }
}

fn masterserver_account(masterserver_account: &mut Element) {
    set_text(masterserver_account, "login", get_env("HOST_USER").unwrap());
    set_text(
        masterserver_account,
        "password",
        get_env("HOST_PASS").unwrap(),
    );
}

fn server_options(server_options: &mut Element) {
    set_text(server_options, "name", get_env("SERVER_NAME").unwrap());
    set_text(server_options, "password", get_env("SERVER_PASS").unwrap());
}

fn system_config(system_config: &mut Element) {
    set_text(
        system_config,
        "server_port",
        get_env("SERVER_PORT").unwrap(),
    );
    set_text(
        system_config,
        "server_p2p_port",
        get_env("P2P_PORT").unwrap(),
    );
    set_text(system_config, "xmlrpc_port", get_env("RPC_PORT").unwrap());

    set_text(system_config, "connection_uploadrate", "5120");
    set_text(system_config, "connection_downloadrate", "81920");
    set_text(system_config, "packmask", "nations");
}

fn dedicated_cfg() {
    let path = "GameData/Config/dedicated_cfg.txt";
    let mut doc = load_file(path).unwrap();

    authorization_levels(doc.root_mut().child_mut("authorization_levels").unwrap());
    masterserver_account(doc.root_mut().child_mut("masterserver_account").unwrap());
    server_options(doc.root_mut().child_mut("server_options").unwrap());
    system_config(doc.root_mut().child_mut("system_config").unwrap());

    doc.save(path).unwrap();
}

fn localdatabase() {
    let path = "xaseco/localdatabase.xml";
    let mut doc = load_file(path).unwrap();

    doc.root_mut()
        .child_mut("mysql_login")
        .unwrap()
        .set_text(get_env("MYSQL_USER").unwrap())
        .unwrap();
    doc.root_mut()
        .child_mut("mysql_password")
        .unwrap()
        .set_text(get_env("MYSQL_PASSWORD").unwrap())
        .unwrap();
    doc.root_mut()
        .child_mut("mysql_database")
        .unwrap()
        .set_text(get_env("MYSQL_DATABASE").unwrap())
        .unwrap();

    doc.save(path).unwrap();
}

fn config() {
    let path = "xaseco/config.xml";
    let mut doc = load_file(path).unwrap();

    let masteradmins = doc
        .root_mut()
        .child_mut("aseco")
        .unwrap()
        .child_mut("masteradmins")
        .unwrap();

    for admin in get_env("ADMINS").unwrap().split(",") {
        let mut el = Element::from_name("tmlogin");
        el.add_text(admin);
        masteradmins.add_child(el);
    }

    let tmserver = doc.root_mut().child_mut("tmserver").unwrap();
    set_text(tmserver, "password", get_env("HOST_PASS").unwrap());
    set_text(tmserver, "port", get_env("RPC_PORT").unwrap());

    doc.save(path).unwrap();
}

fn dedimania() {
    let path = "xaseco/dedimania.xml";
    let mut doc = load_file(path).unwrap();

    let masterserver_account = doc.root_mut().child_mut("masterserver_account").unwrap();
    set_text(masterserver_account, "login", get_env("HOST_USER").unwrap());
    set_text(
        masterserver_account,
        "password",
        get_env("HOST_PASS").unwrap(),
    );
    set_text(masterserver_account, "nation", get_env("NATION").unwrap());

    doc.save(path).unwrap();
}

fn commands() {
    Command::new("./TrackmaniaServer")
        .current_dir("/tmnf")
        .args([
            "/game_settings=MatchSettings/Nations/NationsBlue.txt",
            "/dedicated_cfg=dedicated_cfg.txt",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start TMNF server!");

    Command::new("php")
        .current_dir("/tmnf/xaseco")
        .args(["aseco.php", "TMF", "</dev/null", ">aseco.log", "2>&1"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to start Xaseco server!");
}

fn main() {
    println!("Starting!");

    dedicated_cfg();
    localdatabase();
    config();
    dedimania();

    commands();

    println!("Done!");
}
