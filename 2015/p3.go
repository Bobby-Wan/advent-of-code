package main

import(
  "fmt"
  "os"

)

type Coordinate struct {
  x int
  y int
}

func main() {
  santaX := 0
  santaY := 0

  robotX := 0
  robotY := 0

  visitedHouses := make(map[Coordinate]bool)
  visitedHouses[Coordinate{santaX, santaY}] = true

  f, err := os.Open("./input3")
  if err != nil {
    fmt.Println("Could not open input file")
    return
  }

  santasTurn := true
  currentX := &santaX
  currentY := &santaY

  for{
    var direction rune
    result, err := fmt.Fscanf(f, "%c", &direction)
    if (err != nil) || (result == 0) {
      fmt.Println("Unable to read character")
      break
    }

    if santasTurn {
      currentX = &santaX
      currentY = &santaY
    } else {
      currentX = &robotX
      currentY = &robotY
    }
    
    switch direction {
    case '>':
      *currentX++
    case '<':
      *currentX--
    case '^':
      *currentY++
    case 'v':
      *currentY--
    }

    visitedHouses[Coordinate{*currentX, *currentY}] = true

    santasTurn = !santasTurn
  }

  fmt.Printf("Visited houses: %d\n", len(visitedHouses))
}
