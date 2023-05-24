extern crate clap;
extern crate resolve_dns;

use clap::{ValueEnum, Parser};
use resolve_dns::{resolv, Request};

fn default_address() -> String {
    "127.0.0.1".to_string()
}

#[derive(ValueEnum, Clone, Debug)]
enum Protocol {
   Https,
   Tcp,
   Tls,
   Udp,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Plain,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = default_address() )]
    server: String,

    #[arg(short, long, default_value_t = 53)]
    port: u16,

    #[arg(long, value_enum, default_value_t = Protocol::Udp)]
    protocol: Protocol,

    #[arg(short, long, default_value_t = String::from("A"))]
    type_: String,

    #[arg(short, long, default_value_t = String::from("IN"))]
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
        protocol: String::from("udp"),
        type_: resolve_dns::dns_type_to_u16(&args.type_),
        qname: args.host,
        class: args.class,
    };
    let result = resolve_dns::resolv(request);
    match result {
        Ok(r) => resolve_dns::print_message(r),
        Err(e) => println!("{}", e)
        
    }
}