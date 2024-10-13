## Cranberry Syntax Example

`.crb` is the extension you didn't know you needed until now.

### Hello, World! (Cranberry style):

```crb
print("Hello, world!"); // Not copied from Python.
```

**Note**: Semi-colons are not just a suggestion—they're a lifestyle.

### Declaring Variables

In Cranberry, you have the luxury of choosing between the unbreakable and the, well... breakable.

- **Immutable** (for those who like commitment):

```crb
const var = 1;
```

- **Mutable** (for when you just can’t make up your mind):

```crb
mutate var = 1;
var = 2;
```

### A Simple Calculator:

Because why not? Cranberry makes building calculators as easy as pie... or should we say, as easy as **fruit** pie.

```crb
const n1 = input("Enter the first number: ");
const op = input("Enter an operator (+, -, *, /): ");
const n2 = input("Enter the second number: ");

// Functions to perform operations, because Cranberry believes in delegation.

fn add(a, b) {
    print(a + b);
}

fn sub(a, b) {
    print(a - b);
}

fn mul(a, b) {
    print(a * b);
}

fn div(a, b) {
    // Cranberry has your back with division by zero—it just returns the numerator. But, because we care, here's a classic division-by-zero handler.

    match b {
        0 => print("Cannot divide by zero"),
        default => print(a / b),
    };

    // You could also be a rebel and let Cranberry do its magic:
    /**
        print(a / b); // If a is 10 and b is 0, Cranberry says, "10 it is!"
    **/
}
```

### Handling Operations: Because Your Time is Precious

Cranberry's `whether...otherwise` makes decision-making smooth and guilt-free:

```crb
whether(op == "+") {
    add(n1, n2);
} otherwise whether(op == "-") {
    sub(n1, n2);
} otherwise whether(op == "*") {
    mul(n1, n2);
} otherwise whether(op == "/") {
    div(n1, n2);
} otherwise {
    print("Invalid operator");
}
```

### Advanced Math? Cranberry’s Got You Covered!

For when things get complicated (like explaining programming to non-programmers):

```crb
const problem = input("Enter your problem: "); // Yes, Cranberry solves your problems, even complex ones.

print(Math.calculate(problem));
```
