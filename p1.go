package main

import(
  "fmt"
  "os"
  "bufio"
  "strconv"
)
func swapValues(a *int, b *int) {
    temp := *a
    *a = *b
    *b = temp
}

func sort3numbers(first *int, second *int, third *int) {
  if *second > *first {
    swapValues(first, second)
  }
  if *third > *first {
    swapValues(first, third)
  }
  if *third > *second {
    swapValues(second, third)
  }
}

func main() {
  f, err := os.Open("input1")
  if err != nil {
    fmt.Printf("Error when opening input file: %w", err)
  }
  defer f.Close()

  scanner := bufio.NewScanner(f)

  firstMax := 0
  secondMax := 0
  thirdMax := 0
  currentCalories := 0

  for scanner.Scan() {

    line := scanner.Text()
    if line == "" {
      if currentCalories > thirdMax {
        thirdMax = currentCalories
      } else if currentCalories > secondMax {
        secondMax = currentCalories
      } else if currentCalories > firstMax {
        firstMax = currentCalories
      }

      sort3numbers(&firstMax, &secondMax, &thirdMax)

      currentCalories = 0
      continue
    }

    calories, _ := strconv.Atoi(line)
    currentCalories = currentCalories + int(calories)
  }

  fmt.Println(firstMax + secondMax + thirdMax)
}
