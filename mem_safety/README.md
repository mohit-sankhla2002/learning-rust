# Computer Resources: - 
1. Computation 
	- CPU
2. Memory
	- Persistent (Hard Drive or SSD)
	- Volatile (RAM)

*When we are talking about execution of a program, the persistent memory does not have a say in it. So let's talk about the volatile memory*


This is the diagram of a volatile memory. 
##### RAII (**R**esource **A**cquisition **I**s **I**nitialisation))
A technique / pattern / best-practise for exception safe resource management in C++
Problems: Managing memory manually is error prone and ownership is ambiguous 

```
	// memory leak 
	void memory_example () {
		Car * c = new Car();                 // Data Stored in Heap memory 
		function_that_can_throw_error;      // memory leak if exception is thrown
		if!should_continue()) return;      // memory leak if early return 
		delete car;                       //  clean up memory on the heap
	}

```

The solution to this problem was to use objects (constructors/destructors) *to manage resources*.

```
// For solving that problem, we create a class called CarManager 
class CarManager {
private: 
	Car *p; // points to a car 

public: 
	CarManager(Car *p) {
		this->p = p;
	}
	~CarManager() {
		// Clean up memory on the heap 
		
		delete p;
	}
}
```

**Another way to solve this problem is to use unique_ptr in C++**
`unique_ptr<Car> car= make_unique<Car>();`

#### Ownership Based Resource Management (OBRM): - 
Similar to RAll but instead of being a best-practice/pattern it's a built-in language feature.
Ownership rules are checked at compile time

- Each value in Rust has a variable that's called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.