extern crate clap;
extern crate resolve_dns;

use clap::{ValueEnum, Parser};

fn default_address() -> String {
    "127.0.0.1".to_string()
}

#[derive(ValueEnum, Clone, Debug)]
enum ArgProtocol {
   Https,
   Tcp,
   Tls,
   Udp,
}

fn arg_protocol_as_protocol(protocol: &ArgProtocol) ->resolve_dns::Protocol {
    match protocol {
        ArgProtocol::Https => resolve_dns::Protocol::Https,
        ArgProtocol::Tcp => resolve_dns::Protocol::Tcp,
        ArgProtocol::Tls => resolve_dns::Protocol::Tls,
        ArgProtocol::Udp => resolve_dns::Protocol::Udp,
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Plain,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = default_address())]
    server: String,

    #[arg(short, long, default_value_t = 53)]
    port: u16,

    #[arg(long, value_enum, default_value_t = ArgProtocol::Udp)]
    protocol: ArgProtocol,

    #[arg(short, long, default_value_t = String::from(resolve_dns::DNS_STR_TYPE_A))]
    type_: String,

    #[arg(short, long, default_value_t = String::from(resolve_dns::DNS_STR_CLASS_IN))]
    class: String,

    #[arg(short, long, value_enum, default_value_t = OutputFormat::Plain)]
    output_format: OutputFormat,

    host: String,
}

fn main() {
    let args = Args::parse();

    let request = resolve_dns::Request{
        server: args.server,
        port: args.port,
        protocol: arg_protocol_as_protocol(&args.protocol),
        type_: resolve_dns::dns_type_to_u16(&args.type_),
        qname: args.host,
        class: args.class,
    };
    let result = resolve_dns::resolv(request);
    match result {
        Ok(message) => {
            match args.output_format {
                OutputFormat::Json => resolve_dns::print_json(&message),
                OutputFormat::Plain =>  resolve_dns::print_message(&message),
            }
        },
        Err(e) => println!("{e}")
        
    }
}
