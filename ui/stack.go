package ui

type node[T any] struct {
	data T
	next *node[T]
}

// Linked list based stack for navigation purposes
type stack[T any] struct {
	length int
	head   *node[T]
	tail   *node[T]
}

// [NewStack] returns a new stack instance of type [T]
func NewStack[T any]() *stack[T] {
	return &stack[T]{
		head: &node[T]{},
		tail: &node[T]{},
	}
}

// [NewStackWithValue] takes in the first element in the stack returns a new stack instance of type [T]
func NewStackWithValue[T any](value T) *stack[T] {
	return &stack[T]{
		length: 1,
		head: &node[T]{
			data: value,
		},
		tail: &node[T]{
			data: value,
		},
	}
}

func (s *stack[T]) Len() int {
	return s.length
}

func (s *stack[T]) Push(element T) {
	newElement := &node[T]{data: element}
	defer func() {
		s.length += 1
	}()

	if s.length > 0 {
		s.tail.next = newElement
		s.tail = newElement
		return
	}

	s.head = newElement
	s.tail = newElement
}

func (s *stack[T]) Pop() T {
	defer func() {
		s.length -= 1
	}()

	currentHead := s.head
	s.head = s.head.next

	value := currentHead.data
	currentHead = nil

	return value
}

func (s *stack[T]) Peek() T {
	return s.head.data
}
