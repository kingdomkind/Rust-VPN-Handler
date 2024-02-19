# Rust VPN Handler
This is a simple script, written in rust, which allows you to easily enable a wireguard interface, disable one, toggle between them, and see which is currently active. This is mainly for personal usage. Which interface is chosen, is at random. It internally just calls wg-quick.

# Usage
1) Place all desired interfaces (.conf files) in /etc/wireguard.
2) Bind shortcuts to ./[VPNEXECUTABLE] [arguments]. Please ensure you have a way of answering the sudo request command, eg. by pre-pending konsole -e, or another terminal (konsole -e sudo filepath/vpn_handler/target/release/vpn_handler up, in my case)
3) Call the executable on startup, eg. via a systemd service.

### List of arguments
swap - Swaps the active interface with another (internally just calls down then up)
up - Brings an interface up, if one is not already active
down - Brings the active interface down, if there is one
which - Prints the active interface's name

# Building
Clone the repository, then simply run "cargo run", with administrator privileges (eg. sudo cargo run). Pass the relevant arguments to the program during this.

# Extra
Please ensure you have a way to respond to the sudo request, I do it by simply adding konsole -e before the command, and then I can enter my sudo password. Although I can't comment on the quality of the rust code as I don't do rust much, it has been very reliable for me, once it has been setup.

