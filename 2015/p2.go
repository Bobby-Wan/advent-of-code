package main

import (
  "fmt"
  "os"
  "sort"
)

func main() {
  fmt.Println("Advent of code")
  fmt.Println("--- Day 2: I Was Told There Would Be No Math ---")

  var w int
  var h int
  var l int

  f, err := os.Open("./input2")
  if err != nil {
    fmt.Println("Error opening input file")
    return
  }
  
  totalSquareFeetOfPaper := 0
  ribbonNeeded := 0

  for {
    ret, err := fmt.Fscanf(f, "%dx%dx%d\n", &l, &w, &h)

    if (err != nil) || (ret == 0) {
      fmt.Printf("Stopped reading data. Error: %s\n", err)
      break
    }
    
    values := []int{w, h, l}
    sort.Slice(values, func(i, j int) bool { return values[i] < values[j] })
    extraPaperSqFt := values[0] * values[1]
    var paperToAdd int 
    paperToAdd = 2*w*h + 2*w*l + 2*h*l

    totalSquareFeetOfPaper += extraPaperSqFt + paperToAdd

    p1 := 2*w + 2*h
    p2 := 2*h + 2*l
    p3 := 2*l + 2*w

    perimeters := []int{p1, p2, p3}
    sort.Slice(perimeters, func(i, j int) bool {return perimeters[i] < perimeters[j]} )
    
    ribbonNeeded += perimeters[0]
    ribbonNeeded += w*h*l
  }

  fmt.Printf("Total paper needed: %d\n", totalSquareFeetOfPaper)

  fmt.Printf("Ribbon needed: %d\n", ribbonNeeded)
}
