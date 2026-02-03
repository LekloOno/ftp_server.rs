# FTP Command Parser

This crate provides the simplest abstraction layer possible to build an FTP command. It aims at extensivity and modularity.

The exact type of the FTP command is highly abstract yet can be explicitly specialized to avoid casting and enable a smooth, performant, explicit and extensive architecture. More on that in the [brief implementation explanations](#brief-implementation-explanation).

Further behaviors of the FTP commands, such as how the command should execute and respond, is not the responsibility of this crate.

- [FTP Command Parser](#ftp-command-parser)
  - [Quick overview](#quick-overview)
  - [RFC Analysis](#rfc-analysis)
    - [Commands](#commands)
    - [Command Codes](#command-codes)
    - [Arguments format](#arguments-format)
    - [Characters encoding](#characters-encoding)
  - [RFC implementation guidelines](#rfc-implementation-guidelines)
  - [Brief implementation explanation](#brief-implementation-explanation)
    - [Why not `dyn FtpCommand`](#why-not-dyn-ftpcommand)
    - [Toward a better parser !](#toward-a-better-parser-)
    - [Command Builder](#command-builder)
  - [Demo](#demo)


## Quick overview

Unimplemented - quick note of what it would look like (actual implementation discussed [here](#brief-implementation-explanation), this is purely illustrative and overly-simplified)
```rs
pub struct FtpCommandParser<T: FtpCommand> {
    commands: Hashmap<String, Box<dyn Fn(Option<String>) -> Box<dyn T>>>,
}
```
Essentially, a hashmap of `T` constructors, where `T` is any kind of specific FTP command (for example, an executable command that would be a trait `pub trait ExecFtpCommand : FtpCommand`).

You would pre-register each Ftp commands. Each extension of `FtpCommand` should thus ideally provide quick-hand tools to simplify registration on an `FtpCommandParser<T: FtpCommand>`.



This architecture would provide two types of extensivity :
- RFC extensivity - The parsing is RFC agnostic, you can implement as much FtpCommand as required, and register them dynamically on the Parser.
- Features/Behavior extensivity - The parsing is behavior agnostic, nothing is determined about what an ftp session is, about what executing a command means, etc. You define this by typing a concrete `FtpCommandParser<T>` with `T` the further definition of `FtpCommand`.

The actual implementation is discussed [here](#brief-implementation-explanation).

A simple dummy implementation and usage example is provided in [src/main.rs](src/main.rs) and [src/dummy_impl.rs](src/dummy_impl.rs).

## RFC Analysis

Exact relevant quotes ([source](https://datatracker.ietf.org/doc/html/rfc959#autoid-4:~:text=5%2E3%2E-,COMMANDS)) :

> The commands begin with a command code followed by an argument
field.  The command codes are four or fewer alphabetic characters.
Upper and lower case alphabetic characters are to be treated
identically.

> This also applies to any symbols representing parameter values,
such as A or a for ASCII TYPE.  The command codes and the argument
fields are separated by one or more spaces.

> The argument field consists of a variable length character string
ending with the character sequence `<CRLF>` (Carriage Return, Line
Feed) for NVT-ASCII representation; for other negotiated languages
a different end of line character might be used.  It should be
noted that the server is to take no action until the end of line
code is received.

> The syntax is specified below in NVT-ASCII.  All characters in the
argument field are ASCII characters including any ASCII
represented decimal integers.  Square brackets denote an optional
argument field.  If the option is not taken, the appropriate
default is implied.

> The syntax of the above argument fields (using BNF notation where applicable) is:  
```
<username> ::= <string>
<password> ::= <string>
<account-information> ::= <string>
<string> ::= <char> | <char><string>
<char> ::= any of the 128 ASCII characters except <CR> and
<LF>
<marker> ::= <pr-string>
<pr-string> ::= <pr-char> | <pr-char><pr-string>
<pr-char> ::= printable characters, any
                ASCII code 33 through 126
<byte-size> ::= <number>
<host-port> ::= <host-number>,<port-number>
<host-number> ::= <number>,<number>,<number>,<number>
<port-number> ::= <number>,<number>
<number> ::= any decimal integer 1 through 255
<form-code> ::= N | T | C
<type-code> ::= A [<sp> <form-code>]
                | E [<sp> <form-code>]
                | I
                | L <sp> <byte-size>
<structure-code> ::= F | R | P
<mode-code> ::= S | B | C
<pathname> ::= <string>
<decimal-integer> ::= any decimal integer
```

### Commands
- Are a telnet character strings beginning with a command code followed by an argument field.

### Command Codes
- Are **four** or fewer alphabetic characters.
- Are case incensitive.


### Arguments format

Argument can be composed of mutliple fields, and some could be optionnal. But essentially, argument is just a string that ends with the encoding's `<CRLF>`.

**Conclusion :** To make it more explicit, we could further define an argument trait (typically for the initial RFC 959 implementation) to handle argument parsing. Yet, it seems the exact format of arguments is not specific to FTP, rather to the FTP extensions. The parser will thus only manipulate the string argument as the part succeeding `<CMD> <SP>` until `<CRLF>`. 

### Characters encoding

FTP RFC is encoding-agnostic.
- EBCDIC-based systems – Some older mainframes use EBCDIC instead of ASCII, we could allow that.
- UTF-8 or other multi-byte encodings – Modern extensions to FTP (like RFC 2640, UTF-8 filename support) allow using Unicode.

**Conclusion :** As UTF-8 itself supports ASCII and is the built-in rust String format, I'll stick to UTF-8 for now.


## RFC implementation guidelines

From the previous analysis, `FtpCommand` trait should remain very simple. Notably, the FTP arguments RFC seems to be more specific to RFC 959, and not FTP itself.

From a parser-perspective, it would probably be more relevant to only consider a command as `<CMD> [<SP> <ARGS>] <CRLF>`.

This is what we will go for, thus, the parser `Fn` constructors signature will only take an optionnal string argument, which corresponds to `[<ARGS>]`.

## Brief implementation explanation

What I'm trying to achieve is pretty intricated. I tried achieving similar design in a slightly more complicated project before, a modular RISC-V emulator. I think I found an interesting approach to do so.

Indeed, the snippet given at the beginning of this README is not correct, it was purely illustrative -
```rs
pub struct FtpCommandParser<T: FtpCommand> {
    commands: Hashmap<String, Box<dyn Fn(Option<String>) -> Box<dyn T>>>,
}
```

You cannot make a structure that is generic over a Trait, the semantic of generics implies that `T` must be a type. Thus, `dyn T` is not valid.  

But first, I want to explain the exact intentions behind such an (semantically invalid) `dyn T`.  

### Why not `dyn FtpCommand`

You would think the `FtpCommandParser` could simply build `dyn FtpCommand` instances. "Any" `FtpCommand`.

```rs
pub struct FtpCommandParser {
    commands: Hashmap<String, Box<dyn Fn(Option<String>) -> Box<dyn FtpCommand>>>,
}
```

But that is not extensive enough.  

If we did so, we would miss the functionnal/behavioral extensivity. For example, if we later define `ExecFtpCommand` which defines how an `FtpCommand` can be executed over an `FtpSession`, we would still be constrained to specifically use `FtpCommand` trait out of the command parser.

That is, such code below would be invalid -
```rs
let command_parser = FtpCommandParser::new();
command_parser.register::<MyExecutableCommand>();

let cmd_name = MyExecutableCommand::name();
let arg = Some("Linguanguliguliguli waga");

command_parser
    .get(cmd_name)
    .build(arg);
    .execute(); // ERROR !
```

The retrieved command is of type `Box<dyn FtpCommand>`, not `Box<dyn ExecFtpCommand>`. We would have to do casting, which is possible but defeits the purpose of a nice smooth and extensive architecture : we must be smarter !

### Toward a better parser !

A way to bypass this constraint is to specify that T should be unsized (implicitly, it is a `dyn`).
```rs
pub struct FtpCommandParser<T: FtpCommand + ?Sized + 'static> {
    commands: HashMap<&'static str, Box<dyn Fn(Option<String>) -> Box<T>>>,
}
```

The problem we might have now is that, although T is unsized, it is still considered a type. Thus, we cannot use T to specify our register method (by expecting a `U: T` as such isn't a valid semantic, `T` is not a trait).

This was where I stopped on my RISC-V emulator, but I thought of a potential solution - making the wrapping explicit through another building trait - as follows :
```rs
pub trait BuildableFtpCommand<T: FtpCommand + ?Sized>: FtpCommand + 'static {
    fn new(arg: Option<String>) -> Box<T>;
}
```

`BuildableFtpCommand` is a helper static trait, that is, it will not correspond to any structure instance, but it will be used as a helper for the implementing type on "trait" `T`. This trait will define how we explicitly transform the implementing command to a `T`, implicitly a given `dyn T` where `T` is a specification of `FtpCommand`.

This is the architecture I'll continue the project with. It's experimental for me, so it is highly subject to future revisions.

### Command Builder

To make the library smoother to use, I added another structure `FtpCommandBuilder` which wraps the constructor of a `BuildableFtpCommand` in a structure that is stored in the parser.

So instead of storing `dyn Fn` constructors directly, we store `FtpCommandBuilder` which themselves hold the `dyn Fn`.

We thus get a sweat and clear sequential syntax as -
```rs
let command = command_parser
        .get(code)?     // Retrives the FtpCommandBuilder from the given command code
        .build(arg)?;   // Builds the command with the given argument
```

## Demo

To demonstrate the modularity of this architecture, a simple `FtpCommand` implementation and extension is provided in [src/dummy_impl](src/dummy_impl.rs), which is used in the executable example [src/main.rs](src/main.rs).

The `FtpCommand` trait is extended with `Display` standard trait, wrapped under a blanket implementation `DisplayableFtpCommand` as such :

```rs
pub trait DisplayableFtpCommand : FtpCommand + std::fmt::Display {}

impl<T> DisplayableFtpCommand for T where T: FtpCommand + std::fmt::Display {}
```

We implement `Display` on our dummy command, then provide a builder method for the `dyn DisplayableFtpCommand` target as follows :

```rs
impl std::fmt::Display for DummyFtpCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", Self::name(), self.arg.as_ref().map_or("nothing !", String::as_str))
    }
}

impl BuildableFtpCommand<dyn DisplayableFtpCommand> for DummyFtpCommand {
    fn build(arg: Option<String>) -> Result<Box<dyn DisplayableFtpCommand>, FtpParseError> {
        Ok(Box::new(Self { arg }))
    }
}
```

Now, in our program, we can define a specialized `FtpCommandParser`, and register our dummy command.
```rs
let mut command_parser = FtpCommandParser::<dyn DisplayableFtpCommand>::new();
command_parser.register::<DummyFtpCommand>();
```

And if we try to parse a `DummyFtpCommand` and print it, that works !
```rs
let code = DummyFtpCommand::name();
let arg = Some("geda gedee geda geda ooo".to_string());

let command = command_parser
    .get(code)?
    .build(arg)?;

println!("{command}");  // Output : 'DUM geda gedee geda geda ooo'
```