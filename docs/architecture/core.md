# Core SDK Architecture

The Core SDK represents the fundamental mathematical layer of the PolifunctionsSDK, implementing the theoretical foundation of polifunctions as described in the paper ["Polifunctions: A Formal Framework for Multi-Valued Mathematical Functions"](https://osf.io/ywd4x_v1).

## Key Components

### Base Types and Traits

The core layer defines fundamental mathematical types and abstract traits that form the basis of the polifunction framework:

- **Domain**: Represents the domain of a function
- **Codomain**: Represents the codomain (range) of a function
- **Element**: Represents elements in domains and codomains
- **PolifunctionBase**: The root trait for all polifunctions

### Polifunction Types

Different types of polifunctions are implemented, each with specific characteristics:

- **SetValuedPolifunction**: Functions that map to sets of values
- **IntervalValuedPolifunction**: Functions that map to intervals
- **ProbabilisticPolifunction**: Functions that map to probability distributions
- **FuzzyPolifunction**: Functions with fuzzy set outputs

### Operations

The core SDK provides implementations of key operations on polifunctions:

- **Composition**: Composing multiple polifunctions
- **Inversion**: Creating inverse polifunctions
- **Union and Intersection**: Set-theoretic operations
- **Lifting**: Converting standard functions to polifunctions
- **Projection**: Extracting standard functions from polifunctions

### Type Conversion

Mechanisms for converting between different types of polifunctions:

- **Promotion**: Converting from simpler to more complex types
- **Reduction**: Simplifying complex polifunctions when possible
- **Approximation**: Creating approximate representations

### FFI Layer

The Foreign Function Interface (FFI) layer exposes the core functionality to other programming languages:

- **C API**: A stable C interface that other languages can bind to
- **Memory Management**: Safe handling of resources across language boundaries
- **Error Handling**: Consistent error propagation across language boundaries

## Implementation Details

### Type System

The type system is designed around Rust's trait system:

```rust
pub trait PolifunctionBase {
    type Domain;
    type Codomain;
    
    // Core methods
    fn evaluate(&self, input: &Self::Domain) -> Result<PolifunctionValue<Self::Codomain>, PolifunctionError>;
}

pub enum PolifunctionValue<T> {
    Single(T),
    Set(HashSet<T>),
    Interval(Interval<T>),
    Distribution(ProbabilityDistribution<T>),
    // Other variants...
}

Error Handling
Errors are handled through a dedicated error type:

pub enum PolifunctionError {
    DomainError,
    ComputationError,
    ConvergenceError,
    // Other error types...
}

Concurrency Model
The core implementation is designed to be thread-safe:

Immutable by default for most operations
Thread-safe data structures for concurrent access
Explicit synchronization where necessary

Extension Points
The Core SDK is designed to be extended in several ways:

New Polifunction Types: Implementing the PolifunctionBase trait
Custom Domains and Codomains: Creating new domain and codomain types
New Operations: Adding operations on existing polifunction types
Integration Points: Connecting with domain-specific mathematical constructs

Performance Considerations
Performance is a key consideration in the Core SDK design:

Memory layout optimization for common operations
Lazy evaluation where appropriate
Specialized implementations for common cases
Benchmarking infrastructure to measure and improve performance

