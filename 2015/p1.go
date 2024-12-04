package main

import "fmt"
import "bufio"
import "os"

func countFloors(input string) int {
  floor := 0
  for i := 0; i < len(input); i++ {
    if input[i] == '(' {
      floor++
    } else if input[i] == ')' {
      floor--
    }

    if floor == -1 {
      fmt.Printf("Santa entered basement at position %d\n", i+1)
    }
  }

  return floor
}

func main() {
  f, err := os.Open("inputp1")
  if err != nil {
    fmt.Errorf("Error when opening input file: %w", err)
  }

  reader := bufio.NewReader(f)
  input, err := reader.ReadString('\n')
  if err != nil {
    fmt.Errorf("Error when reading input: %w", err)
  } 
  fmt.Printf("Input size: %d\n", len(input))
  fmt.Println(input)

  result := countFloors(input)
  fmt.Println(result)
}
