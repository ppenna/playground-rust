// An enum is a type formed from multiple disjoint possibilities.

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DayData {
    Mon,
    Tue,
    Wed,
    Thu(usize), // class time
    Fri,
}

pub fn format_day_data(d: &DayData) -> String {
    // Different ways to go from &str to String:
    // "mon".to_owned()
    // "mon".to_string()
    // format!("mon")
    // Clippy discourages the last of these.

    match d {
        DayData::Mon => "mon".to_owned(),
        DayData::Tue => "tue".to_owned(),
        DayData::Wed => String::from("wed"),
        DayData::Thu(x) => format!("thu -- class at {}!", x),
        DayData::Fri => "fri".to_owned(),
    }
}

pub fn get_current_day(day: usize) -> DayData {
    if day == 4 {
        DayData::Thu(2)
    } else if day == 1 {
        DayData::Mon
    } else {
        DayData::Fri
    } // etc.
}

#[test]
pub fn test_day() {
    let day: DayData = get_current_day(4);
    assert_eq!(format_day_data(&day), String::from("thu -- class at 2!"),);
}

// But often it's more typical in Rust to do things in a slightly more
// object-oriented looking form by implementing Traits:

// Common traits in Rust: Debug, Clone, Eq, PartialEq, ...
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DayData2 {
    Mon,
    Tue,
    Wed,
    Thu(usize), // class time
    Fri,
    OtherDayData(DayData),
}
impl DayData2 {
    pub fn format_day(&self) -> String {
        // means the same thing as fn format_day(self: &DayData2) -> String.
        // Don't have to implement the formatting myself because we derive(Debug)
        format!("{:?}", self) // relying on Debug
    }
    pub fn duplicate(&self) -> (DayData2, DayData2) {
        (self.clone(), self.clone()) // relying on Clone
    }
    pub fn check_all_eq(&self, other1: &DayData2, other2: &DayData2) -> bool {
        self == other1 && self == other2 // relying on PartialEq, Eq
    }
}

// We will see a similar code structure / syntax for structs.
// structs and enums are the basic building blocks of defining
// datatypes in Rust.

// A special enum type: Option!
// It's equivalent to:
pub enum OptionString {
    SomeString(String),
    NoString,
}
impl OptionString {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            OptionString::SomeString(s) => s.clone(),
            OptionString::NoString => "".to_owned(),
        }
    }
    pub fn from_string(s: String) -> Self {
        OptionString::SomeString(s)
    }
    // also note:
    pub fn do_some_mutation(&mut self) {
        // if you want mutable access, the "&mut self" is equivalent to
        // 'self: &mut Self' or 'self: &mut OptionString'.
        // new syntax that is useful: 'if let':
        if let OptionString::SomeString(_s) = self {
            *self = OptionString::NoString
        }
    }
}

// Rust has options types, an enumeration which is either Some(val) or None:

// syntax: Option<T> means optional of type T.
pub fn take_option(val: Option<i32>) {
    match val {
        Some(x) => println!("Option contained: {}", x),
        None => println!("Nothing in option"),
    }

    // Similarish to:
    println!("Option: {:?}", val);
}

// Useful for taking optional values. Can represent functions which may only
// return a value sometimes:
// Rough type of get function for HashTable:

// fn get(&self, k: &Q) -> Option<&V>

// If element is missing returns none.

// Rust does not have exceptions.
// Options must be explicitly matched: compiler enforces this, no more null pointer
// exceptions :)

// In Java similar method may look like:

// If value is missing returns Null
// V get(Q k)
