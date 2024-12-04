use paste::paste;
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JobTitle {
    Programmer,
    Artist,
    Designer,
    Writer,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Clearance {
    #[default]
    Level0,
    Level1,
    Level2,
    Restricted,
}

pub struct Context {
    name: String,
    job: JobTitle,
    clearance: Clearance,
}

impl Context {
    pub fn new<S: Into<String>>(name: S, job: JobTitle, clearance: Clearance) -> Self {
        Self {
            name: name.into(),
            job,
            clearance,
        }
    }
}

pub trait Executor {
    fn exec(&mut self, context: &Context);
}

pub trait FieldGetter {
    fn get_field(context: &Context) -> Self;
}

impl FieldGetter for String {
    fn get_field(context: &Context) -> Self {
        context.name.clone()
    }
}

impl FieldGetter for JobTitle {
    fn get_field(context: &Context) -> Self {
        context.job
    }
}

impl FieldGetter for Clearance {
    fn get_field(context: &Context) -> Self {
        context.clearance
    }
}

pub struct FieldInjector<Data: 'static, Args: 'static, F: 'static>
where
    Self: Executor {
    phantom: PhantomData<(Args)>,
    data: Data,
    function: F,
}

impl<Data: 'static, Args: 'static, F: 'static> FieldInjector<Data, Args, F>
where
    Self: Executor {
    #[inline]
    pub fn new(data: Data, function: F) -> Self {
        Self {
            phantom: PhantomData,
            data,
            function,
        }
    }
}

// Data: (T0, T1, T2), Inject: (I0: Inject<T0>, I1: Inject<T1>, I2: Inject<T2>), F: Fn(I0, I1, I2)

macro_rules! make_executor {
    (($($dt:ident),*$(,)?), ($($tt:ident),*$(,)?)) => {
        paste! {
            #[allow(non_snake_case)]
            impl<$($dt,)* $($tt,)* F> Executor for FieldInjector<($($dt,)*), ($($tt,)*), F>
                where
                $(
                    $tt: FieldGetter,
                )*
                F: Fn($(&mut $dt,)* $($tt,)*) {
                    #[inline]
                    fn exec(&mut self, context: &Context) {
                        let (
                            $(
                                [<_ $dt>],
                            )*
                        ) = &mut self.data;
                        (self.function)(
                            $(
                                [<_ $dt>],
                            )*
                            $(
                                $tt::get_field(context),
                            )*
                        );
                    }
                }
        }
    };
}

make_executor!((), ());
make_executor!((), (T0));
make_executor!((), (T0, T1));
make_executor!((), (T0, T1, T2));
make_executor!((D0), ());
make_executor!((D0), (T0));
make_executor!((D0), (T0, T1));
make_executor!((D0), (T0, T1, T2));
make_executor!((D0, D1), ());
make_executor!((D0, D1), (T0));
make_executor!((D0, D1), (T0, T1));
make_executor!((D0, D1), (T0, T1, T2));
make_executor!((D0, D1, D2), ());
make_executor!((D0, D1, D2), (T0));
make_executor!((D0, D1, D2), (T0, T1));
make_executor!((D0, D1, D2), (T0, T1, T2));
make_executor!((D0, D1, D2, D3), ());
make_executor!((D0, D1, D2, D3), (T0));
make_executor!((D0, D1, D2, D3), (T0, T1));
make_executor!((D0, D1, D2, D3), (T0, T1, T2));

fn inject<Data, Args, F>(data: Data, f: F) -> FieldInjector<Data, Args, F>
where
    FieldInjector<Data, Args, F>: Executor {
        FieldInjector { phantom: PhantomData, data, function: f }
}

#[test]
fn experiment() {

    let mut ctx = Context::new("Jorgathon Nafaniels", JobTitle::Artist, Clearance::Restricted);

    let mut executors = Vec::<Box<dyn Executor>>::new();

    struct Invoker {
        executors: Vec<Box<dyn Executor>>,
    }

    impl Invoker {
        pub fn new() -> Self {
            Self {
                executors: Vec::new(),
            }
        }

        pub fn push<Args: 'static, F: 'static>(&mut self, function: F)
        where FieldInjector<(), Args, F>: Executor {
            self.executors.push(Box::new(FieldInjector::new((), function)));
        }

        pub fn push_with_data<Data: 'static, Args: 'static, F: 'static>(&mut self, data: Data, function: F)
        where FieldInjector<Data, Args, F>: Executor {
            self.executors.push(Box::new(FieldInjector::new(data, function)));
        }

        pub fn invoke(&mut self, context: &Context) {
            self.executors.iter_mut().for_each(|exec| {
                exec.exec(context);
            });
        }
    }


    let inv = Invoker {
        executors: Vec::new(),
    };


    fn take_injector<Data: 'static, Args: 'static, F: 'static>(ctx: &Context, data: Data, mut f: F)
    where FieldInjector<Data, Args, F>: Executor {
        FieldInjector::new(data, f).exec(ctx);
    }

    struct Foo {
        bar: String,
        baz: JobTitle,
    }

    impl Foo {
        fn new<S: Into<String>>(bar: S, baz: JobTitle) -> Self {
            Self {
                bar: bar.into(),
                baz,
            }
        }

        fn fnord(&mut self, baz: JobTitle) {
            self.foo();
            self.baz = baz;
            self.foo();
        }

        fn foo(&self) {
            println!("Foo::foo() = {:#?}", self.baz);
        }
    }

    take_injector(
    &ctx,
    (
        Foo::new("Boo", JobTitle::Programmer),
        1234i32,
    ),
    |foo: &mut Foo, num: &mut i32, job: JobTitle| {

    });

    // take_injector(&ctx, (), |name: String, job: JobTitle| {
    //     println!("Name: {name}, Job: {job:#?}");
    // });

    // take_injector(&ctx, (), |name: String, job: JobTitle, clearance: Clearance| {
    //     println!("Name: {name}, Job: {job:#?}, Clearance: {clearance:#?}");
    // });
}