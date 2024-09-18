package navigator

type Navigator[T any] interface {
	Len() int
	Peek() T
	Pop() T
	Push(element T)
}
