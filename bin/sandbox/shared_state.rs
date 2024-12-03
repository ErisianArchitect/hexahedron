pub fn experiment() {
    let mut injector1 = ArgInjector::new((
            1234i32,
            String::from("Hello, world!")
        ),
        |num: &mut i32, text: &mut String, age: &u8, name: &String| {
            println!("***\nNum: {num}\nText: {text}\nName: {name}\nAge: {age}\n***");
            *num += 1;
        });
    let mut injector2 = ArgInjector::new((
            420i32,
            69i32,
            1337i32
        ),
        |x: &mut i32, y: &mut i32, z: &mut i32, name: &String, occupation: &Occupation| {
            println!("({x}, {y}, {z}): {name}, {occupation:#?}");
            *z += 1;
        }
    );
    fn take_injector<Inject, Args, Injector: ArgInject<Inject, Args>>(injector: &mut Injector) {
        let mut state = State::new("Harold", 84, Occupation::Programmer);
        injector.exec(&state);
    }
    take_injector(&mut injector1);
    take_injector(&mut injector1);
    take_injector(&mut injector2);
    take_injector(&mut injector2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Occupation {
    Programmer,
    Artist,
    Musician,
    SoundDesigner,
    GameDesigner,
    Writer,
}

pub struct State {
    name: String,
    age: u8,
    occupation: Occupation,
}

impl State {
    pub fn new<Name: Into<String>>(name: Name, age: u8, occupation: Occupation) -> Self {
        Self {
            name: name.into(),
            age,
            occupation,
        }
    }
}

pub trait StateFilter {
    fn get_value(state: &State) -> &Self;
}

struct StateName<'a>(&'a String);
struct StateAge;
struct StateOccupation;

impl StateFilter for String {
    fn get_value(state: &State) -> &Self {
        &state.name
    }
}

impl StateFilter for u8 {
    fn get_value(state: &State) -> &Self {
        &state.age
    }
}

impl StateFilter for Occupation {
    fn get_value(state: &State) -> &Self {
        &state.occupation
    }
}

pub struct ArgInjector<Args, F> {
    args: Args,
    function: F,
}

impl<Args, F> ArgInjector<Args, F> {
    pub fn new(args: Args, function: F) -> Self {
        Self {
            args,
            function,
        }
    }
}

pub trait ArgInject<Inject, Args> {
    fn exec(&mut self, state: &State);
}

impl<I0, I1, T0: StateFilter, T1: StateFilter, F: Fn(&mut I0, &mut I1, &T0, &T1)> ArgInject<(I0, I1), (T0, T1)> for ArgInjector<(I0, I1), F> {
    fn exec(&mut self, state: &State) {
        let (
            arg0,
            arg1,
        ) = &mut self.args;
        let (
            arg2,
            arg3,
        ) = (
            T0::get_value(state),
            T1::get_value(state),
        );
        (self.function)(arg0, arg1, arg2, arg3);
    }
}

impl<I0, I1, I2, T0: StateFilter, T1: StateFilter, F: Fn(&mut I0, &mut I1, &mut I2, &T0, &T1)> ArgInject<(I0, I1, I2), (T0, T1)> for ArgInjector<(I0, I1, I2), F> {
    fn exec(&mut self, state: &State) {
        let (
            arg0,
            arg1,
            arg2,
        ) = &mut self.args;
        let (
            arg3,
            arg4,
        ) = (
            T0::get_value(state),
            T1::get_value(state),
        );
        (self.function)(arg0, arg1, arg2, arg3, arg4);
    }
}