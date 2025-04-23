//! Set-valued polifunctions implementation.
//!
//! This module provides traits and implementations for polifunctions
//! that map inputs to sets of output values.

use std::collections::HashSet;
use std::hash::Hash;

use super::polifunction::{PolifunctionBase, PolifunctionValue, PolifunctionError, Domain, Codomain};

/// Trait for set-valued polifunctions
pub trait SetValuedPolifunction: PolifunctionBase {
    /// Get the set of values at the given input
    fn value_set(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<HashSet<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    /// Check if a specific value is in the output set for a given input
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element, 
                     value: &<Self::Codomain as Codomain>::Element) 
        -> Result<bool, PolifunctionError>;
    
    /// Get the cardinality of the output set for a given input
    fn cardinality(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<usize, PolifunctionError>;
}

/// Basic implementation of a set-valued polifunction
pub struct BasicSetValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    D::Element: Clone + Hash + Eq,
    C::Element: Clone + Hash + Eq,
{
    /// Function that maps inputs to sets of outputs
    mapping_function: Box<dyn Fn(&D::Element) -> Result<HashSet<C::Element>, PolifunctionError>>,
    /// Domain of the function
    domain: D,
    /// Codomain of the function
    codomain: C,
}

impl<D, C> BasicSetValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    D::Element: Clone + Hash + Eq,
    C::Element: Clone + Hash + Eq,
{
    /// Create a new set-valued polifunction with the given mapping function
    pub fn new(
        mapping_function: impl Fn(&D::Element) -> Result<HashSet<C::Element>, PolifunctionError> + 'static,
        domain: D,
        codomain: C,
    ) -> Self {
        Self {
            mapping_function: Box::new(mapping_function),
            domain,
            codomain,
        }
    }
}

impl<D, C> PolifunctionBase for BasicSetValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    D::Element: Clone + Hash + Eq,
    C::Element: Clone + Hash + Eq,
{
    type Domain = D;
    type Codomain = C;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let result_set = (self.mapping_function)(input)?;
        Ok(PolifunctionValue::Set(result_set))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.domain.contains(input)
    }
}

impl<D, C> SetValuedPolifunction for BasicSetValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    D::Element: Clone + Hash + Eq,
    C::Element: Clone + Hash + Eq,
{
    fn value_set(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<HashSet<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        (self.mapping_function)(input)
    }
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element,
                     value: &<Self::Codomain as Codomain>::Element)
        -> Result<bool, PolifunctionError> {
        let set = self.value_set(input)?;
        Ok(set.contains(value))
    }
    
    fn cardinality(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<usize, PolifunctionError> {
        let set = self.value_set(input)?;
        Ok(set.len())
    }
}

/// Union of two set-valued polifunctions
pub struct UnionPolifunction<P1, P2>
where
    P1: SetValuedPolifunction,
    P2: SetValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
{
    p1: P1,
    p2: P2,
}

impl<P1, P2> UnionPolifunction<P1, P2>
where
    P1: SetValuedPolifunction,
    P2: SetValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
{
    /// Create a new union of two set-valued polifunctions
    pub fn new(p1: P1, p2: P2) -> Self {
        Self { p1, p2 }
    }
}

impl<P1, P2> PolifunctionBase for UnionPolifunction<P1, P2>
where
    P1: SetValuedPolifunction,
    P2: SetValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Domain as Domain>::Element: Clone + Hash + Eq,
    <P1::Codomain as Codomain>::Element: Clone + Hash + Eq,
{
    type Domain = P1::Domain;
    type Codomain = P1::Codomain;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let mut result_set = HashSet::new();
        
        // Try to get values from the first polifunction
        match self.p1.value_set(input) {
            Ok(set1) => {
                result_set.extend(set1);
            },
            Err(e) => {
                if matches!(e, PolifunctionError::DomainError) {
                    // If it's a domain error, that's fine, we'll just use the second function
                } else {
                    return Err(e);
                }
            }
        }
        
        // Try to get values from the second polifunction
        match self.p2.value_set(input) {
            Ok(set2) => {
                result_set.extend(set2);
            },
            Err(e) => {
                if matches!(e, PolifunctionError::DomainError) {
                    // If it's a domain error, that's fine, we already have results from the first function
                    if result_set.is_empty() {
                        // But if we don't have any results from the first function either, it's an error
                        return Err(PolifunctionError::DomainError);
                    }
                } else {
                    return Err(e);
                }
            }
        }
        
        Ok(PolifunctionValue::Set(result_set))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.p1.in_domain(input) || self.p2.in_domain(input)
    }
}

impl<P1, P2> SetValuedPolifunction for UnionPolifunction<P1, P2>
where
    P1: SetValuedPolifunction,
    P2: SetValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Domain as Domain>::Element: Clone + Hash + Eq,
    <P1::Codomain as Codomain>::Element: Clone + Hash + Eq,
{
    fn value_set(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<HashSet<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let mut result_set = HashSet::new();
        
        // Try to get values from the first polifunction
        if let Ok(set1) = self.p1.value_set(input) {
            result_set.extend(set1);
        }
        
        // Try to get values from the second polifunction
        if let Ok(set2) = self.p2.value_set(input) {
            result_set.extend(set2);
        }
        
        if result_set.is_empty() {
            return Err(PolifunctionError::DomainError);
        }
        
        Ok(result_set)
    }
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element,
                     value: &<Self::Codomain as Codomain>::Element)
        -> Result<bool, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        // Check if either polifunction contains the value
        match self.p1.contains_value(input, value) {
            Ok(true) => return Ok(true),
            Ok(false) => {},
            Err(e) => {
                if !matches!(e, PolifunctionError::DomainError) {
                    return Err(e);
                }
            }
        }
        
        match self.p2.contains_value(input, value) {
            Ok(result) => return Ok(result),
            Err(e) => {
                if matches!(e, PolifunctionError::DomainError) {
                    // If both functions have domain errors, then it's a domain error
                    return Err(PolifunctionError::DomainError);
                } else {
                    return Err(e);
                }
            }
        }
    }
    
    fn cardinality(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<usize, PolifunctionError> {
        let set = self.value_set(input)?;
        Ok(set.len())
    }
}
