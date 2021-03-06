//! Logistic Regression module
//!
//! Contains implemention of logistic regression using
//! gradient descent optimization.
//!
//! The regressor will automatically add the intercept term
//! so you do not need to format the input matrices yourself.
//!
//! # Usage
//!
//! ```
//! use rusty_machine::learning::logistic_reg::LogisticRegressor;
//! use rusty_machine::learning::SupModel;
//! use rusty_machine::linalg::matrix::Matrix;
//! use rusty_machine::linalg::vector::Vector;
//!
//! let inputs = Matrix::new(4,1,vec![1.0,3.0,5.0,7.0]);
//! let targets = Vector::new(vec![0.,0.,1.,1.]);
//!
//! let mut log_mod = LogisticRegressor::default();
//!
//! // Train the model
//! log_mod.train(&inputs, &targets);
//!
//! // Now we'll predict a new point
//! let new_point = Matrix::new(1,1,vec![10.]);
//! let output = log_mod.predict(&new_point);
//!
//! // Hopefully we classified our new point correctly!
//! assert!(output[0] > 0.5, "Our classifier isn't very good!");
//! ```
//!
//! We could have been more specific about the learning of the model
//! by using the `new` constructor instead. This allows us to provide
//! a `GradientDesc` object with custom parameters.

use learning::SupModel;
use linalg::matrix::Matrix;
use linalg::vector::Vector;
use learning::toolkit::activ_fn::ActivationFunc;
use learning::toolkit::activ_fn::Sigmoid;
use learning::toolkit::cost_fn::CostFunc;
use learning::toolkit::cost_fn::CrossEntropyError;
use learning::optim::grad_desc::GradientDesc;
use learning::optim::OptimAlgorithm;
use learning::optim::Optimizable;

/// Logistic Regression Model.
///
/// Contains option for optimized parameter.
pub struct LogisticRegressor {
    /// The parameters for the regression model.
    parameters: Option<Vector<f64>>,
    gd: GradientDesc,
}

impl Default for LogisticRegressor {
    fn default() -> LogisticRegressor {
        LogisticRegressor {
            parameters: None,
            gd: GradientDesc::default(),
        }
    }
}
impl LogisticRegressor {
    /// Constructs untrained logistic regression model.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::logistic_reg::LogisticRegressor;
    /// use rusty_machine::learning::optim::grad_desc::GradientDesc;
    ///
    /// let gd = GradientDesc::default();
    /// let mut logistic_mod = LogisticRegressor::new(gd);
    /// ```
    pub fn new(gd: GradientDesc) -> LogisticRegressor {
        LogisticRegressor {
            parameters: None,
            gd: gd,
        }
    }

    /// Get the parameters from the model.
    ///
    /// Returns an option that is None if the model has not been trained.
    pub fn parameters(&self) -> Option<Vector<f64>> {
        match self.parameters {
            None => None,
            Some(ref x) => Some(x.clone()),
        }
    }
}

impl SupModel<Matrix<f64>, Vector<f64>> for LogisticRegressor {
    /// Train the logistic regression model.
    ///
    /// Takes training data and output values as input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::logistic_reg::LogisticRegressor;
    /// use rusty_machine::linalg::matrix::Matrix;
    /// use rusty_machine::linalg::vector::Vector;
    /// use rusty_machine::learning::SupModel;
    ///
    /// let mut logistic_mod = LogisticRegressor::default();
    /// let inputs = Matrix::new(3,2, vec![1.0, 2.0, 1.0, 3.0, 1.0, 4.0]);
    /// let targets = Vector::new(vec![5.0, 6.0, 7.0]);
    ///
    /// logistic_mod.train(&inputs, &targets);
    /// ```
    fn train(&mut self, inputs: &Matrix<f64>, targets: &Vector<f64>) {
        let ones = Matrix::<f64>::ones(inputs.rows(), 1);
        let full_inputs = ones.hcat(inputs);

        let initial_params = vec![0.5; full_inputs.cols()];

        let optimal_w = self.gd.optimize(self, &initial_params[..], &full_inputs, targets);
        self.parameters = Some(Vector::new(optimal_w));
    }

    /// Predict output value from input data.
    ///
    /// Model must be trained before prediction can be made.
    fn predict(&self, inputs: &Matrix<f64>) -> Vector<f64> {
        if let Some(ref v) = self.parameters {
            let ones = Matrix::<f64>::ones(inputs.rows(), 1);
            let full_inputs = ones.hcat(inputs);
            (full_inputs * v).apply(&Sigmoid::func)
        } else {
            panic!("Model has not been trained.");
        }
    }
}

impl Optimizable for LogisticRegressor {
    type Inputs = Matrix<f64>;
    type Targets = Vector<f64>;

    fn compute_grad(&self,
                    params: &[f64],
                    inputs: &Matrix<f64>,
                    targets: &Vector<f64>)
                    -> (f64, Vec<f64>) {

        let beta_vec = Vector::new(params.to_vec());
        let outputs = (inputs * beta_vec).apply(&Sigmoid::func);

        let cost = CrossEntropyError::cost(&outputs, targets);
        let grad = (inputs.transpose() * (outputs - targets)) / (inputs.rows() as f64);

        (cost, grad.into_vec())
    }
}
