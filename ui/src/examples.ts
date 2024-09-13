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
};
