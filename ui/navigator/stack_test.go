package navigator_test

import (
	"fmt"
	"testing"

	"github.com/davecgh/go-spew/spew"
	"github.com/ghoshRitesh12/brook/ui/navigator"
)

// go test -run TestStack ./ui/navigator -v -count=1
func TestStack(t *testing.T) {
	stack := navigator.NewStack[string]()

	stack.Push("hi") // this will be the second last element, as it's a stack
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
