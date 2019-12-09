def distance(a, b)
  a.abs + b.abs
end

wire1, wire2 = $<.map { |l| l.split(',') }

cross_points = []
board = Hash.new(0)
max_x, max_y, min_x, min_y = 0, 0, 0, 0
x, y = 0, 0

wire1.each do |mv|
  direction, magnitude = mv[0], mv[1..].to_i

  case direction
  when 'R'
    magnitude.times do
      x += 1
      board[[x, y]] = 1
    end
    max_x = [max_x, x].max
  when 'D'
    magnitude.times do
      y -= 1
      board[[x, y]] = 1
    end
    min_y = [min_y, y].min
  when 'L'
    magnitude.times do
      x -= 1
      board[[x, y]] = 1
    end
    min_x = [min_x, x].min
  when 'U'
    magnitude.times do
      y += 1
      board[[x, y]] = 1
    end
    max_y = [max_y, y].max
  end
end

x, y = 0, 0
wire2.each do |mv|
  direction, magnitude = mv[0], mv[1..].to_i

  case direction
  when 'R'
    magnitude.times do
      x += 1
      board[[x, y]] += 1
    end
    max_x = [max_x, x].max
  when 'D'
    magnitude.times do
      y -= 1
      board[[x, y]] += 1
    end
    min_y = [min_y, y].min
  when 'L'
    magnitude.times do
      x -= 1
      board[[x, y]] += 1
    end
    min_x = [min_x, x].min
  when 'U'
    magnitude.times do
      y += 1
      board[[x, y]] += 1
    end
    max_y = [max_y, y].max
  end
end

board.each do |key, value|
  cross_points << key if value >= 2
end

puts cross_points.map { |arr| distance(*arr) }.sort
