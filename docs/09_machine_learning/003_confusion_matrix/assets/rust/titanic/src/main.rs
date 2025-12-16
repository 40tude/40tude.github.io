// Rust guideline compliant 2025-12-15

use csv::Reader;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{s, Array1, Array2, Axis};
use serde::Deserialize;

/// Passenger record from Titanic dataset
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Passenger {
    #[serde(rename = "PassengerId")]
    passenger_id: u32,

    #[serde(rename = "Survived")]
    survived: u8,

    #[serde(rename = "Pclass")]
    pclass: u8,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Sex")]
    sex: String,

    #[serde(rename = "Age")]
    age: Option<f64>,

    #[serde(rename = "SibSp")]
    sibsp: Option<u8>,

    #[serde(rename = "Parch")]
    parch: Option<u8>,

    #[serde(rename = "Ticket")]
    ticket: String,

    #[serde(rename = "Fare")]
    fare: f64,

    #[serde(rename = "Cabin")]
    cabin: Option<String>,

    #[serde(rename = "Embarked")]
    embarked: Option<String>,
}

/// Loads Titanic dataset and returns features matrix X and labels y.
///
/// Features: Pclass, Sex, Age, SibSp, Parch, Fare, Embarked (encoded)
///
/// This function mimics the Python preprocessing:
/// - Drops PassengerId, Name, Ticket, Cabin columns
/// - Keeps rows with missing values (imputation done separately)
fn load_titanic(path: &str) -> (Vec<Passenger>, Array1<u8>) {
    let mut rdr = Reader::from_path(path).expect("Failed to read CSV file");

    let mut passengers = Vec::new();
    let mut labels = Vec::new();

    for result in rdr.deserialize::<Passenger>() {
        let p = result.expect("Failed to deserialize passenger");
        labels.push(p.survived);
        passengers.push(p);
    }

    let y = Array1::from(labels);
    (passengers, y)
}

/// Computes mean of non-missing Age values.
fn compute_age_mean(passengers: &[Passenger]) -> f64 {
    let ages: Vec<f64> = passengers.iter().filter_map(|p| p.age).collect();
    let sum: f64 = ages.iter().sum();
    sum / ages.len() as f64
}

/// Imputes missing Age values with mean and missing Embarked with "Unknown".
fn impute_missing(passengers: &mut [Passenger], age_mean: f64) {
    for p in passengers.iter_mut() {
        if p.age.is_none() {
            p.age = Some(age_mean);
        }
        if p.sibsp.is_none() {
            p.sibsp = Some(0);
        }
        if p.parch.is_none() {
            p.parch = Some(0);
        }
        if p.embarked.is_none() {
            p.embarked = Some(String::from("Unknown"));
        }
    }
}

/// One-hot encodes categorical variable.
///
/// Returns (encoded_features, categories) where categories maps index to category name.
fn one_hot_encode(values: &[String]) -> (Array2<f64>, Vec<String>) {
    let mut categories: Vec<String> = values.iter().cloned().collect();
    categories.sort();
    categories.dedup();

    let n_samples = values.len();
    let n_categories = categories.len();
    let mut encoded = Array2::zeros((n_samples, n_categories));

    for (i, val) in values.iter().enumerate() {
        let cat_idx = categories.iter().position(|c| c == val).unwrap();
        encoded[[i, cat_idx]] = 1.0;
    }

    (encoded, categories)
}

/// Converts passengers to feature matrix with preprocessing.
///
/// Features order: Pclass, Sex (one-hot), Age, SibSp, Parch, Fare, Embarked (one-hot)
fn passengers_to_features(passengers: &[Passenger]) -> Array2<f64> {
    // Extract categorical features
    let sex_values: Vec<String> = passengers.iter().map(|p| p.sex.clone()).collect();
    let embarked_values: Vec<String> = passengers
        .iter()
        .map(|p| {
            p.embarked
                .clone()
                .unwrap_or_else(|| String::from("Unknown"))
        })
        .collect();

    // One-hot encode
    let (sex_encoded, _) = one_hot_encode(&sex_values);
    let (embarked_encoded, _) = one_hot_encode(&embarked_values);

    // Extract numeric features
    let n_samples = passengers.len();
    let mut numeric_features = Array2::zeros((n_samples, 5)); // Pclass, Age, SibSp, Parch, Fare

    for (i, p) in passengers.iter().enumerate() {
        numeric_features[[i, 0]] = f64::from(p.pclass);
        numeric_features[[i, 1]] = p.age.unwrap_or(0.0);
        numeric_features[[i, 2]] = f64::from(p.sibsp.unwrap_or(0));
        numeric_features[[i, 3]] = f64::from(p.parch.unwrap_or(0));
        numeric_features[[i, 4]] = p.fare;
    }

    // Concatenate: Sex (one-hot) + numeric + Embarked (one-hot)
    // This matches Python's ColumnTransformer order: cat, num
    let mut features = Array2::zeros((
        n_samples,
        sex_encoded.ncols() + numeric_features.ncols() + embarked_encoded.ncols(),
    ));

    let mut col_idx = 0;

    // Add sex one-hot
    for col in 0..sex_encoded.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&sex_encoded.column(col));
        col_idx += 1;
    }

    // Add embarked one-hot
    for col in 0..embarked_encoded.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&embarked_encoded.column(col));
        col_idx += 1;
    }

    // Add numeric features
    for col in 0..numeric_features.ncols() {
        features
            .slice_mut(s![.., col_idx])
            .assign(&numeric_features.column(col));
        col_idx += 1;
    }

    features
}

