//! Neural Network module
//!
//! Contains implementation of simple feed forward neural network.
//!
//! # Usage
//!
//! ```
//! use rusty_machine::learning::nnet::NeuralNet;
//! use rusty_machine::linalg::matrix::Matrix;
//! use rusty_machine::learning::SupModel;
//!
//! let inputs = Matrix::new(5,3, vec![1.,1.,1.,2.,2.,2.,3.,3.,3.,
//! 								4.,4.,4.,5.,5.,5.,]);
//! let targets = Matrix::new(5,3, vec![1.,0.,0.,0.,1.,0.,0.,0.,1.,
//! 									0.,0.,1.,0.,0.,1.]);
//!
//! let layers = &[3,5,11,7,3];
//! let mut model = NeuralNet::default(layers);
//!
//! model.train(&inputs, &targets);
//!
//! let test_inputs = Matrix::new(2,3, vec![1.5,1.5,1.5,5.1,5.1,5.1]);
//!
//! model.predict(&test_inputs);
//! ```
//!
//! The neural networks are specified via a criterion - similar to [Torch](https://github.com/torch/nn/blob/master/doc/criterion.md).
//! The criterions combine an activation function and a cost function.
//!
//! You can define your own criterion by implementing the `Criterion`
//! trait with a concrete ActivationFunc and CostFunc.

use linalg::matrix::Matrix;
use learning::SupModel;
use learning::toolkit::activ_fn;
use learning::toolkit::activ_fn::ActivationFunc;
use learning::toolkit::cost_fn;
use learning::toolkit::cost_fn::CostFunc;
use learning::optim::{Optimizable, OptimAlgorithm};
use learning::optim::grad_desc::StochasticGD;

use rand::{Rng, thread_rng};

/// Neural Network struct
pub struct NeuralNet<'a, T: Criterion> {
    layer_sizes: &'a [usize],
    weights: Vec<f64>,
    gd: StochasticGD,
    criterion: T,
}

impl<'a> NeuralNet<'a, BCECriterion> {
    /// Creates a neural network with the specified layer sizes.
    ///
    /// Uses the default settings (gradient descent and sigmoid activation function).
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::nnet::NeuralNet;
    ///
    /// // Create a neural net with 4 layers, 3 neurons in each.
    /// let layers = &[3; 4];
    /// let mut net = NeuralNet::default(layers);
    /// ```
    pub fn default(layer_sizes: &[usize]) -> NeuralNet<BCECriterion> {
        NeuralNet {
            layer_sizes: layer_sizes,
            weights: NeuralNet::<BCECriterion>::create_weights(layer_sizes),
            gd: StochasticGD::default(),
            criterion: BCECriterion,
        }
    }
}
impl<'a, T: Criterion> NeuralNet<'a, T> {
    /// Create a new neural network with the specified layer sizes.
    ///
    /// The layer sizes slice should include the input, hidden layers, and output layer sizes.
    /// The type of activation function must be specified.
    ///
    /// Currently defaults to simple batch Gradient Descent for optimization.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::nnet::BCECriterion;
    /// use rusty_machine::learning::nnet::NeuralNet;
    ///
    /// // Create a neural net with 4 layers, 3 neurons in each.
    /// let layers = &[3; 4];
    /// let mut net = NeuralNet::new(layers, BCECriterion);
    /// ```
    pub fn new(layer_sizes: &[usize], criterion: T) -> NeuralNet<T> {
        NeuralNet {
            layer_sizes: layer_sizes,
            weights: NeuralNet::<T>::create_weights(layer_sizes),
            gd: StochasticGD::default(),
            criterion: criterion,
        }
    }

    /// Creates initial weights for all neurons in the network.
    fn create_weights(layer_sizes: &[usize]) -> Vec<f64> {
        let total_layers = layer_sizes.len();

        let mut layers = Vec::new();

        for (l, item) in layer_sizes.iter().enumerate().take(total_layers - 1) {
            layers.append(&mut NeuralNet::<T>::initialize_weights(item + 1,
                                                             layer_sizes[l + 1]));
        }
        layers.shrink_to_fit();

        layers
    }

    /// Initializes the weights for a single layer in the network.
    fn initialize_weights(l_in: usize, l_out: usize) -> Vec<f64> {
        let mut weights = Vec::with_capacity(l_in * l_out);
        let eps_init = (6f64 / (l_in + l_out) as f64).sqrt();

        let mut rng = thread_rng();

        for _i in 0..l_in * l_out {
            let w = (rng.gen_range(0f64, 1f64) * 2f64 * eps_init) - eps_init;
            weights.push(w);
        }

        weights
    }

