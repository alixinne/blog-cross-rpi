use color_eyre::eyre::Result;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg_attr(feature = "python", pyclass)]
#[derive(Default, Debug, Clone)]
struct User {
    name: String,
    age: f64,
}

/// Get users from the SQLite database
#[cfg(feature = "sql")]
fn get_users() -> Result<Vec<User>> {
    use sqlite::State;

    // From the sqlite crate example
    let connection = sqlite::open(":memory:")?;

    connection.execute(
        "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42.5);
        INSERT INTO users VALUES ('Bob', 69.69);
        ",
    )?;

    let mut statement = connection.prepare("SELECT * FROM users")?;
    let mut result = Vec::with_capacity(2);

    while let State::Row = statement.next()? {
        result.push(User {
            name: statement.read(0)?,
            age: statement.read(1)?,
        });
    }

    Ok(result)
}

/// Get users from memory
#[cfg(not(feature = "sql"))]
fn get_users() -> Result<Vec<User>> {
    Ok(vec![
        User {
            name: "StaticAlice".into(),
            age: 42.5,
        },
        User {
            name: "StaticBob".into(),
            age: 69.69,
        },
    ])
}

/// Print user names using Python
#[cfg(feature = "python")]
fn python_print(users: Vec<User>) -> Result<()> {
    use pyo3::types::*;

    #[pymethods]
    impl User {
        #[getter]
        fn name(&self) -> &str {
            self.name.as_str()
        }
    }

    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        locals.set_item(
            "users",
            PyList::new(
                py,
                users.into_iter().map(|user| PyCell::new(py, user).unwrap()),
            ),
        )?;

        py.run("print([user.name for user in users])", None, Some(&locals))?;

        Ok(())
    })
}

#[cfg(not(feature = "python"))]
fn python_print(_users: Vec<User>) -> Result<()> {
    println!("Python feature disabled.");
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Get users
    let users = get_users()?;

    // Print users for debugging
    println!(
        "Read users from {}: {:#?}",
        if cfg!(feature = "sql") {
            "sqlite"
        } else {
            "static data"
        },
        users
    );

    python_print(users)
}
