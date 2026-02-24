#### Authors
- Shems PETREMAND : shems.petremand.etu@univ-lille.fr
- Eva BOUDIER : eva.boudier.etu@univ-lille.fr

# FTP Server Project
| crate     | details   |
|-----------|-----------|
| [ftp_server (src/)](src/)                 | Defines the ftp_server, its components and command line interface.            |
| --*dependencies*--                        | -- |
| [ftp_command_parser](ftp_command_parser/) | Defines an abstract and extendable initial interface to parse ftp commands.   |
| [ftp_response_impl](ftp_response_impl/)   | Defines a procedural macro to simplify implementing new Ftp server responses. |