    /// Gets matrix of weights between specified layer and forward layer for the weights.
    fn get_layer_weights(&self, weights: &[f64], idx: usize) -> Matrix<f64> {
        assert!(idx < self.layer_sizes.len() - 1);

        // Check that the weights are the right size.
        let mut full_size = 0usize;
        for l in 0..self.layer_sizes.len() - 1 {
            full_size += (self.layer_sizes[l] + 1) * self.layer_sizes[l + 1];
        }

        assert_eq!(full_size, weights.len());

        let mut start = 0usize;

        for l in 0..idx {
            start += (self.layer_sizes[l] + 1) * self.layer_sizes[l + 1]
        }

        let capacity = (self.layer_sizes[idx] + 1) * self.layer_sizes[idx + 1];

        let mut layer_weights = Vec::with_capacity((self.layer_sizes[idx] + 1) *
                                                   self.layer_sizes[idx + 1]);
        unsafe {
            for i in start..start + capacity {
                layer_weights.push(*weights.get_unchecked(i));
            }
        }

        Matrix::new(self.layer_sizes[idx] + 1,
                    self.layer_sizes[idx + 1],
                    layer_weights)

    }

    /// Gets matrix of weights between specified layer and forward layer.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::learning::nnet::NeuralNet;
    ///
    /// // Create a neural net with 4 layers, 3 neurons in each.
    /// let layers = &[3; 4];
    /// let mut net = NeuralNet::default(layers);
    ///
    /// let w = &net.get_net_weights(2);
    ///
    /// // We add a bias term to the weight matrix
    /// assert_eq!(w.rows(), 4);
    /// assert_eq!(w.cols(), 3);
    /// ```
    pub fn get_net_weights(&self, idx: usize) -> Matrix<f64> {
        self.get_layer_weights(&self.weights[..], idx)
    }

    // Get the matrix of weights without the bias terms.
    // fn get_regular_weights(&self, weights: &[f64]) -> Vec<f64> {
    // let mut reg_weights = Vec::new();
    //
    // Check that the weights are the right size.
    // let mut start = 0usize;
    // for l in 0..self.layer_sizes.len() - 1 {
    //
    // for i in 0..self.layer_sizes[l] {
    // for j in 0..self.layer_sizes[l + 1] {
    // reg_weights.push(weights[start + j*(1+self.layer_sizes[l]) + 1 + i] )
    // }
    // }
    //
    // start += (self.layer_sizes[l]+1) * self.layer_sizes[l + 1];
    // }
    //
    // reg_weights
    // }
    //

    /// Compute the gradient using the back propagation algorithm.
    fn compute_grad(&self,
                    weights: &[f64],
                    inputs: &Matrix<f64>,
                    targets: &Matrix<f64>)
                    -> (f64, Vec<f64>) {
        assert_eq!(inputs.cols(), self.layer_sizes[0]);

        let mut forward_weights = Vec::with_capacity(self.layer_sizes.len() - 1);
        let mut activations = Vec::with_capacity(self.layer_sizes.len());

        let net_data = Matrix::ones(inputs.rows(), 1).hcat(inputs);

        activations.push(net_data.clone());

        // Forward propagation
        {
            let mut z = net_data * self.get_layer_weights(weights, 0);
            forward_weights.push(z.clone());

            for l in 1..self.layer_sizes.len() - 1 {
                let mut a = self.criterion.activate(z.clone());
                let ones = Matrix::ones(a.rows(), 1);

                a = ones.hcat(&a);

                activations.push(a.clone());
                z = a * self.get_layer_weights(weights, l);
                forward_weights.push(z.clone());
            }

            activations.push(self.criterion.activate(z));
        }

        let mut deltas = Vec::with_capacity(self.layer_sizes.len() - 1);
        // Backward propagation
        {
            let z = forward_weights[self.layer_sizes.len() - 2].clone();
            let g = self.criterion.grad_activ(z);

            // Take GRAD_cost to compute this delta.
            let mut delta = self.criterion
                                .cost_grad(&activations[self.layer_sizes.len() - 1], targets)
                                .elemul(&g);

            deltas.push(delta.clone());

            for l in (1..self.layer_sizes.len() - 1).rev() {
                let mut z = forward_weights[l - 1].clone();
                let ones = Matrix::ones(z.rows(), 1);
                z = ones.hcat(&z);

                let g = self.criterion.grad_activ(z);
                delta = (delta * self.get_layer_weights(weights, l).transpose()).elemul(&g);

                let non_one_rows = &(1..delta.cols()).collect::<Vec<usize>>()[..];
                delta = delta.select_cols(non_one_rows);
                deltas.push(delta.clone());
            }
        }

        let mut grad = Vec::with_capacity(self.layer_sizes.len() - 1);
        let mut capacity = 0;

        for (l, activ_item) in activations.iter().enumerate().take(self.layer_sizes.len() - 1) {
            let g = deltas[self.layer_sizes.len() - 2 - l].transpose() * activ_item;
            capacity += g.cols() * g.rows();
            grad.push(g / (inputs.rows() as f64));
        }

        let mut gradients = Vec::with_capacity(capacity);

        for g in grad {
            gradients.append(&mut g.into_vec());
        }
        (self.criterion.cost(&activations[activations.len() - 1], targets),
         gradients)
    }

