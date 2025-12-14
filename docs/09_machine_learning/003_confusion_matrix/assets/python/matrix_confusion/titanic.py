import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.impute import SimpleImputer
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import confusion_matrix, ConfusionMatrixDisplay

# Mandatory when running in a terminal
import matplotlib.pyplot as plt

df = pd.read_csv("data/titanic.csv")
# print("df datatype & shape :", type(df), df.shape)

# In a script one must use print to see the head of dataframe
print("\n\nDataset head:")
print(df.head())


# Remove `PassengerId`, `Name`, `Ticket`, `Cabin` columns from the dataset
df = df.drop(columns=["PassengerId", "Name", "Ticket", "Cabin"])

print("\n\nDataset head (useless col removed) :")
print(df.head())

# ## Preprocessing
# No EDA? What a shame!
# Split the dataset by $X$ and
# $y$ = Survived
# $X$ = Pclass	Sex	Age	SibSp	Parch	Fare	Embarked

y = df.loc[:, "Survived"]

features_list = ["Pclass", "Sex", "Age", "SibSp", "Parch", "Fare", "Embarked"]
X = df.loc[:, features_list]

# Split the data in `train` and `test` sets
X_train, X_test, y_train, y_test = train_test_split(
    # `stratify=y` allows to stratify our sample.
    # Meaning, we will have the same proportion of categories in test and train set
    X,
    y,
    test_size=0.2,
    random_state=0,
    stratify=y,
)


# Deal with missing values with `SimpleImputer`
# Create an imputer for numerical columns
numerical_imputer = SimpleImputer(strategy="mean")

# Apply it on "Age" column.
# ! See the X[["Age"]] to get a 2D array rather than 1D
X_train[["Age"]] = numerical_imputer.fit_transform(X_train[["Age"]])

# In col `Embarked` replace missing val with "Unknown"
categorical_imputer = SimpleImputer(strategy="constant", fill_value="Unknown")
X_train[["Embarked"]] = categorical_imputer.fit_transform(X_train[["Embarked"]])

# Make all the required preprocessing on the train set
print("\n\nX_train head:")
print(X_train.head())

# Reminder, we have : ['Pclass', 'Sex', 'Age', "SibSp", "Parch", "Fare", "Embarked"]
numeric_features = [0, 2, 3, 4, 5]
numeric_transformer = StandardScaler()

categorical_features = [1, 6]
categorical_transformer = OneHotEncoder()

# No change in score with or without drop=first
# TODO: I think it's better without because in `LogisticRegression` there is an l2-type regulation/penalty
# categorical_transformer = OneHotEncoder(drop="first")

# Apply ColumnTransformer to create a pipeline that will apply the above preprocessing
feature_encoder = ColumnTransformer(
    transformers=[
        ("cat", categorical_transformer, categorical_features),
        ("num", numeric_transformer, numeric_features),
    ]
)

X_train = feature_encoder.fit_transform(X_train)
print("\n\nX_train fit_transformed head:")
print(X_train[0:5, :].round(2))  # print first 5 rows (not using iloc since now X_train became a numpy array)

# Build the Logistic Regression model
classifier = LogisticRegression(random_state=0)  # Instantiate model
classifier.fit(X_train, y_train)  # Fit model. Ajustement

y_train_pred = classifier.predict(X_train)
print(f"\n\ny_train predictions head: {y_train_pred[0:5]}")

# Evaluate the model but preprocess `X_test` first
X_test[["Age"]] = numerical_imputer.transform(X_test[["Age"]])
X_test[["Embarked"]] = categorical_imputer.transform(X_test[["Embarked"]])
X_test = feature_encoder.transform(X_test)

y_test_pred = classifier.predict(X_test)
print(f"y_test predictions head: {y_test_pred[0:5]}")

# Create the confusion matrix with `plot_confusion_matrix`
cm = confusion_matrix(y_train, y_train_pred, labels=classifier.classes_)
cm_display = ConfusionMatrixDisplay.from_predictions(y_train, y_train_pred)
cm_display.ax_.set_title("Confusion matrix on train set ")

# Mandatory when running in a terminal
plt.show()

print(f"\n\nAccuracy-score on train set : {classifier.score(X_train, y_train):.3f}")

cm = confusion_matrix(y_test, y_test_pred, labels=classifier.classes_)
cm_display = ConfusionMatrixDisplay.from_predictions(y_test, y_test_pred)
cm_display.ax_.set_title("Confusion matrix on test set ")

# Mandatory when running in a terminal
plt.show()

print(f"Accuracy-score on test set : {classifier.score(X_test, y_test):.3f}")
