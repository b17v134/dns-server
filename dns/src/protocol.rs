use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Protocol {
    Https,
    Tcp,
    Tls,
    Udp,
}
