use std::fmt;


#[derive(Debug)]
pub enum Installable {
    Flake(FlakeInstallable),
}

impl fmt::Display for Installable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Installable::Flake(flake) => {
                write!(f, "{}", flake.reference)?;

                if !flake.attribute.is_empty() {
                    write!(f, "#")?;

                    let mut first = true;

                    for elem in &flake.attribute {
                        if !first {
                            write!(f, ".")?;
                        }

                        if elem.contains('.') {
                            write!(f, r#""{}""#, elem)?;
                        } else {
                            write!(f, "{}", elem)?;
                        }

                        first = false;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FlakeInstallable {
    pub reference: String,
    pub attribute: Vec<String>,
}

impl Installable {
    pub fn flake<S>(reference: S, attribute: &[S]) -> Self
    where
        S: AsRef<str>,
    {
        Installable::Flake(FlakeInstallable {
            reference: reference.as_ref().to_string(),
            attribute: attribute.iter().map(|s| s.as_ref().to_string()).collect(),
        })
    }
}

#[test]
fn test_display() {
    let installable = Installable::flake(".", &["foo", "bar.local", "baz"]);

    let displayed = format!("{}", installable);
    assert_eq!(displayed, r#".#foo."bar.local".baz"#);
}
