package main

import (
	"errors"
	"fmt"
	"os"
	"time"

	"github.com/tealeg/xlsx/v3"
)

func cellVisitor(c *xlsx.Cell) error {
	return nil
}

func rowVisitor(r *xlsx.Row) error {
	return r.ForEachCell(cellVisitor)
}

func fn(filepath string, sheetName string) {
	wb, err := xlsx.OpenFile(filepath)
	if err != nil {
		panic(err)
	}

	sh, ok := wb.Sheet[sheetName]
	if !ok {
		panic(errors.New("Sheet not found"))
	}

	sh.ForEachRow(rowVisitor)
}

func test() {
	const sheetName = "PRODUCTION SCHEDULE"
	filepath := os.Getenv("FILEPATH")
	if filepath == "" {
		filepath = "../schedule.xlsm"
	}

	fn(filepath, sheetName)
}

type afunc func()

func timeit(f afunc) float64 {
	start := time.Now()
	f()
	duration := time.Since(start)
	return float64(duration/time.Nanosecond) / 1e9
}

func main() {
	fmt.Println(timeit(test))
}
