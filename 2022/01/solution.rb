file = File.open('input.txt')
elves_data = file.read

elves_with_calories = elves_data.split("\n\n").map { |elf| elf.split("\n").map(&:to_i).sum }

puts 'the most performer:'
puts elves_with_calories.max

puts 'the Top 3 performers:'
puts elves_with_calories.max(3).sum
