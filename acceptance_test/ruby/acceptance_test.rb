#!/usr/bin/env ruby

require 'faraday'
require 'faraday_middleware'
require 'nokogumbo'
require 'minitest/autorun'


class TodoMVPAcceptance < Minitest::Test
  i_suck_and_my_tests_are_order_dependent!

  def self.random_string
    (0...8).map { (65 + rand(26)).chr }.join
  end

  @@todo_name = random_string

  @@http = Faraday.new("http://localhost:3000") do |conn|
    conn.request  :url_encoded
    conn.use FaradayMiddleware::FollowRedirects, limit: 5
    conn.adapter Faraday.default_adapter
  end

  def test_the_starting_page_has_the_right_html
    assert_equal page.to_s, golden_master.to_s
  end

  def test_adding_a_todo
    add_todo
    assert todo_on_page?
  end

  def test_completing_a_todo
    complete_todo
    assert todo_complete?
  end

  def test_deleting_a_todo
    delete_todo
    refute todo_on_page?
  end

  private

  def find_id_for_todo_in page
    find_todo_in(page)
      .css('form[action="/delete"] input[name="item"]')
      .attr('value')
  end

  def find_todo_in page
    page.css('.todo')
      .find { |node| node.inner_text.include? @@todo_name }
  end

  def complete_todo
    id = find_id_for_todo_in page
    @@http.post('/done', item: id).body
  end

  def add_todo
    @@http.post('/', item: @@todo_name).body
  end

  def page
    Nokogiri::HTML5(@@http.get('/').body)
  end

  def golden_master
    Nokogiri.HTML5(File.read("golden_master.html"))
  end

  def delete_todo
    id = find_id_for_todo_in page
    @@http.post('/delete', item: id).body
  end

  def todo_complete?
    todo = find_todo_in page
    todo.css('s') != nil
  end

  def todo_on_page?
    find_todo_in(page) != nil
  end

end
