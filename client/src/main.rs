extern crate clap;
extern crate resolve_dns;

use clap::Parser;

fn default_address() -> String {
    "127.0.0.1".to_string()
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = default_address())]
    server: String,

    #[arg(short, long, default_value_t = 53)]
    port: u16,

    #[arg(long, value_enum, default_value_t = resolve_dns::Protocol::Udp)]
    protocol: resolve_dns::Protocol,

    #[arg(short, long, default_value_t = String::from(resolve_dns::DNS_STR_TYPE_A))]
    type_: String,

    #[arg(short, long, default_value_t = String::from(resolve_dns::DNS_STR_CLASS_IN))]
    class: String,

    #[arg(short, long, value_enum, default_value_t = resolve_dns::OutputFormat::Plain)]
    output_format: resolve_dns::OutputFormat,

    host: String,
}

fn main() {
    let args = Args::parse();

    let request = resolve_dns::Request {
        server: args.server,
        port: args.port,
        protocol: args.protocol,
        type_: resolve_dns::dns_type_to_u16(&args.type_),
        qname: args.host,
        class: args.class,
    };
    let result = resolve_dns::resolv(request);
    match result {
        Ok(message) => message.print(&args.output_format),
        Err(e) => println!("{e}"),
    }
}
