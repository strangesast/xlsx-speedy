package main

import (
	"fmt"
	"os"
	"time"

	"github.com/360EntSecGroup-Skylar/excelize/v2"
)

func fn(filepath string, sheetName string) {
	f, err := excelize.OpenFile(filepath)
	if err != nil {
		panic(err)
	}

	rows, err := f.GetRows(sheetName)
	if err != nil {
		panic(err)
	}

	for _, row := range rows {
		for range row {
		}
	}
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
