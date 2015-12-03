//! A machine learning crate for Rust.
//!
//!
//! # Introduction
//!
//! This crate is mostly an excuse for me to learn Rust. Nevertheless, it contains reasonably effective
//! implementations of a number of common machine learing algorithms.
//!
//! At the moment, `rustlearn` uses its own basic dense and sparse array types, but I will be happy
//! to use something more robust once a clear winner in that space emerges.
//!
//! # Features
//!
//! ## Matrix primitives
//!
//! - [dense matrices](array/dense/index.html)
//! - [sparse matrices](array/sparse/index.html)
//! 
//! ## Models
//! 
//! - [logistic regression](linear_models/sgdclassifier/index.html) using stochastic gradient descent,
//! - [decision trees](trees/decision_tree/index.html) using the CART algorithm, and
//! - [random forests](ensemble/random_forest/index.html) using CART decision trees.
//! 
//! All the models support fitting and prediction on both dense and sparse data, and the implementations
//! should be roughly competitive with Python `sklearn` implementations, both in accuracy and performance.
//!
//! # Using `rustlearn`
//! Usage should be straightforward.
//! 
//! - import the prelude for alll the linear algebra primitives and common traits:
//!
//! ```
//! use rustlearn::prelude::*;
//! ```
//!
//! - import individual models and utilities from submodules:
//!
//! ```
//! use rustlearn::prelude::*;
//!
//! use rustlearn::linear_models::sgdclassifier::Hyperparameters;
//! // more imports
//! ```
//! 
//! # Examples
//!
//! ## Logistic regression
//!
//! ```
//! use rustlearn::prelude::*;
//! use rustlearn::datasets::iris;
//! use rustlearn::cross_validation::CrossValidation;
//! use rustlearn::linear_models::sgdclassifier::Hyperparameters;
//! use rustlearn::metrics::accuracy_score;
//! 
//!
//! let (X, y) = iris::load_data();
//!
//! let num_splits = 10;
//! let num_epochs = 5;
//!
//! let mut accuracy = 0.0;
//!
//! for (train_idx, test_idx) in CrossValidation::new(X.rows(), num_splits) {
//!
//!     let X_train = X.get_rows(&train_idx);
//!     let y_train = y.get_rows(&train_idx);
//!     let X_test = X.get_rows(&test_idx);
//!     let y_test = y.get_rows(&test_idx);
//! 
//!     let mut model = Hyperparameters::new(X.cols())
//!                                     .learning_rate(0.5)
//!                                     .l2_penalty(0.0)
//!                                     .l1_penalty(0.0)
//!                                     .one_vs_rest();
//!
//!     for _ in 0..num_epochs {
//!         model.fit(&X_train, &y_train).unwrap();
//!     }
//!
//!     let prediction = model.predict(&X_test).unwrap();
//!     accuracy += accuracy_score(&y_test, &prediction);
//! }
//!
//! accuracy /= num_splits as f32;
//!
//! ```
//!
//! ## Random forest
//!
//! ```
//! use rustlearn::prelude::*;
//! 
//! use rustlearn::ensemble::random_forest::Hyperparameters;
//! use rustlearn::datasets::iris;
//! use rustlearn::trees::decision_tree;
//! 
//! let (data, target) = iris::load_data();
//! 
//! let mut tree_params = decision_tree::Hyperparameters::new(data.cols());
//! tree_params.min_samples_split(10)
//!     .max_features(4);
//! 
//! let mut model = Hyperparameters::new(tree_params, 10)
//!     .one_vs_rest();
//! 
//! model.fit(&data, &target).unwrap();
//! 
//! let prediction = model.predict(&data).unwrap();
//! ```
//!


#![feature(test)]
#![allow(non_snake_case)]

extern crate rand;
extern crate test;

pub mod array;
pub mod cross_validation;
pub mod datasets;
pub mod ensemble;
pub mod linear_models;
pub mod metrics;
pub mod multiclass;
pub mod feature_extraction;
pub mod trees;
pub mod traits;


#[allow(unused_imports)]
pub mod prelude {
    //! Basic data structures and traits used throughout `rustlearn`.
    pub use array::prelude::*;
    pub use traits::*;
}