package ui_test

import (
	"fmt"
	"testing"

	"github.com/davecgh/go-spew/spew"
	"github.com/ghoshRitesh12/brook/ui"
)

// go test -run TestStack ./ui -v -count=1
func TestStack(t *testing.T) {
	stack := ui.NewStack[string]()

	stack.Push("hi")
	stack.Push("this ")
	stack.Push("is_a")
	stack.Push("stack_implemented_using_linked_list")
	stack.Push("in_golang")

	spew.Dump(stack)

	fmt.Println("length:", stack.Len())

	topData, _ := stack.Peek()
	fmt.Println("peek:", topData)

	poppedValue, _ := stack.Pop()
	fmt.Println("pop:", poppedValue)

	spew.Dump(stack)

	fmt.Println("length:", stack.Len())

	topData, _ = stack.Peek()
	fmt.Println("peek:", topData)
}
