# frozen_string_literal: true

parts = File.read('input.txt').split("\n\n")

# holds the information of the array in the input.txt
class OriginalStacks
  attr_reader :content

  def initialize(content)
    @content = content.map(&:dup)
  end
end

def build_stacks_as_array(stacks_information)
  result = stacks_information.last.split(' ').map(&:to_i).last
  Array.new(result) { |_| [] }
end

def build_row_as_content(row_information)
  row_information.chars.each_slice(4).map(&:join).map(&:strip).to_a
end

stacks_information = parts[0].split("\n")

## Stack calculations
initial_stacks = build_stacks_as_array(stacks_information)

stacks_information.each do |row|
  next if row.equal?(stacks_information.last)

  row_content = build_row_as_content(row)

  row_content.each_with_index do |value, index|
    next if value.nil? || value == ''

    initial_stacks[index].unshift(value)
  end
end

part1_stacks = OriginalStacks.new(initial_stacks)

part2_stacks = OriginalStacks.new(initial_stacks)

### Instructions

movement_instructions = parts[1].split("\n")

def get_row_instruction(instruction)
  instruction.slice!('move')
  instruction.slice!('from')
  instruction.slice!('to')

  instruction.split(' ').map(&:to_i)
end

def part1_movements(stacks, movement_instructions)
  movement_instructions.each do |instruction|
    amount, from, to = get_row_instruction(instruction)
    amount.times do
      crate = stacks[from - 1].pop(1)
      stacks[to - 1].concat(crate)
    end
  end
end

def part2_movements(stacks, movement_instructions)
  movement_instructions.each do |instruction|
    amount, from, to = get_row_instruction(instruction)
    crate = stacks[from - 1].pop(amount)
    stacks[to - 1].concat(crate)
  end
end

## Calculations

def crate_cleanup(crate)
  crate.slice!('[')
  crate.slice!(']')
  crate
end

def whats_on_top(stacks)
  value = ''

  stacks.each do |stack|
    value += crate_cleanup(stack.last)
  end
  value
end

def part1_result(stacks, movement_instructions)
  part1_movements(stacks, movement_instructions)
  whats_on_top(stacks)
end

def part2_result(stacks, movement_instructions)
  part2_movements(stacks, movement_instructions)
  whats_on_top(stacks)
end



puts "Part 1 - result #{part1_result(part1_stacks.content, movement_instructions)}"
puts "Part 2 - result #{part2_result(part2_stacks.content, movement_instructions)}"