/// Standardizes features (zero mean, unit variance).
///
/// Returns (standardized_features, mean, std) for later use on test set.
fn standardize_features(x: &Array2<f64>) -> (Array2<f64>, Array1<f64>, Array1<f64>) {
    let mean = x.mean_axis(Axis(0)).unwrap();
    let std = x.std_axis(Axis(0), 0.0);

    let mut x_scaled = x.clone();
    for i in 0..x.ncols() {
        let std_val = if std[i] > 1e-10 { std[i] } else { 1.0 };
        x_scaled
            .column_mut(i)
            .mapv_inplace(|v| (v - mean[i]) / std_val);
    }

    (x_scaled, mean, std)
}

/// Applies standardization using pre-computed mean and std.
fn apply_standardization(x: &Array2<f64>, mean: &Array1<f64>, std: &Array1<f64>) -> Array2<f64> {
    let mut x_scaled = x.clone();
    for i in 0..x.ncols() {
        let std_val = if std[i] > 1e-10 { std[i] } else { 1.0 };
        x_scaled
            .column_mut(i)
            .mapv_inplace(|v| (v - mean[i]) / std_val);
    }
    x_scaled
}

/// Splits data into train and test sets.
///
/// Uses simple 80/20 split (random_state=0 equivalent: first 80% train, last 20% test).
fn train_test_split<T: Clone>(
    x: Array2<f64>,
    y: Array1<T>,
    test_size: f64,
) -> (Array2<f64>, Array2<f64>, Array1<T>, Array1<T>) {
    let n_samples = x.nrows();
    let n_test = (n_samples as f64 * test_size).round() as usize;
    let n_train = n_samples - n_test;

    let x_train = x.slice(s![0..n_train, ..]).to_owned();
    let x_test = x.slice(s![n_train.., ..]).to_owned();
    let y_train = y.slice(s![0..n_train]).to_owned();
    let y_test = y.slice(s![n_train..]).to_owned();

    (x_train, x_test, y_train, y_test)
}

/// Computes confusion matrix.
///
/// Returns [[TN, FP], [FN, TP]]
fn confusion_matrix(y_true: &Array1<u8>, y_pred: &Array1<u8>) -> [[usize; 2]; 2] {
    let mut cm = [[0usize; 2]; 2];

    for (t, p) in y_true.iter().zip(y_pred.iter()) {
        cm[*t as usize][*p as usize] += 1;
    }

    cm
}

/// Computes accuracy score.
fn accuracy_score(y_true: &Array1<u8>, y_pred: &Array1<u8>) -> f64 {
    let correct = y_true
        .iter()
        .zip(y_pred.iter())
        .filter(|(t, p)| t == p)
        .count();
    correct as f64 / y_true.len() as f64
}

fn main() {
    println!("Loading Titanic dataset...\n");

    // Load dataset
    let (mut passengers, y) = load_titanic("data/titanic.csv");

    println!("Dataset shape: {} passengers\n", passengers.len());
    println!("First 5 passengers (before preprocessing):");
    for (i, p) in passengers.iter().take(5).enumerate() {
        println!("{}: {:?}", i, p);
    }

    // Remove useless columns (already done by not including them in features)
    // Columns removed: PassengerId, Name, Ticket, Cabin

    // Preprocessing: impute missing values
    let age_mean = compute_age_mean(&passengers);
    println!("\n\nAge mean for imputation: {:.2}", age_mean);

    impute_missing(&mut passengers, age_mean);

    // Convert to feature matrix
    let x = passengers_to_features(&passengers);
    println!("\nFeature matrix shape: {:?}", x.dim());
    println!("First 5 rows (after encoding):");
    println!("{:.2}", x.slice(s![0..5, ..]));

    // Split into train and test sets (80/20)
    let (x_train, x_test, y_train, y_test) = train_test_split(x, y, 0.2);

    println!(
        "\n\nTrain set: {} samples, Test set: {} samples",
        x_train.nrows(),
        x_test.nrows()
    );

    // Standardize features (fit on train, apply on test)
    let (x_train_scaled, mean, std) = standardize_features(&x_train);
    let x_test_scaled = apply_standardization(&x_test, &mean, &std);

    println!("\nStandardized train set (first 5 rows):");
    println!("{:.2}", x_train_scaled.slice(s![0..5, ..]));

    // Build and train Logistic Regression model
    println!("\n\nTraining Logistic Regression model...");
    let dataset = Dataset::new(x_train_scaled.clone(), y_train.clone());

    let model = LogisticRegression::default()
        .max_iterations(100)
        .fit(&dataset)
        .expect("Failed to fit model");

    // Predictions on train set
    let y_train_pred = model.predict(&x_train_scaled);
    println!(
        "\nTrain predictions (first 5): {:?}",
        y_train_pred.slice(s![0..5])
    );

    // Confusion matrix on train set
    let cm_train = confusion_matrix(&y_train, &y_train_pred);
    println!("\n\nConfusion Matrix on train set:");
    println!("TN: {}  FP: {}", cm_train[0][0], cm_train[0][1]);
    println!("FN: {}  TP: {}", cm_train[1][0], cm_train[1][1]);

    let acc_train = accuracy_score(&y_train, &y_train_pred);
    println!("Accuracy on train set: {:.3}", acc_train);

    // Predictions on test set
    let y_test_pred = model.predict(&x_test_scaled);
    println!(
        "\n\nTest predictions (first 5): {:?}",
        y_test_pred.slice(s![0..5])
    );

    // Confusion matrix on test set
    let cm_test = confusion_matrix(&y_test, &y_test_pred);
    println!("\nConfusion Matrix on test set:");
    println!("TN: {}  FP: {}", cm_test[0][0], cm_test[0][1]);
    println!("FN: {}  TP: {}", cm_test[1][0], cm_test[1][1]);

    let acc_test = accuracy_score(&y_test, &y_test_pred);
    println!("Accuracy on test set: {:.3}", acc_test);
}
