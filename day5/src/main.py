def get_input(filename):
    with open(filename, 'r') as f:
        text_lines = f.readlines()
        out = [[tuple(int(s) for s in p.split(','))
               for p in text_line.split(" -> ")]
               for text_line in text_lines]

    return out


def is_horizontal(line):
    if line[0][1] == line[1][1]:
        return True
    return False


def get_horizontal(lines):
    out = []
    for line in lines:
        if is_horizontal(line):
            out.append(line)
    return out

    def print_board(board):
        for l in board:
            print(l)


def part1(lines):
    board = [[0]*10]*10
    print("Initial board: ")
    print_board(board)

    for line in lines:
        if is_horizontal(line):
            if line[0][0] > line[1][0]:
                startpoint = line[1][0]
                endpoint = line[0][0]
            else:
                startpoint = line[0][0]
                endpoint = line[1][0]
            for i in range(startpoint, endpoint + 1):
                board[i][line[0][1]] += 1

    print_board(board)


if __name__ == '__main__':
    filename = "day5_sample.in"
    res = get_input(filename)
    horizontal = get_horizontal(res)
    print(horizontal)
