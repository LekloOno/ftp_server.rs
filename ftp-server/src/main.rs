mod cli;

use clap::Parser;

use ftp_server::command::commands::{
    user::CmdUser, pass::CmdPass, pwd::CmdPwd, cwd::CmdCwd, 
    pasv::CmdPasv, cdup::CmdCdup, list::CmdList, quit::CmdQuit, 
    dtype::CmdType, retr::CmdRetr, stor::CmdStor, dele::CmdDele,
    mkd::CmdMkd, rmd::CmdRmd, syst::CmdSyst, port::CmdPort
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
        .register_command::<CmdStor>()
        .register_command::<CmdDele>()
        .register_command::<CmdMkd>()
        .register_command::<CmdRmd>()
        .register_command::<CmdSyst>()
        .register_command::<CmdPort>()
        .hostname(args.host)
        .port(args.port)
        .add_user(args.username, args.password)
        .root_path(args.root_path.canonicalize().unwrap())
        .protect_root(!args.allow_traversal)
        .run()
        .unwrap();
}