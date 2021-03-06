# Development

This document will keep track of my development goals for this project.

---

## Current Progress

There is now a first pass at the linear algebra library and some basic machine learning algorithms in place.

### Matrices

- Generic data matrices
- Concatenation
- Data manipulation (row and column selection/repetition etc.)
- Arithmetic

### Machine Learning

- Linear Regression
- K-Means Clustering
- Neural Networks
- Gaussian Processes
- Logistic Regression

I've decided for now to halt optimization efforts. It seems the best course of action is to decide as a community a single linear algebra library to utilize. This should also probably utilize BLAS and LAPACK bindings.

---

## Timeline

This marks my intended release goals. I won't estimate the actual dates of release but rather the content I want to include in each version. I am actively developing and so expect to move through these at a good pace!

<table>
    <tr>
        <th>Version</th><th>Features</th><th>Dependencies</th>
    </tr>
    <tr>
        <td>0.1.*</td><td><ul><li>Logistic Regression.</li><ul></td><td><ul><li>Looking to Generalized lin reg.</li><ul></td>
    </tr>
    <tr>
        <td>0.1.*</td><td><ul><li>SVMs</li><ul></td><td><ul><li>(Adaptive) Coordinate Descent. <i>See below</i></li><li>Subgradient methods.</li><ul></td>
    </tr>
    <tr>
        <td>0.1.*</td><td><ul><li>Generalized linear regression.</li><ul></td><td><ul><li>None.</li><ul></td>
    </tr>
    <tr>
        <td>0.1.*</td><td><ul><li>Regularization on cost functions.</li><ul></td><td><ul><li>Data Normalization.</li><li>Some refactoring.</li><ul></td>
    </tr>
    <tr>
        <td>0.2.0</td><td><ul><li>More advanced GD algorithms.</li><li>Optimization</li><li>Bug fixes</li></ul></td><td></td>
    </tr>
</table>

For Coordinate descent I will follow the algorithm defined in [this paper](http://www.loshchilov.com/publications/GECCO2011_AdaptiveCoordinateDescent.pdf).

*Note*: Regularization has been pushed back as it will require careful planning. I plan to implement the other features first and then try to round off the 0.2.0 release with regularization.

### Unplanned:

- Multi-threaded divide and conquer matrix multiplication (currently iterative).
- Tidy up indexing.
- Data Handling.
- Convolutional and Recurrent neural nets.
