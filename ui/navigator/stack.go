package navigator

// Node is an element in the [Stack]
type Node[T any] struct {
	Data T
	Next *Node[T]
}

// [NewNode] returns a new [Node] instance of type [T]
func NewNode[T any](data T, next *Node[T]) *Node[T] {
	return &Node[T]{
		Data: data,
		Next: next,
	}
}

// Linked list based stack for navigation purposes
type Stack[T any] struct {
	Length int
	Top    *Node[T]
}

// [NewStack] returns a new [Stack] instance of type [T]
func NewStack[T any]() *Stack[T] {
	return &Stack[T]{
		Top: NewNode(*new(T), nil),
	}
}

// [NewStackWithData] takes in the first element in the [Stack] & returns a new [Stack] instance of type [T]
func NewStackWithData[T any](data T) *Stack[T] {
	return &Stack[T]{
		Length: 1,
		Top:    NewNode(data, nil),
	}
}

// [Len] returns the length of the [Stack]
func (s *Stack[T]) Len() int {
	return s.Length
}

// [Push] adds an element to the top of the [Stack]
func (s *Stack[T]) Push(element T) {
	newElement := NewNode(element, s.Top)
	s.Top = newElement

	s.Length++
}

// [Pop] removes the top element from the [Stack] and a boolean indicating if the [Stack] is empty
func (s *Stack[T]) Pop() (T, bool) {
	if s.Length == 0 {
		return *new(T), false
	}

	prevTop := s.Top
	value := prevTop.Data

	s.Top = prevTop.Next
	prevTop = nil

	s.Length--
	return value, true
}

// [Peek] returns the top element of the [Stack] and a boolean indicating if the [Stack] is empty
func (s *Stack[T]) Peek() (T, bool) {
	if s.Length == 0 {
		return *new(T), false
	}

	return s.Top.Data, true
}
