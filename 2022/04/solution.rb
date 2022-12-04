# frozen_string_literal: true
pairs = File.read('input.txt').split("\n")

def get_all_sections_elf(sections)
  start_end = sections.split('-').map(&:to_i)
  (start_end[0] .. start_end[1]).to_a
end

def elves_sections(pair)
  elves_sections = pair.split(',')
  [get_all_sections_elf(elves_sections[0]), get_all_sections_elf(elves_sections[1])]
end

part1_count = 0
pairs.each do |pair|
  sections_elf1, sections_elf2 = elves_sections(pair)
  if (sections_elf1 - sections_elf2).empty? || (sections_elf2 - sections_elf1).empty?
    part1_count += 1
  end
end


part2_count = 0
pairs.each do |pair|
  sections_elf1, sections_elf2 = elves_sections(pair)
  if sections_elf1.intersection(sections_elf2).length > 0
    part2_count += 1
  end
end



puts "Part 1 - total points #{part1_count}"

puts "Part 2 - total points #{part2_count}"
