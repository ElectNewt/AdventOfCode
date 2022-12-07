folders = Hash.new(0)

output = File.readlines('input.txt').map(&:split)

output.each_with_object([]) do |line, directory|
  case line
  in ['$', 'cd', '..']
    directory.pop
  in ['$', 'cd', folder]
    directory.push [directory.last, folder].compact.join(' ')
  in [size, file] if size.match?(/^\d+$/)
    directory.each { |i| folders[i] += size.to_i }
  else
  end
end


puts folders.values.reject { |i| i > 100_000 }.sum
puts folders.values.reject { |i| i < folders['/'] - 40_000_000 }.min
