export const codeExamples: Record<string, string> = {
  "Hello, World!": `
let message = "Hello, World!";
message;
  `,
  Fibonacci: `
let fib = fn(n) {
  if (n < 2) {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
};

fib(10);
  `,
  Factorial: `
let factorial = fn(n) {
  if (n == 0) {
    return 1;
  }
  return n * factorial(n - 1);
};

factorial(10);
  `,
  "Simple Addition": `
let add = fn(a, b) {
  return a + b;
};

add(2, 3);
  `,
  "Array Indexing": `
let myArray = [1, 2, 3];
myArray[1 * 3 - 2];
  `,
  Map: `
let map = fn(arr, f) {
  let iter = fn(arr, accumulated) {
    if (len(arr) == 0) {
      accumulated
    } else {
      iter(rest(arr), push(accumulated, f(first(arr))));
    }
  };
  iter(arr, []);
};

let data = [1, 2, 3];
let squared = fn(x) { x * x };

map(data, squared);`,
  Fold: `
let fold = fn(arr, initial, f) {
  let iter = fn(arr, accumulated) {
    if (len(arr) == 0) {
      accumulated
    } else {
      iter(rest(arr), f(accumulated, first(arr)));
    }
  };
  iter(arr, initial);
};

let sum = fn(arr) {
    fold(arr, 0, fn(initial, element) { initial + element });
};

sum([1, 2, 3, 4, 5]);`,
  "Hash Maps": `
let myHash = {
  "one": 1,
  two: 2,
  true: 3,
  12: 4,
  false: 5
};

myHash["one"];
  `,
};
