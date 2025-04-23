//! Operations on polifunctions.
//!
//! This module provides common operations that can be performed on polifunctions,
//! such as composition, inversion, and algebraic operations.

use super::polifunction::{PolifunctionBase, PolifunctionValue, PolifunctionError, Domain, Codomain};
use super::set_valued::{SetValuedPolifunction};
use super::interval_valued::{IntervalValuedPolifunction};
use std::collections::HashSet;
use std::marker::PhantomData;

/// Lift a standard function to a polifunction
pub struct LiftedPolifunction<F, D, C>
where
    F: Fn(&D::Element) -> Result<C::Element, PolifunctionError>,
    D: Domain,
    C: Codomain,
{
    /// The original function
    function: F,
    /// Domain of the function
    domain: D,
    /// Codomain of the function
    codomain: C,
    /// Phantom data for type safety
    _phantom: PhantomData<(D::Element, C::Element)>,
}

impl<F, D, C> LiftedPolifunction<F, D, C>
where
    F: Fn(&D::Element) -> Result<C::Element, PolifunctionError>,
    D: Domain,
    C: Codomain,
{
    /// Create a new lifted polifunction from a standard function
    pub fn new(function: F, domain: D, codomain: C) -> Self {
        Self {
            function,
            domain,
            codomain,
            _phantom: PhantomData,
        }
    }
}

impl<F, D, C> PolifunctionBase for LiftedPolifunction<F, D, C>
where
    F: Fn(&D::Element) -> Result<C::Element, PolifunctionError>,
    D: Domain,
    C: Codomain,
    C::Element: Clone,
{
    type Domain = D;
    type Codomain = C;
    
    fn evaluate(&self, input: &<Self::Domain as Domain>::Element)
        -> Result<PolifunctionValue<<Self::Codomain as Codomain>::Element>, PolifunctionError> {
        if !self.in_domain(input) {
            return Err(PolifunctionError::DomainError);
        }
        
        let value = (self.function)(input)?;
        Ok(PolifunctionValue::Single(value))
    }
    
    fn in_domain(&self, input: &<Self::Domain as Domain>::Element) -> bool {
        self.domain.contains(input)
    }
}

/// Invert a polifunction (domain and codomain are swapped)
pub struct InvertedPolifunction<P>
where
    P: PolifunctionBase,
{
    /// The original polifunction
    original: P,
}

impl<P> InvertedPolifunction<P>
where
    P: PolifunctionBase,
{
    /// Create a new inverted polifunction
    pub fn new(original: P) -> Self {
        Self {
