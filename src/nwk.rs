//! this module contains traits and data structures for the network layer.
pub trait NlmeSap{
    fn nlme_get(&self);
    fn nlme_set(&self);
}
