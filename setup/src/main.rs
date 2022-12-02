use exile::{Document, Element};
use phf::phf_map;
use regex::Regex;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{Read, Write},
    process::{Command, Stdio},
};
use strip_bom::StripBom;

fn get_env<S: AsRef<str> + std::fmt::Display>(key: S) -> Option<String> {
    const ENVS: phf::Map<&str, &str> = phf_map! {
        "SERVER_NAME" => "TMNF Docker Server",
        "SERVER_PASS" => "P@ssw0rd123",
        "MYSQL_DATABASE" => "aseco",
        "MYSQL_USER" => "tmf",
        "MYSQL_PASSWORD" => "MYSQL_P4SS",
        "MYSQL_ROOT_PASSWORD" => "MYSQL_R00T_P4SS",
        "SERVER_PORT" => "2350",
        "P2P_PORT" => "3450",
        "RPC_PORT" => "5000",
        "ADMINS" => "",
        "AUTOSAVE" => "OFF",
        "RANDOM_MAP_ORDER" => "0",
    };

    match env::var(key.as_ref()) {
        Ok(val) => Some(val),
        Err(_e) => match ENVS.get(key.as_ref()) {
            Some(val) => Some(val.to_string()),
            None => {
                println!("Requested key \"{key}\" not found!");
                None
            }
        },
    }
}

fn load_xml(path: &str) -> exile::error::Result<Document> {
    let file = fs::read_to_string(path).expect(format!("File not found: <{}>", path).as_str());

    exile::parse(file.strip_bom())
}

fn set_text<S: AsRef<str>>(el: &mut Element, node: &str, text: S) {
    el.child_mut(node).unwrap().set_text(text.as_ref()).unwrap();
}

fn boolean_env<S: AsRef<str>>(key: S) -> Result<bool, ()> {
    match get_env(key.as_ref()) {
        Some(x) => Ok(x.eq_ignore_ascii_case("ON") || x.eq_ignore_ascii_case("TRUE")),
        None => Err(()),
    }
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
    let mut doc = load_xml(path).unwrap();

    authorization_levels(doc.root_mut().child_mut("authorization_levels").unwrap());
    masterserver_account(doc.root_mut().child_mut("masterserver_account").unwrap());
    server_options(doc.root_mut().child_mut("server_options").unwrap());
    system_config(doc.root_mut().child_mut("system_config").unwrap());

    doc.save(path).unwrap();
}

fn localdatabase() {
    let path = "xaseco/localdatabase.xml";
    let mut doc = load_xml(path).unwrap();

    const GAME_MODE: phf::Map<&str, &str> = phf_map! {
        "mysql_login" => "MYSQL_USER",
        "mysql_password" => "MYSQL_PASSWORD",
        "mysql_database" => "MYSQL_DATABASE",
    };

    GAME_MODE.entries().for_each(|(key, value)| {
        doc.root_mut()
            .child_mut(key)
            .unwrap()
            .set_text(get_env(value).unwrap())
            .unwrap()
    });

    doc.save(path).unwrap();
}

fn config() {
    let path = "xaseco/config.xml";
    let mut doc = load_xml(path).unwrap();

    let aseco = doc.root_mut().child_mut("aseco").unwrap();
    let masteradmins = aseco.child_mut("masteradmins").unwrap();

    for admin in get_env("ADMINS").unwrap().split(",") {
        let mut el = Element::from_name("tmlogin");
        el.add_text(admin);
        masteradmins.add_child(el);
    }

    if boolean_env("AUTOSAVE").unwrap() {
        set_text(
            aseco,
            "default_tracklist",
            get_env("GAME_CONFIG").unwrap_or("NationsBlue.txt".to_string()),
        );
    }

    let tmserver = doc.root_mut().child_mut("tmserver").unwrap();
    set_text(tmserver, "password", get_env("HOST_PASS").unwrap());
    set_text(tmserver, "port", get_env("RPC_PORT").unwrap());

    doc.save(path).unwrap();
}

