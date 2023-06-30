require 'json'
require 'open3'
require "test/unit"

class TestAxfr < Test::Unit::TestCase
    def test_axfr
        stdout, stderr, status = Open3.capture3('./target/release/resolv -s 10.23.0.7 example.test-bind -t axfr --protocol tcp -o json')
 
        assert_equal(0, status)
        assert_equal("", stderr)
        data = JSON.parse(stdout)
        
        answers = data['answers']
        answers.sort_by! { |value| value['rdata']}
        assert_equal([
            {"name"=>"example.test-bind.","type"=>"HINFO", "class"=>"IN", "ttl"=>300,"rdata"=>"\"i686\" \"unix\""}, 
            {"name"=>"example.test-bind.", "type"=>"MX", "class"=>"IN", "ttl"=>300, "rdata"=>"1 mail1.example.test-bind."}, 
            {"name"=>"example.test-bind.", "type"=>"MX", "class"=>"IN", "ttl"=>300, "rdata"=>"10 mail10.example.test-bind."}, 
            {"name"=>"example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.7"}, 
            {"name"=>"mail1.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.7"}, 
            {"name"=>"ns.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.7"}, 
            {"name"=>"a.foo.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.8"}, 
            {"name"=>"mail5.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.8"}, 
            {"name"=>"b.foo.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.9"}, 
            {"name"=>"mail10.example.test-bind.", "type"=>"A", "class"=>"IN", "ttl"=>300, "rdata"=>"10.23.0.9"}, 
            {"name"=>"example.test-bind.", "type"=>"MX", "class"=>"IN", "ttl"=>300, "rdata"=>"5 mail5.example.test-bind."}, 
            {"name"=>"example.test-bind.", "type"=>"AAAA", "class"=>"IN", "ttl"=>300, "rdata"=>"fde3:b2a:7ba2:f133:e679:29a8:3e53:f438"}, 
            {"name"=>"example.test-bind.", "type"=>"AAAA", "class"=>"IN", "ttl"=>300, "rdata"=>"fde3:b2a:7ba2:f133:e679:29a8:3e53:f439"}, 
            {"name"=>"example.test-bind.", "type"=>"AAAA", "class"=>"IN", "ttl"=>300, "rdata"=>"fde3:b2a:7ba2:f133:e679:29a8:3e53:f43a"}, 
            {"name"=>"example.test-bind.", "type"=>"AAAA", "class"=>"IN", "ttl"=>300, "rdata"=>"fde3:b2a:7ba2:f133:e679:29a8:3e53:f43b"}, 
            {"name"=>"test.example.test-bind.", "type"=>"DNAME", "class"=>"IN", "ttl"=>300, "rdata"=>"foo.example.test-bind."}, 
            {"name"=>"example.test-bind.", "type"=>"NS", "class"=>"IN", "ttl"=>300, "rdata"=>"ns.example.test-bind."}, 
            {"name"=>"example.test-bind.", "type"=>"SOA", "class"=>"IN", "ttl"=>300, "rdata"=>"ns.example.test-bind. mail.example.test-bind. 20230409 604800 86400 86400 86400"}, 
            {"name"=>"example.test-bind.", "type"=>"SOA", "class"=>"IN", "ttl"=>300, "rdata"=>"ns.example.test-bind. mail.example.test-bind. 20230409 604800 86400 86400 86400"}
        ], answers)
    end
end