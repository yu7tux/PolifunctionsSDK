# PolifunctionsSDK Integration Guide

This guide provides instructions on how to integrate the PolifunctionsSDK into your projects across different programming languages and environments.

## Table of Contents

- [Rust Integration](#rust-integration)
- [C++ Integration](#c-integration)
- [Python Integration](#python-integration)
- [.NET Integration](#net-integration)
- [Language-Specific Considerations](#language-specific-considerations)
- [Performance Optimization](#performance-optimization)
- [Troubleshooting](#troubleshooting)

## Rust Integration

### Adding the Dependency

To use PolifunctionsSDK in a Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
polifunctions-sdk = "0.1.0"

Basic Usage

use polifunctions_sdk::core::{
    Domain, Codomain, PolifunctionBase, SetValuedPolifunction,
    lift_to_set, UnionPolifunction
};

// Define a simple domain and codomain
struct RealNumbers;
impl Domain for RealNumbers {
    type Element = f64;
    fn contains(&self, _: &Self::Element) -> bool { true }
}

struct PositiveReals;
impl Codomain for PositiveReals {
    type Element = f64;
    fn contains(&self, x: &Self::Element) -> bool { *x >= 0.0 }
}

fn main() {
    // Create a simple polifunction that returns the square and square root of a number
    let domain = RealNumbers;
    let codomain = PositiveReals;
    
    // Function that returns the square
    let square_fn = lift_to_set(
        |x: &f64| Ok(*x * *x),
        domain,
        codomain
    );
    
    // Function that returns the absolute value and then square root
    let sqrt_fn = lift_to_set(
        |x: &f64| Ok(x.abs().sqrt()),
        domain,
        codomain
    );
    
    // Create a union of these functions
    let combined = UnionPolifunction::new(square_fn, sqrt_fn);
    
    // Evaluate the combined function
    let result = combined.value_set(&2.0).unwrap();
    println!("f(2.0) = {:?}", result); // Will contain both 4.0 and sqrt(2)
}

Advanced Usage
For more advanced usage, including quantum computing applications, see the Rust API Examples.
C++ Integration
Requirements

A C++17 compatible compiler
CMake 3.15 or higher

Building and Installing

git clone https://github.com/yourusername/PolifunctionsSDK.git
cd PolifunctionsSDK
mkdir build && cd build
cmake ..
make
make install

Including in Your Project

#include <polifunctions/core.hpp>
#include <polifunctions/scientific.hpp>

int main() {
    // Create a simple interval-valued polifunction
    auto domain = polifunctions::RealDomain();
    auto codomain = polifunctions::RealDomain();
    
    // Function that returns an interval [x-1, x+1]
    auto interval_fn = polifunctions::make_interval_function(
        [](double x) {
            return polifunctions::Interval<double>{x-1, x+1, true, true};
        },
        domain, codomain
    );
    
    // Evaluate the function
    auto result = interval_fn.value_interval(3.0);
    std::cout << "Interval: [" << result.lower << ", " << result.upper << "]" << std::endl;
    // Output: Interval: [2, 4]
    
    return 0;
}

Linking
When building your C++ project, link against the PolifunctionsSDK:

g++ -std=c++17 main.cpp -o my_program -lpolifunctions

Python Integration
Installation

pip install polifunctions-sdk

import polifunctions as pf

# Define a simple set-valued polifunction
def square_and_sqrt(x):
    return {x**2, abs(x)**0.5}

# Create a polifunction from this function
real_domain = pf.RealDomain()
positive_domain = pf.PositiveDomain()
poly_func = pf.SetValuedPolifunction(square_and_sqrt, real_domain, positive_domain)

# Evaluate the function
result = poly_func.value_set(2.0)
print(f"f(2.0) = {result}")  # Will output {4.0, 1.414...}

Integration with NumPy and Scientific Libraries

import numpy as np
import polifunctions as pf

# Create an interval-valued polifunction from a numpy function
def uncertain_measurement(x):
    return pf.Interval(x - 0.1, x + 0.1)

func = pf.IntervalValuedPolifunction(uncertain_measurement, pf.RealDomain(), pf.RealDomain())

# Create a batch of inputs
inputs = np.linspace(0, 10, 100)

# Get the intervals for all inputs
intervals = [func.value_interval(x) for x in inputs]

# Extract lower and upper bounds for plotting
lower_bounds = [interval.lower for interval in intervals]
upper_bounds = [interval.upper for interval in intervals]

# Plot with matplotlib (not shown)

.NET Integration
NuGet Package

Install-Package PolifunctionsSDK

using PolifunctionsSDK;
using PolifunctionsSDK.Core;

// Define a simple set-valued polifunction
var domain = new RealDomain();
var codomain = new RealDomain();

// Function that returns both positive and negative square roots
var sqrtFunc = new SetValuedPoliFunctionBuilder<double, double>()
    .WithMapping(x => {
        var result = new HashSet<double>();
        if (x >= 0) {
            result.Add(Math.Sqrt(x));
            result.Add(-Math.Sqrt(x));
        }
        return result;
    })
    .WithDomain(domain)
    .WithCodomain(codomain)
    .Build();

// Evaluate the function
var result = sqrtFunc.ValueSet(4.0);
Console.WriteLine($"f(4.0) = {{{string.Join(", ", result)}}}");  // Outputs {2, -2}

Language-Specific Considerations
Memory Management
The SDK handles memory management differently across languages:

Rust: Uses Rust's ownership system for zero-cost abstractions
C++: Follows RAII principles with smart pointers
Python: Automatic garbage collection with reference counting
.NET: Managed by the CLR garbage collector

Error Handling
Error handling strategies differ by language:

Rust: Uses Result<T, PolifunctionError> for error handling
C++: Throws exceptions of type PolifunctionException
Python: Raises PolifunctionError exceptions
.NET: Throws PolifunctionException instances

Thread Safety
The SDK is designed to be thread-safe with the following considerations:

Core polifunctions are immutable and can be safely shared between threads
Mutable operations create new polifunctions rather than modifying existing ones
Language-specific thread safety mechanisms are respected

Performance Optimization
Compilation Flags
For Rust and C++, enable optimizations for maximum performance:

# Rust
cargo build --release

# C++
cmake -DCMAKE_BUILD_TYPE=Release ..

Batched Operations
For performance-critical applications, use batched operations when available:

# Python example of batched operation
results = poly_func.batch_evaluate(input_array)

Memory Usage
For large domains or complex polifunctions, consider using lazy evaluation strategies:

// Rust example of lazy evaluation
let lazy_fn = LazySetValuedPolifunction::new(mapping_function, domain, codomain);

Troubleshooting
Common Issues

Domain Errors: Ensure inputs are within the domain of your polifunctions
Type Compatibility: Check that type parameters match between composed polifunctions
Memory Leaks: In C++, ensure proper handling of polifunctions with custom deleters
Performance Issues: Use batched operations and appropriate optimization flags

Diagnostic Tools
The SDK provides diagnostic tools for troubleshooting:

// Enable debug logging
polifunctions_sdk::enable_debug_logging();

// Profile a polifunction
let profile = polifunctions_sdk::profile_function(&my_func, test_inputs);
println!("Performance profile: {:?}", profile);

Getting Help

Check the GitHub Issues
Ask questions on Stack Overflow
Join our Discord Community

Version Compatibility
SDK VersionRustC++Python.NET0.1.x1.53+C++173.7+.NET 5.0+0.2.x1.56+C++173.8+.NET 6.0+
Next Steps

Explore Domain Extensions
Check out Examples and Tutorials
Learn about Performance Optimization