    /// Forward propagation of the model weights to get the outputs.
    fn forward_prop(&self, inputs: &Matrix<f64>) -> Matrix<f64> {
        assert_eq!(inputs.cols(), self.layer_sizes[0]);

        let net_data = Matrix::ones(inputs.rows(), 1).hcat(inputs);

        let mut z = net_data * self.get_net_weights(0);
        let mut a = self.criterion.activate(z.clone());

        for l in 1..self.layer_sizes.len() - 1 {
            let ones = Matrix::ones(a.rows(), 1);
            a = ones.hcat(&a);
            z = a * self.get_net_weights(l);
            a = self.criterion.activate(z.clone());
        }

        a
    }
}

impl<'a, T: Criterion> Optimizable for NeuralNet<'a, T> {
    type Inputs = Matrix<f64>;
	type Targets = Matrix<f64>;

    /// Compute the gradient of the neural network.
    fn compute_grad(&self,
                    params: &[f64],
                    inputs: &Matrix<f64>,
                    targets: &Matrix<f64>)
                    -> (f64, Vec<f64>) {
        self.compute_grad(params, inputs, targets)
    }
}

impl<'a, T: Criterion> SupModel<Matrix<f64>, Matrix<f64>> for NeuralNet<'a, T> {
    /// Predict neural network output using forward propagation.
    fn predict(&self, inputs: &Matrix<f64>) -> Matrix<f64> {
        self.forward_prop(inputs)
    }

    /// Train the model using gradient optimization and back propagation.
    fn train(&mut self, inputs: &Matrix<f64>, targets: &Matrix<f64>) {
        let start = self.weights.clone();
        let optimal_w = self.gd.optimize(self, &start[..], inputs, targets);
        self.weights = optimal_w;
    }
}

/// Criterion for Neural Networks
///
/// Specifies an activation function and a cost function.
pub trait Criterion {
    /// The activation function for the criterion.
    type ActFunc: ActivationFunc;
    /// The cost function for the criterion.
    type Cost: CostFunc<Matrix<f64>>;

    /// The activation function applied to a matrix.
    fn activate(&self, mat: Matrix<f64>) -> Matrix<f64> {
        mat.apply(&Self::ActFunc::func)
    }

    /// The gradient of the activation function applied to a matrix.
    fn grad_activ(&self, mat: Matrix<f64>) -> Matrix<f64> {
        mat.apply(&Self::ActFunc::func_grad)
    }

    /// The cost function.
    ///
    /// Returns a scalar cost.
    fn cost(&self, outputs: &Matrix<f64>, targets: &Matrix<f64>) -> f64 {
        Self::Cost::cost(outputs, targets)
    }

    /// The gradient of the cost function.
    ///
    /// Returns a matrix of cost gradients.
    fn cost_grad(&self, outputs: &Matrix<f64>, targets: &Matrix<f64>) -> Matrix<f64> {
        Self::Cost::grad_cost(outputs, targets)
    }
}

/// The binary cross entropy criterion.
///
/// Uses the Sigmoid activation function and the
/// cross entropy error.
pub struct BCECriterion;

impl Criterion for BCECriterion {
    type ActFunc = activ_fn::Sigmoid;
    type Cost = cost_fn::CrossEntropyError;
}

/// The mean squared error criterion.
///
/// Uses the Linear activation function and the
/// mean squared error.
pub struct MSECriterion;

impl Criterion for MSECriterion {
    type ActFunc = activ_fn::Linear;
    type Cost = cost_fn::MeanSqError;
}
