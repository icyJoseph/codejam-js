// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4621b
import Foundation

let T = Int(readLine()!)!

for ti in 1 ... T {
    let line = readLine()!.components(separatedBy: " ")

    let R = Int(line[0])!
    let C = Int(line[1])!

    let width = 2 * C + 1
    let height = 2 * R + 1

    let dottedRow = Array(repeating: ".", count: width)

    var grid = Array(repeating: dottedRow, count: height)

    for y in 0 ..< height {
        for x in 0 ..< width {
            if y % 2 == 0 {
                grid[y][x] = x % 2 == 0 ? "+" : "-"
            } else {
                if x % 2 == 0 {
                    grid[y][x] = "|"
                }
            }
        }
    }

    grid[0][0] = "."
    grid[1][0] = "."
    grid[0][1] = "."

    print("Case #\(ti):")

    for row in grid {
        print(row.joined(separator: ""))
    }
}
