//! Interval-valued polifunctions implementation.
//!
//! This module provides traits and implementations for polifunctions
//! that map inputs to intervals of output values.

use super::polifunction::{PolifunctionBase, PolifunctionValue, PolifunctionError, Domain, Codomain, Interval};
use std::cmp::PartialOrd;
use std::ops::{Add, Sub};

/// Trait for interval-valued polifunctions
pub trait IntervalValuedPolifunction: PolifunctionBase {
    /// Get the interval of values at the given input
    fn value_interval(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<Interval<<Self::Codomain as Codomain>::Element>, PolifunctionError>;
    
    /// Check if a specific value is in the output interval for a given input
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element, 
                     value: &<Self::Codomain as Codomain>::Element) 
        -> Result<bool, PolifunctionError>;
    
    /// Get the width of the output interval for a given input
    fn interval_width(&self, input: &<Self::Domain as Domain>::Element) 
        -> Result<<Self::Codomain as Codomain>::Element, PolifunctionError>
    where
        <Self::Codomain as Codomain>::Element: Sub<Output = <Self::Codomain as Codomain>::Element> + Clone;
}

/// Basic implementation of an interval-valued polifunction
pub struct BasicIntervalValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    C::Element: PartialOrd + Clone,
{
    /// Function that maps inputs to intervals of outputs
    mapping_function: Box<dyn Fn(&D::Element) -> Result<Interval<C::Element>, PolifunctionError>>,
    /// Domain of the function
    domain: D,
    /// Codomain of the function
    codomain: C,
}

impl<D, C> BasicIntervalValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    C::Element: PartialOrd + Clone,
{
    /// Create a new interval-valued polifunction with the given mapping function
    pub fn new(
        mapping_function: impl Fn(&D::Element) -> Result<Interval<C::Element>, PolifunctionError> + 'static,
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

impl<D, C> PolifunctionBase for BasicIntervalValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    C::Element: PartialOrd + Clone,
{
    type Domain = D;
    type Codomain = C;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let interval = (self.mapping_function)(input)?;
        Ok(PolifunctionValue::Interval(interval))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.domain.contains(input)
    }
}

impl<D, C> IntervalValuedPolifunction for BasicIntervalValuedPolifunction<D, C>
where
    D: Domain,
    C: Codomain,
    C::Element: PartialOrd + Clone,
{
    fn value_interval(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<Interval<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        (self.mapping_function)(input)
    }
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element,
                     value: &<Self::Codomain as Codomain>::Element)
        -> Result<bool, PolifunctionError> {
        let interval = self.value_interval(input)?;
        
        let lower_check = match (&interval.lower_inclusive, value.partial_cmp(&interval.lower)) {
            (true, Some(std::cmp::Ordering::Equal)) => true,
            (_, Some(std::cmp::Ordering::Greater)) => true,
            _ => false,
        };
        
        let upper_check = match (&interval.upper_inclusive, value.partial_cmp(&interval.upper)) {
            (true, Some(std::cmp::Ordering::Equal)) => true,
            (_, Some(std::cmp::Ordering::Less)) => true,
            _ => false,
        };
        
        Ok(lower_check && upper_check)
    }
    
    fn interval_width(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<<Self::Codomain as Codomain>::Element, PolifunctionError>
    where
        <Self::Codomain as Codomain>::Element: Sub<Output = <Self::Codomain as Codomain>::Element> + Clone,
    {
        let interval = self.value_interval(input)?;
        Ok(interval.upper.clone() - interval.lower.clone())
    }
}

/// Hull of two interval-valued polifunctions (smallest interval containing both)
pub struct HullPolifunction<P1, P2>
where
    P1: IntervalValuedPolifunction,
    P2: IntervalValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Codomain as Codomain>::Element: PartialOrd + Clone,
{
    p1: P1,
    p2: P2,
}

impl<P1, P2> HullPolifunction<P1, P2>
where
    P1: IntervalValuedPolifunction,
    P2: IntervalValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Codomain as Codomain>::Element: PartialOrd + Clone,
{
    /// Create a new hull of two interval-valued polifunctions
    pub fn new(p1: P1, p2: P2) -> Self {
        Self { p1, p2 }
    }
}

impl<P1, P2> PolifunctionBase for HullPolifunction<P1, P2>
where
    P1: IntervalValuedPolifunction,
    P2: IntervalValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Codomain as Codomain>::Element: PartialOrd + Clone,
{
    type Domain = P1::Domain;
    type Codomain = P1::Codomain;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let interval = self.value_interval(input)?;
        Ok(PolifunctionValue::Interval(interval))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.p1.in_domain(input) || self.p2.in_domain(input)
    }
}

