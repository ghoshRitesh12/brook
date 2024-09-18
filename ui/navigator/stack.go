package navigator

// Node is an element in the linked list
type Node[T any] struct {
	data T
	next *Node[T]
}

// [NewNode] returns a new node instance of type [T]
func NewNode[T any](data T, next *Node[T]) *Node[T] {
	return &Node[T]{
		data: data,
		next: next,
	}
}

// Linked list based stack for navigation purposes
type Stack[T any] struct {
	length int
	top    *Node[T]
}

// [NewStack] returns a new stack instance of type [T]
func NewStack[T any]() *Stack[T] {
	return &Stack[T]{
		top: NewNode(*new(T), nil),
	}
}

// [NewStackWithData] takes in the first element in the stack returns a new stack instance of type [T]
func NewStackWithData[T any](data T) *Stack[T] {
	return &Stack[T]{
		length: 1,
		top:    NewNode(data, nil),
	}
}

// [Len] returns the length of the stack
func (s *Stack[T]) Len() int {
	return s.length
}

// [Push] adds an element to the top of the stack
func (s *Stack[T]) Push(element T) {
	newElement := NewNode(element, s.top)
	s.top = newElement

	s.length++
}

// [Pop] removes the top element from the stack and a boolean indicating if the stack is empty
func (s *Stack[T]) Pop() (T, bool) {
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
func (s *Stack[T]) Peek() (T, bool) {
	if s.length == 0 {
		return *new(T), false
	}

	return s.top.data, true
}
