
// Builder Design Pattern (Creational)
#[derive(Debug, PartialEq)]
pub struct Person {
    name: String,
    surname: String,
    age: u8,
}

impl Person {
    pub fn builder() -> PersonBuilder {
        PersonBuilder::default()
    }
}

#[derive(Default)]
pub struct PersonBuilder {
    name: String,
    surname: String,
    age: u8,
}

impl PersonBuilder {
    pub fn new() -> PersonBuilder {
        PersonBuilder {
            name: String::from(""),
            surname: String::from(""),
            age: 0,
        }
    }

    pub fn name(mut self, name: String) -> PersonBuilder {
        self.name = name;
        self
    }

    pub fn surname(mut self, surname: String) -> PersonBuilder {
        self.surname = surname;
        self
    }

    pub fn age(mut self, age: u8) -> PersonBuilder {
        self.age = age;
        self
    }

    pub fn build(self) -> Person {
        Person {   name: self.name,
                surname: self.surname,
                age: self.age,
        }
    }
}


#[test]
fn builder_test() {
    let person = Person {
        name: String::from("Innova"),
        surname: String::from("Andoni"),
        age: 41,
    };
    let person_from_builder: Person = PersonBuilder::new()
                                        .name(String::from("Innova"))
                                        .surname(String::from("Andoni"))
                                        .age(41)
                                        .build();
    assert_eq!(person, person_from_builder);
}