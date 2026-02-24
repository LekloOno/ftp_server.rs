mod cli;

use clap::Parser;

use ftp_server::command::commands::{
    user::CmdUser, pass::CmdPass, pwd::CmdPwd, cwd::CmdCwd, 
    pasv::CmdPasv, cdup::CmdCdup, list::CmdList, quit::CmdQuit, 
    dtype::CmdType, retr::CmdRetr
};

use ftp_server::server::FtpServer;

fn main() {
    let args = cli::Cli::parse();

    FtpServer::new()
        .register_command::<CmdUser>()
        .register_command::<CmdPass>()
        .register_command::<CmdPwd>()
        .register_command::<CmdCwd>()
        .register_command::<CmdPasv>()
        .register_command::<CmdCdup>()
        .register_command::<CmdList>()
        .register_command::<CmdQuit>()
        .register_command::<CmdType>()
        .register_command::<CmdRetr>()
        .hostname(args.host)
        .port(args.port)
        .add_user(args.username, args.password)
        .root_path(args.root_path.canonicalize().unwrap())
        .protect_root(args.prevent_traversal)
        .run()
        .unwrap();
}