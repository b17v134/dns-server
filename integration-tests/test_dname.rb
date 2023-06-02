require 'json'
require 'open3'
require "test/unit"

class TestDname < Test::Unit::TestCase
    def test_dname
        stdout, stderr, status = Open3.capture3('./target/release/resolv -s 10.23.0.7 test.example.test-bind -t dname -o json')
 
        assert_equal(0, status)
        assert_equal("", stderr)
        data = JSON.parse(stdout)
        assert_equal([{
            'name'=> 'test.example.test-bind.', 
            'ttl'=> 300, 
            'class'=> 'IN', 
            'type'=> 'DNAME', 
            'rdata'=> 'foo.example.test-bind.'
        }], data['answers'])
    end
end