# frozen_string_literal: true

stream_buffer = File.read('input.txt')

def calculate_stream(stream_buffer, chunk_size)
  stream_buffer.chars.each_cons(chunk_size).with_index
               .filter { |buffer, _| buffer.uniq.length == buffer.length }
               .first
               .then { |_, i| i + chunk_size }
end

puts "Part 1 - result #{calculate_stream(stream_buffer, 4)}"
puts "Part 2 - result #{calculate_stream(stream_buffer, 14)}"

