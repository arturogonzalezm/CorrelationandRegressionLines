# Regression Slope Calculator

This project computes the slope of a regression line given two sets of data points (independent and dependent variables). It treats the first dataset (Physics scores) as the independent variable (x) and the second dataset (History scores) as the dependent variable (y).

## Problem Statement

Given the test scores of 10 students in Physics and History, compute the slope of the regression line obtained by treating Physics as the independent variable. The result should be rounded to three decimal places.

### The scores to use:

```
Physics Scores: 15, 12, 8, 8, 7, 7, 7, 6, 5, 3
History Scores: 10, 25, 17, 11, 13, 17, 20, 13, 9, 15
```

## Formula for the Slope

The slope \(m\) of a regression line is given by:

\[
 m = \frac{\sum_{i=1}^n (x_i - \bar{x})(y_i - \bar{y})}{\sum_{i=1}^n (x_i - \bar{x})^2}
\]

Where:

- \(m\) is the slope of the regression line.
- \(x_i\) and \(y_i\) are the data points.
- \(\bar{x}\) and \(\bar{y}\) are the means of the x-values and y-values, respectively.
- \(n\) is the number of data points.

## Usage

1. Clone the repository.
2. Ensure you have Rust and Cargo installed.
3. Build and run the project:

   ```bash
   cargo run
   ```

4. The program will print the slope of the regression line rounded to three decimal places.

## Output Format

- The output is a single floating-point number rounded to three decimal places.
- Example (not the actual answer):

  ```
  0.255
  ```

Make sure not to include any leading or trailing spaces in your answer.

