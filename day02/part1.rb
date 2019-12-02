numbers = $<.first.split(',').map(&:to_i)
numbers[1] = 12
numbers[2] = 2

numbers.each_slice(4) do |x|
  a, b, c, d = x
  case a
  when 1
    numbers[d] = numbers[b] + numbers[c]
  when 2
    numbers[d] = numbers[b] * numbers[c]
  when 99
    break
  end
end

puts numbers.first
