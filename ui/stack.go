package ui

// Node is an element in the linked list
type node[T any] struct {
	data T
	next *node[T]
}

// [NewNode] returns a new node instance of type [T]
func NewNode[T any](value T, nextPointer *node[T]) *node[T] {
	return &node[T]{
		data: value,
		next: nextPointer,
	}
}

// Linked list based stack for navigation purposes
type stack[T any] struct {
	length int
	top    *node[T]
}

// [NewStack] returns a new stack instance of type [T]
func NewStack[T any]() *stack[T] {
	emptyValue := new(T)
	return &stack[T]{
		top: NewNode(*emptyValue, nil),
	}
}

// [NewStackWithData] takes in the first element in the stack returns a new stack instance of type [T]
func NewStackWithData[T any](data T) *stack[T] {
	return &stack[T]{
		length: 1,
		top:    NewNode(data, nil),
	}
}

// [Len] returns the length of the stack
func (s *stack[T]) Len() int {
	return s.length
}

// [Push] adds an element to the top of the stack
func (s *stack[T]) Push(element T) {
	newElement := NewNode(element, s.top)
	s.top = newElement

	s.length++
}

// [Pop] removes the top element from the stack and a boolean indicating if the stack is empty
func (s *stack[T]) Pop() (T, bool) {
	if s.length == 0 {
		return *new(T), false
	}

	prevTop := s.top
	value := prevTop.data

	s.top = prevTop.next
	prevTop = nil

	s.length--
	return value, true
}

// [Peek] returns the top element of the stack and a boolean indicating if the stack is empty
func (s *stack[T]) Peek() (T, bool) {
	if s.length == 0 {
		return *new(T), false
	}

	return s.top.data, true
}
