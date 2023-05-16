use std::process::Command;
use std::env;

use clap::Parser;

mod arg_list;

#[derive(Parser)]
#[command(name = "MCLI")]
#[command(author = "Joss Lei")]
#[command(version = "0.1")]
#[command(about = "MCLI - Minecraft Launcher CLI")]
#[command(long_about = None)]
struct CliOptions {
    game_dir: String,
    game_version: String,

    #[arg(long)]
    memory_min: u32,
    #[arg(long)]
    memory_max: u32,

    #[arg(long)]
    username: String,

    #[arg(long, default_value_t = 854)]
    display_width: u32,
    #[arg(long, default_value_t = 480)]
    display_height: u32,
    #[arg(long, default_value_t = false)]
    fullscreen: bool,

    #[arg(long)]
    server_address: String,
    #[arg(long)]
    server_port: u16
}

fn main() {
    let args = CliOptions::parse();

    let game_dir = args.game_dir;
    let game_version = args.game_version;

    let username = args.username;

    let memory_min = args.memory_min;
    let memory_max = args.memory_max;

    let display_width = args.display_width;
    let display_height = args.display_height;
    let fullscreen = args.fullscreen;

    let server_address = args.server_address;
    let server_port = args.server_port;

    run_minecraft(&game_dir,
                  &game_version,
                  memory_min, memory_max,
                  &username,
                  display_width, display_height,
                  fullscreen,
                  &server_address, server_port);
}

fn run_minecraft(game_dir: &String,
                 game_version: &String,
                 memory_min: u32, memory_max: u32,
                 username: &String,
                 display_width: u32, display_height: u32,
                 fullscreen: bool,
                 server_address: &String, server_port: u16) {
    let launcher_brand: String = "MCLI".to_string();
    let launcher_version: String = "0.1".to_string();

    // TODO: get their full path
    let const_para_game_dir: String = "./".to_string();
    let const_para_assets_dir: String = "../../assets/".to_string();
    let const_para_libraries_dir: String = "../../libraries/".to_string();

    let launcher_info_settings = arg_list::launcher_info_settings(&launcher_brand, &launcher_version);

    let encoding_settings = arg_list::encoding_settings();
    
    let misc_settings = arg_list::misc_settings(&const_para_game_dir);

    let memory_settings = arg_list::memory_settings(memory_min, memory_max);

    let performance_settings = arg_list::performance_settings();

    let game_start_settings = arg_list::game_start_settings(&game_version,
                                                            &const_para_game_dir,
                                                            &const_para_libraries_dir);

    let minecraft_settings = arg_list::minecraft_settings_server_autoconnect(
                            &game_version,
                            &const_para_game_dir,
                            &const_para_assets_dir,
                            &launcher_brand,
                            &launcher_version,
                            &username,
                            display_width,
                            display_height,
                            fullscreen,
                            3,
                            &server_address,
                            server_port);

    let game_dir_full = env::current_dir().expect("REASON").join(game_dir);
    Command::new("java")
        .current_dir(game_dir_full)
        .args(launcher_info_settings)
        .args(encoding_settings)
        .args(misc_settings)
        .args(memory_settings)
        .args(performance_settings)
        .args(game_start_settings)
        .args(minecraft_settings)
        .spawn()
        .expect("Minecraft failed to start.");
}

