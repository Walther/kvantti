# kvantti
A quantum computer implementation

## Intro

Number: The simplest unit. A single value denoting a single point on a continuous line of one-dimensional values.

Imaginary number: a conceptual number often denoted with $i$ or $j$, with the unit value defined as $\sqrt{-1}$. Imaginary numbers can be varying scalar multiples of the unit value: you can have $2i$, or $0.3i$, and so on.

Vector: a single value, whose definition consists of multiple singular values. A two-dimensional vector consists of two one-dimensional values. If we assume the dimensions are perpendicular, a two-dimensional vector can denote a single point on a two-dimensional plane.

Complex number: a two-dimensional vector, in which one singular value is a number and the other is an imaginary number.

Ket: a two-dimensional vector, where both values are complex numbers. Each of these values are also known as amplitudes.

Ket zero: a ket with the value $[1, 0]$. Analogous to the classical bit $0$. Has the symbol $|0>$.

Ket one: a ket with the value $[0, 1]$. Analogous to the classical bit $1$. Has the  symbol $|1>$.

Quantum state: a fundamental unit in quantum computation, also known as the qubit. Analogous to the bit in classical computing. The value can be represented as a ket, or alternatively, as a sum of scalar multiples of ket zero and ket one. $0.6 |0> + 0.8|1>$. A valid quantum state has extra constraint: sum of the squares of the amplitudes must be one. Not all kets are valid quantum states.

