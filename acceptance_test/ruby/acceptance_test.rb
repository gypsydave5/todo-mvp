#!/bin/env ruby

require 'faraday'
require 'faraday_middleware'
require 'nokogiri'
require 'nokogumbo'

$http = Faraday.new("http://localhost:3000") do |conn|
  conn.request  :url_encoded
  conn.use FaradayMiddleware::FollowRedirects, limit: 5
  conn.adapter Faraday.default_adapter
end

$item_name = 'item_name'

def test_the_starting_page_has_the_right_html
  expected = Nokogiri.HTML5(File.read("golden_master.html")).to_s
  actual = Nokogiri::HTML5($http.get('/').body).to_s
  if expected != actual
    puts "Starting page does not have the right HTML"
    exit 1
  end
end

def test_adding_a_todo
  response = Nokogiri::HTML5($http.post('/', item: $item_name).body)
  if response.css('.item-name').first.inner_html.strip != $item_name
    puts "Did not add a todo item"
    exit 1
  end
end

def test_completing_a_todo
  response = Nokogiri::HTML5($http.post('/done', item: $item_name).body)
  if response.css('s').first.inner_html.strip != $item_name
    puts "Did not add a todo item"
    exit 1
  end

end

def test_deleting_a_todo
  response = Nokogiri::HTML5($http.post('/delete', item: $item_name).body)
  if response.css('.item-name').length != 0
    puts "Did not delete a todo item"
    exit 1
  end
end

test_the_starting_page_has_the_right_html
test_adding_a_todo
test_completing_a_todo
test_deleting_a_todo
puts "Acceptance tests pass"
exit 0
