require 'open3'
require "test/unit"

class TestHelp < Test::Unit::TestCase
    def test_help
        usage_string =  <<-TEXT
Usage: resolv [OPTIONS] <HOST>

Arguments:
  <HOST>  

Options:
  -s, --server <SERVER>                [default: 127.0.0.1]
  -p, --port <PORT>                    [default: 53]
      --protocol <PROTOCOL>            [default: udp] [possible values: https, tcp, tls, udp]
  -t, --type <TYPE>                    [default: A]
  -c, --class <CLASS>                  [default: IN]
  -o, --output-format <OUTPUT_FORMAT>  [default: plain] [possible values: json, plain, yaml]
  -h, --help                           Print help
  -V, --version                        Print version
TEXT
        for help_param in ['-h', '--help']
            stdout, stderr, status = Open3.capture3("./target/release/resolv " + help_param)
 
            assert_equal(0, status)
            assert_equal("", stderr)
            assert_equal(usage_string, stdout)
        end
    end
end