package main

import "fmt"

type Fib struct {
	a, b int
}

func (f *Fib) Next() int {
	a, b := f.a, f.b
	f.a = b
	f.b = b + a
	return a
}

func foo(bar string) (baz int) {
	baz = len(bar)
	return
}

func main() {
	fib := Fib{0x0, 1}
	for i := 0; i < 10; i++ {
		fmt.Println(fib.Next())
	}
}
