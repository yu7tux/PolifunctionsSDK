# PolifunctionsSDK Interface Reference

This document provides detailed information about all interfaces and types in the PolifunctionsSDK. It's automatically generated and synchronized with the codebase to ensure accuracy.

## Core Traits and Interfaces

### PolifunctionBase

The foundational trait for all polifunctions in the SDK.

```rust
pub trait PolifunctionBase {
    type Domain: Domain;
    type Codomain: Codomain;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool;
}


Description
PolifunctionBase defines the core behavior of all polifunctions. A polifunction is a generalization of mathematical functions that can map an input to multiple output values. This trait provides the foundation for implementing different types of polifunctions.
Type Parameters

Domain: The domain type of the polifunction
Codomain: The codomain (range) type of the polifunction

Methods

evaluate: Computes the polifunction value at the given input
in_domain: Checks if the input is in the domain of the polifunction

SetValuedPolifunction
Interface for polifunctions that return discrete sets of values.

pub trait SetValuedPolifunction: PolifunctionBase {
    fn value_set(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<HashSet<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element, 
                     value: &<Self::Codomain as Codomain>::Element) 
        -> Result<bool, PolifunctionError>;
    
    fn cardinality(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<usize, PolifunctionError>;
}

Description
SetValuedPolifunction extends the base polifunction concept to work specifically with discrete sets of values. This is useful for modeling functions that can return multiple distinct values.
Methods

value_set: Returns the set of all possible output values for the given input
contains_value: Checks if a specific value is in the output set for a given input
cardinality: Returns the number of elements in the output set for a given input

IntervalValuedPolifunction
Interface for polifunctions that return continuous intervals of values.

pub trait IntervalValuedPolifunction: PolifunctionBase {
    fn value_interval(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<Interval<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element, 
                     value: &<Self::Codomain as Codomain>::Element) 
        -> Result<bool, PolifunctionError>;
    
    fn interval_width(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<<Self::Codomain as Codomain>::Element, PolifunctionError>
    where
        <Self::Codomain as Codomain>::Element: Sub<Output = <Self::Codomain as Codomain>::Element> + Clone;
}

Description
IntervalValuedPolifunction specializes polifunctions to work with continuous intervals of values. This is particularly useful for representing uncertainty ranges or solution sets of inequalities.
Methods

value_interval: Returns the interval of possible output values for the given input
contains_value: Checks if a specific value is within the output interval
interval_width: Calculates the width of the output interval

Core Types
PolifunctionValue
An enum representing the different kinds of output values a polifunction can produce.

pub enum PolifunctionValue<T> {
    Single(T),
    Set(HashSet<T>),
    Interval(Interval<T>),
    Distribution(ProbabilityDistribution<T>),
    FuzzySet(FuzzySet<T>),
}

Variants

Single: A single, deterministic value
Set: A discrete set of possible values
Interval: A continuous interval between two values
Distribution: A probability distribution over possible values
FuzzySet: A fuzzy set with membership degrees

Interval
A structure representing a continuous interval.

pub struct Interval<T> {
    pub lower: T,
    pub upper: T,
    pub lower_inclusive: bool,
    pub upper_inclusive: bool,
}

Fields

lower: The lower bound of the interval
upper: The upper bound of the interval
lower_inclusive: Whether the lower bound is included in the interval
upper_inclusive: Whether the upper bound is included in the interval

Domain and Codomain
Traits for representing mathematical domains and codomains.

pub trait Domain {
    type Element;
    fn contains(&self, element: &Self::Element) -> bool;
}

pub trait Codomain {
    type Element;
    fn contains(&self, element: &Self::Element) -> bool;
}

Description

Domain: Represents the set of valid inputs for a function
Codomain: Represents the set of possible outputs for a function

Operations
Function Lifting
Functions to convert standard functions to polifunctions.

pub fn lift_to_set<F, D, C>(
    f: F, 
    domain: D, 
    codomain: C
) -> impl SetValuedPolifunction<Domain = D, Codomain = C>
where
    F: Fn(&D::Element) -> Result<C::Element, PolifunctionError>,
    D: Domain,
    C: Codomain,
    C::Element: Clone + std::hash::Hash + Eq;

Description
Converts a standard single-valued function into a set-valued polifunction.
Composition
Functions to compose polifunctions.

pub fn compose<P1, P2>(
    p1: P1, 
    p2: P2
) -> impl PolifunctionBase<Domain = P2::Domain, Codomain = P1::Codomain>
where
    P1: PolifunctionBase,
    P2: PolifunctionBase,
    <P2::Codomain as Codomain>::Element: Into<<P1::Domain as Domain>::Element>,
    <P1::Codomain as Codomain>::Element: Clone;

Description
Creates a new polifunction that is the composition of two existing polifunctions (p1 ∘ p2).
Type Conversion
Functions to convert between different types of polifunctions.

pub fn to_interval<P>(
    p: P
) -> impl IntervalValuedPolifunction<Domain = P::Domain, Codomain = P::Codomain>
where
    P: SetValuedPolifunction,
    <P::Codomain as Codomain>::Element: Clone + Ord;

Description
Converts a set-valued polifunction to an interval-valued one by taking the minimum and maximum values of the set.
Implementation Classes
BasicSetValuedPolifunction
A basic implementation of a set-valued polifunction.

pub struct BasicSetValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    D::Element: Clone + Hash + Eq,
    C::Element: Clone + Hash + Eq,
{
    mapping_function: Box<dyn Fn(&D::Element) -> Result<HashSet<C::Element>, PolifunctionError>>,
    domain: D,
    codomain: C,
}

Description
Provides a concrete implementation of a set-valued polifunction using a mapping function.
BasicIntervalValuedPolifunction
A basic implementation of an interval-valued polifunction.

pub struct BasicIntervalValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    C::Element: PartialOrd + Clone,
{
    mapping_function: Box<dyn Fn(&D::Element) -> Result<Interval<C::Element>, PolifunctionError>>,
    domain: D,
    codomain: C,
}

Description
Provides a concrete implementation of an interval-valued polifunction using a mapping function.
Extension Points
The SDK provides several extension points for custom implementations:

Custom Polifunction Types: By implementing the PolifunctionBase trait
Custom Domains and Codomains: By implementing the Domain and Codomain traits
Custom Operations: By creating functions that operate on polifunctions
Domain-Specific Extensions: Through the quantum, scientific computing, and verification modules

For detailed information on extending the SDK, see the Extension Points document.

