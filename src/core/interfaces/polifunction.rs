//! Core traits and types for the polifunction framework.
//! 
//! This module defines the fundamental abstractions for working with
//! polifunctions - mathematical functions that can return multiple values.

use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display};

/// Error type for polifunction operations
#[derive(Debug)]
pub enum PolifunctionError {
    /// Input is outside the function's domain
    DomainError,
    /// Error during computation or evaluation
    ComputationError,
    /// Failed to converge to a result
    ConvergenceError,
    /// Invalid operation for this polifunction type
    InvalidOperation,
    /// Other errors with description
    Other(String),
}

impl Display for PolifunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolifunctionError::DomainError => write!(f, "Input is outside the function's domain"),
            PolifunctionError::ComputationError => write!(f, "Error during computation"),
            PolifunctionError::ConvergenceError => write!(f, "Failed to converge to a result"),
            PolifunctionError::InvalidOperation => write!(f, "Invalid operation for this polifunction type"),
            PolifunctionError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for PolifunctionError {}

/// Represents possible output values of a polifunction
#[derive(Debug, Clone)]
pub enum PolifunctionValue<T> {
    /// A single value
    Single(T),
    /// A set of discrete values
    Set(HashSet<T>),
    /// A continuous interval
    Interval(Interval<T>),
    /// A probability distribution over possible values
    Distribution(ProbabilityDistribution<T>),
    /// A fuzzy set with membership degrees
    FuzzySet(FuzzySet<T>),
}

/// Trait for mathematical domains
pub trait Domain {
    /// Type of elements in this domain
    type Element;
    
    /// Check if an element belongs to this domain
    fn contains(&self, element: &Self::Element) -> bool;
}

/// Trait for mathematical codomains (ranges)
pub trait Codomain {
    /// Type of elements in this codomain
    type Element;
    
    /// Check if an element belongs to this codomain
    fn contains(&self, element: &Self::Element) -> bool;
}

/// Base trait for all polifunctions
pub trait PolifunctionBase {
    /// Type representing the domain of this polifunction
    type Domain: Domain;
    
    /// Type representing the codomain of this polifunction
    type Codomain: Codomain;
    
    /// Evaluate the polifunction at the given input
    /// 
    /// Returns a PolifunctionValue representing the possible outputs,
    /// or an error if evaluation is not possible.
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    /// Check if a given input is in the domain of this polifunction
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool;
}

/// Continuous interval [a, b]
#[derive(Debug, Clone)]
pub struct Interval<T> {
    pub lower: T,
    pub upper: T,
    pub lower_inclusive: bool,
    pub upper_inclusive: bool,
}

/// Probability distribution over possible values
#[derive(Debug, Clone)]
pub struct ProbabilityDistribution<T> {
    // Implementation details would depend on specific needs
    // This is a placeholder
}

/// Fuzzy set with membership degrees
#[derive(Debug, Clone)]
pub struct FuzzySet<T> {
    // Implementation details would depend on specific needs
    // This is a placeholder
}

/// Trait for composable polifunctions
pub trait Composable: PolifunctionBase {
    /// Compose this polifunction with another
    fn compose<P>(&self, other: &P) -> Result<ComposedPolifunction<Self, P>, PolifunctionError>
    where
        P: PolifunctionBase,
        <P::Codomain as Codomain>::Element: Into<<Self::Domain as Domain>::Element>,
        Self: Sized;
}

/// Result of composing two polifunctions
pub struct ComposedPolifunction<P1, P2>
where
    P1: PolifunctionBase,
    P2: PolifunctionBase,
{
    p1: P1,
    p2: P2,
}

impl<P1, P2> PolifunctionBase for ComposedPolifunction<P1, P2>
where
    P1: PolifunctionBase,
    P2: PolifunctionBase,
    <P2::Codomain as Codomain>::Element: Into<<P1::Domain as Domain>::Element>,
{
    type Domain = P2::Domain;
    type Codomain = P1::Codomain;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        // This would contain the actual implementation for function composition
        // For now, we just return an error as a placeholder
        Err(PolifunctionError::Other("ComposedPolifunction evaluation not implemented".to_string()))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.p2.in_domain(input)
    }
}
