use std::process::Command;
use rand::Rng;
use std::env;

fn main() 
{
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2
    { println!("invalid number of arguments - only one argument can be supplied - returning."); return; }

    match &arguments[1][..] {
        "up" => {up_vpn(); return;},
        "down" => {down_vpn(); return;},
        "swap" => {swap_vpn(); return;},
        "which" => {which_vpn(); return;},
        _ => {println!("Invalid Argument Supplied!"); return;},
    }
}

fn up_vpn()
{

    let currently_active = get_active_vpn();

    if currently_active != "0"
    {
        println!("{} is already active - did nothing.", currently_active);
        return;
    }

    // Uncomment this and change to desired IPs to allow IP routing
    /*
    Command::new("sh")
        .arg("-c")
        .arg("sudo ip route add 192.168.0.0/24 via 127.0.0.1 dev enp5s0")
        .output()
        .expect("failed to execute process");
    */

    let bash_server = Command::new("ls").current_dir("/etc/wireguard").output().expect("process failed to execute");
    let string_servers = String::from_utf8(bash_server.stdout).unwrap();
    let available_servers = string_servers.lines().collect::<Vec<_>>();
    let chosen_index = rand::thread_rng().gen_range(0..available_servers.len());
    let chosen_server = &available_servers[chosen_index][0..available_servers[chosen_index].len()-5];
    let final_string = "sudo wg-quick up ".to_owned() + chosen_server;

    println!("{} was enabled.", chosen_server);
    Command::new("sh")
    .arg("-c")
    .arg(final_string)
    .output()
    .expect("failed to execute process");
}

fn down_vpn()
{    
    let disabled_vpn = get_active_vpn();

    if disabled_vpn == "0"
    {
        println!("No VPN is currently online - did nothing.");
        return;
    }

    // Cleaning up IP routes
    /*
    Command::new("sh")
        .arg("-c")
        .arg("sudo ip route del 192.168.0.0/24 via 127.0.0.1 dev enp5s0")
        .output()
        .expect("failed to execute process");
    */

    let final_string = "sudo wg-quick down ".to_owned() + &disabled_vpn;
    Command::new("sh")
        .arg("-c")
        .arg(final_string)
        .output()
        .expect("failed to execute process");

    println!("{} was disabled.", disabled_vpn);
}

fn swap_vpn()
{
    down_vpn();
    up_vpn();
}

fn which_vpn()
{
    let active_vpn = get_active_vpn();

    if active_vpn == "0"
    {
        println!("No VPN is currently online.");
        return;
    }
    println!("{} is currently active.", get_active_vpn());
}

fn get_active_vpn() -> String
{
    let vpn_output = Command::new("sh")
        .arg("-c")
        .arg("sudo wg show")
        .output()
        .expect("failed to execute process");
    
    let vpn_output = String::from_utf8(vpn_output.stdout).unwrap();
    let chunks: Vec<_> = vpn_output.split(" ").collect();
    
    if chunks.len() > 1 {
        return chunks[1][0..chunks[1].len()-1].to_owned();
    } else {
        return String::from("0");
    }
}