impl<P1, P2> IntervalValuedPolifunction for HullPolifunction<P1, P2>
where
    P1: IntervalValuedPolifunction,
    P2: IntervalValuedPolifunction<Domain = P1::Domain, Codomain = P1::Codomain>,
    <P1::Codomain as Codomain>::Element: PartialOrd + Clone,
{
    fn value_interval(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<Interval<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let interval1 = match self.p1.value_interval(input) {
            Ok(i) => i,
            Err(e) => {
                if matches!(e, PolifunctionError::DomainError) {
                    // If it's a domain error, try the second function only
                    return self.p2.value_interval(input);
                } else {
                    return Err(e);
                }
            }
        };
        
        let interval2 = match self.p2.value_interval(input) {
            Ok(i) => i,
            Err(e) => {
                if matches!(e, PolifunctionError::DomainError) {
                    // If it's a domain error, use just the first interval
                    return Ok(interval1);
                } else {
                    return Err(e);
                }
            }
        };
        
        // Compute the hull (smallest interval containing both intervals)
        let lower = match interval1.lower.partial_cmp(&interval2.lower) {
            Some(std::cmp::Ordering::Less) => (interval1.lower.clone(), interval1.lower_inclusive),
            Some(std::cmp::Ordering::Equal) => (interval1.lower.clone(), interval1.lower_inclusive || interval2.lower_inclusive),
            Some(std::cmp::Ordering::Greater) => (interval2.lower.clone(), interval2.lower_inclusive),
            None => return Err(PolifunctionError::ComputationError),
        };
        
        let upper = match interval1.upper.partial_cmp(&interval2.upper) {
            Some(std::cmp::Ordering::Greater) => (interval1.upper.clone(), interval1.upper_inclusive),
            Some(std::cmp::Ordering::Equal) => (interval1.upper.clone(), interval1.upper_inclusive || interval2.upper_inclusive),
            Some(std::cmp::Ordering::Less) => (interval2.upper.clone(), interval2.upper_inclusive),
            None => return Err(PolifunctionError::ComputationError),
        };
        
        Ok(Interval {
            lower: lower.0,
            upper: upper.0,
            lower_inclusive: lower.1,
            upper_inclusive: upper.1,
        })
    }
    
    fn contains_value(&self, input: &<Self::Domain as Domain>::Element,
                     value: &<Self::Codomain as Codomain>::Element)
        -> Result<bool, PolifunctionError> {
        let interval = self.value_interval(input)?;
        
        let lower_check = match (&interval.lower_inclusive, value.partial_cmp(&interval.lower)) {
            (true, Some(std::cmp::Ordering::Equal)) => true,
            (_, Some(std::cmp::Ordering::Greater)) => true,
            _ => false,
        };
        
        let upper_check = match (&interval.upper_inclusive, value.partial_cmp(&interval.upper)) {
            (true, Some(std::cmp::Ordering::Equal)) => true,
            (_, Some(std::cmp::Ordering::Less)) => true,
            _ => false,
        };
        
        Ok(lower_check && upper_check)
    }
    
    fn interval_width(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<<Self::Codomain as Codomain>::Element, PolifunctionError>
    where
        <Self::Codomain as Codomain>::Element: Sub<Output = <Self::Codomain as Codomain>::Element> + Clone,
    {
        let interval = self.value_interval(input)?;
        Ok(interval.upper.clone() - interval.lower.clone())
    }
}
