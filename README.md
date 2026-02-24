#### Authors
- Shems PETREMAND : shems.petremand.etu@univ-lille.fr
- Eva BOUDIER : eva.boudier.etu@univ-lille.fr

# FTP Server Project
| resource  | details   |
|-----------|-----------|
| [ftp_server (src/)](src/)                 | Defines the ftp_server, its components and command line interface.            |
| --*dependencies*--                        | -- |
| [ftp_command_parser](ftp_command_parser/) | Defines an abstract and extendable initial interface to parse ftp commands.   |
| [ftp_response_impl](ftp_response_impl/)   | Defines a procedural macro to simplify implementing new Ftp server responses. |
| --*CI*--                                  | -- |
| [documentation](https://gitlabpages.univ-lille.fr/sr1-tp2-petremand-boudier-2de2f6/ftp_server/index.html) | A complete web documentation of the project. |

## Usage

You can run the project with:
```
cargo run --release -- [OPTIONS] [HOSTNAME]
```

You can see available options, format and brief command documentation with :
```
cargo run --release -- -h
```

*--release is optional, you can run it without the option for a faster compilation time.  
**It is also possible to call the command directly from the built binary, on standard configuration, you will find them under `target/debug/ftp_server` and `target/release/ftp_server`.

## Brief

The project includes all the main requested features:
- The requested `ftp_server` command.
- Tree navigation.
- Download/Upload of text and binary files.
- Root directory configuration & traversal protection (see command options `-t` and main command arg).
- Recursive download/upload.
- Delete, create and rename resources.

And some additionnal features:
- User hashed authentification with user configuration file (see command options `-U`).*
- Active and passive mode.
- REST and SYST commands.

*These users are purely authentication keys, and are not used for specific server permissions, but they could be later extended to do so !

All these have been tested using filezilla.

It does not include :
- Unit tests.

You can read more about the structure and implementation of the project in the dedicated [documentation page](https://gitlabpages.univ-lille.fr/sr1-tp2-petremand-boudier-2de2f6/ftp_server/index.html).