fn dedimania() {
    let path = "xaseco/dedimania.xml";
    let mut doc = load_xml(path).unwrap();

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

fn guest_list() {
    let path = "GameData/Config/guestlist.txt";
    let mut doc = load_xml(path).unwrap();

    let guest_list = doc.root_mut();

    for admin in get_env("ADMINS").unwrap().split(",") {
        let mut player = Element::from_name("player");
        let mut login = Element::from_name("login");
        login.add_text(admin);
        player.add_child(login);
        guest_list.add_child(player);
    }

    doc.save(path).unwrap();
}

fn custom_gamemode() {
    let config_file = match get_env("GAME_CONFIG") {
        Some(file_name) => file_name,
        None => return println!("No config file specified."),
    };

    let path = format!("GameData/Tracks/MatchSettings/Nations/{config_file}");
    let mut doc = load_xml(path.as_str()).unwrap();

    set_text(
        doc.root_mut().child_mut("filter").unwrap(),
        "random_map_order",
        (boolean_env("RANDOM_MAP_ORDER").unwrap() as i32).to_string(),
    );

    let gameinfos = doc.root_mut().child_mut("gameinfos").unwrap();

    const GAME_MODE: phf::Map<&str, &str> = phf_map! {
        "Rounds" => "0",
        "TimeAttack" => "1",
        "Team" => "2",
        "Laps" => "3",
        "Stunts" => "4",
    };

    let game_mode_env = get_env("GAME_MODE").unwrap();

    set_text(
        gameinfos,
        "game_mode",
        GAME_MODE.get(game_mode_env.as_str()).unwrap(),
    );

    let mut set_text_env = |s| set_text(gameinfos, s, get_env(s.to_uppercase()).unwrap());

    match game_mode_env.as_str() {
        "Rounds" => {
            set_text_env("rounds_pointslimit");
            set_text_env("rounds_usenewrules");
            set_text_env("rounds_forcedlaps");
        }
        "TimeAttack" => {
            set_text_env("timeattack_limit");
        }
        "Team" => {
            set_text_env("team_pointslimit");
            set_text_env("team_maxpoints");
            set_text_env("team_usenewrules");
        }
        "Laps" => {
            set_text_env("laps_nblaps");
            set_text_env("laps_timelimit");
        }
        "Stunts" => {}
        _ => println!("Invalid game mode specified!!"),
    };

    doc.save(path).unwrap();
}

fn autosave() {
    println!("Autosave: {:?}", boolean_env("AUTOSAVE"));

    if !boolean_env("AUTOSAVE").unwrap() {
        return;
    }

    // https://www.tm-forum.com/viewtopic.php?t=26755

    let path = "xaseco/includes/rasp.settings.php";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(path)
        .unwrap();
    let mut contents = String::new();

    println!("{}", contents);

    file.read_to_string(&mut contents).unwrap();

    let variable_name = "autosave_matchsettings";
    let new_value = get_env("GAME_CONFIG").unwrap_or("NationsBlue.txt".to_string());

    let re = Regex::new(format!(r"^\$({variable_name}) = (?P<val>.*?);").as_str()).unwrap();
    let new_contents = re.replace(contents.as_str(), format!("${variable_name} = {new_value}"));

    println!("{}", new_contents);

    file.write_all(new_contents.as_bytes()).unwrap();
}

fn commands() {
    let game_config = format!(
        "/game_settings=MatchSettings/Nations/{}",
        get_env("GAME_CONFIG").unwrap_or("NationsBlue.txt".to_string())
    );

    Command::new("./TrackmaniaServer")
        .current_dir("/tmnf")
        .args([game_config.as_str(), "/dedicated_cfg=dedicated_cfg.txt"])
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
    guest_list();

    custom_gamemode();
    autosave();

    commands();

    println!("Done!");
}
