


use crate::*;



async fn test(){

    async fn func(){}
    type Type = Box<dyn std::future::Future<Output=()> + Send + Sync + 'static>;
    struct Generic<'lifetmie, Type>{
        pub data: &'lifetmie mut Type // mutating mutable pointer mutates the underlying data too
    }
    let mut instance = Generic{
        /*  
            future objects must be pinned into the ram to get solved later
            and since they're trait objects which are stored on the heap 
            their pointer which is Box<dyn Future> must be pinned into the 
            ram like Box::pin(async move{})

            to have future objects as a type which are of type Future trait we have to
            put them behind a pointer and pin the pointer into the ram to get their result
            in later scopes by awaiting on them which actually will unpin their pointer,
            we can't use Box::new(async move{()}) if we want to access the result of the 
            future outside of the Boxed scope to solve this we must pin the boxed value 
            which in this case is pinning the pointer to the Future trait, and put an await
            on that in later scopes to unpin the boxed value from the ram to get the result
            of the future object

            since Future trait doesn't implement Unpin trait thus we can pin the boxed 
            type into the ram by constructing a new Pin<Box<Type>>. then Type will be 
            pinned in memory and unable to be moved.
        */
        data: &mut Box::pin(func()) // passing the result of calling async func to the pinned box
    };
    let unpinned_boxed = instance.data.await;
    /*  
        moving type can also be dereferencing the type which converts
        the pointer into the owned value but based on the fact that 
        if the type is behind a pointer we can't move it! so we can't
        deref the pinned boxed in here, we must clone it or borrow it 
        which clone is not working in here because Clone it's not 
        implemented for &mut Type which is the type of data field
    */
    // let deref_boxed = *instance.data;
    instance.data = &mut Box::pin(func()); // passing the result of calling async func to the pinned box

    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- Generic Fns =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
    type Nothing = ();
    struct Information<'bl, B, F1: Fn() -> (), 
                F2: FnMut() -> &'bl B + Send + Sync + 'static, 
                T, F3 = fn() -> Nothing> // here T can be &[u8]
        where B: Send + Sync + 'static{

        f1: F1,
        f2: F2,
        f3: F3,
        lost_bits: T,
        entropy: &'bl B

    }

    /* -------------------------------------------------------------------------------------- 

        Closure Case: In the closure example, the closure captures the environment in 
            which it's defined. This includes the value 34, which is actually stored 
            within the closure itself. As a result, the reference &34 is valid for 
            the lifetime of the closure.

        Function Case: In the function example, there's no environment to capture.
            The value 45 is a temporary value created within the function and destroyed 
            when the function returns. Consequently, you can't safely return a reference to it.
        
        When we try to return a reference to a temporary value, we'll usually encounter a 
        compilation error stating that you're trying to return a reference to a local variable, 
        which is deallocated after the function returns. In summary, closures can capture 
        and own their environment, so they can safely return references to it. Functions can't 
        do that, so returning a reference to a temporary value is not safe.

        in other words returning reference from the closure is safe cause they'll capture the
        type into their own scope which allows us to borrow them thus return a pointer to them
        but for the functions, since there is no capturing process during the execution thus
        everything will be created inside the function body and is limited to that scope, so 
        we can't return a reference (without valid lifetime like static or the lifetime of self)
        to the type cause there is no value for it to be borrowed from because local types inside
        the function will be dropped once the function gets executed on the stack.
    
     -------------------------------------------------------------------------------------- */
    let cls = ||{
        &34
    };

    // fn ret_i32() -> &i32{
    //     &45
    // }

    fn amethod(){}
    let instance = Information::<'_, i32, _, _, &[u8], _>{
        f1: ||{},
        f2: cls,
        f3: amethod,
        lost_bits: &[1],
        entropy: &32
    };
    let new_instance = Information::<'_, i32, _, _, &[u8], _>{
        ..instance // filling the rest with the old ones
    };
    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=


    /* an inline returning from a closure example */
    let cls = |name: &str| match name{
        "wildonion" => {
            1
        },
        _ => {
            2
        }
    };

    /* inline creating and calling */
    (
        |name: &str| match name{
            "wildonion" => {
                1
            },
            _ => {
                2
            }
        }
    )("wildonion");

    let boxed_cls = Box::new(cls);
    let unboxed_cls = *boxed_cls; /* we can unbox it using * since box is an smart pointer */
    let called_unboxed_cls = unboxed_cls("wildonion");
    let called_unboxed_cls = (*boxed_cls)("wildonion");


    const CONST: () = ();
    let traits_slice: &[&dyn Fn()];
    let traits_slices: &[&[Box<dyn Fn()>]];
    let traits_slices_and_dyn: &[&[&dyn Fn()]];
    type CallbackHell = fn(Callback) -> u8;
    struct Callback;
    impl Interface for Callback{
        fn check(&mut self){}
    }
    impl Callback{
        pub fn run(&self){}
    }

    struct Wannet<'a, G: 'a + Fn() -> Callback, T = fn(Callback) -> String>{
        pub data: &'a G, /* if we want to have pointer, lifetime is required */
        pub info: T
    }

    /* path param can be any type bounded to the following traits */
    fn run<T: Send + ?Sized>(path: impl Send + Sync + 'static + Sized) 
        -> impl FnOnce() -> &'static str + Send{

        ||{
            let name = "";
            let boxed_name: &'static mut str = Box::leak(name.into()); /* converting the &str to &'static str by leaking the memory of the String */
            boxed_name
        }

    }
    /* the run() method param can be any type which is bounded to Send + Sync + 'static + Sized */
    run::<str>("src"); /* since ?Sized is implemeneted for the generic thus we can use str unless we have to use &str */
    /* run() method has a parameter of type Calleback */
    run::<Callback>(Callback{});

    impl<'a, G: Fn() -> Callback, T> Wannet<'a, G, T>{ 
        /* 
            a mutable pointer to self lets us to have the instance later on 
            in other scopes when we call its methods on it since it's behind 
            a pointer its ownership won't be lost
        */
        fn implement(&mut self) -> impl Interface{
            Callback
        }

        fn ret_func(f: T) where T: FnMut(){

            /* 
                basically we have to put the slice types and dynamic types like traits behind pointer 
                like &[u8] and &dyn Trait or use Box for traits, to have them as types in scopes, cause 
                they’re not sized and for traits we can use them as the followings
                    - return type of methods like -> impl Trait or Future
                    - return type of closure or function to to lifetimes and traits like -> R + Send + Sync + 'static
                    - method param type bounded to that trait like using some_param: impl Trait or Future
                    - method param type like put them behind pointer
                        - Box<dyn Fn() or Future>, 
                        - &dyn Fn() or Future
                    - in method and struct signatures like bounding generic to traits using where G: Trait
            */

            fn implement_<G, R>(mut g: impl Interface) 
                where G: 'static + Send + Sync + FnMut() -> R + Send + Sync + 'static,
                      R: Send + Sync + 'static{ /* the type of g is an structure or a type that implements the Interface */
                    g.check(); /* check method is mutable thus g must be mutable */
            } 

            fn run_fut<G: Into<Callback>>(gen: impl Into<Callback> + AsRef<Callback>, 
                fut: impl std::future::Future<Output = fn() -> String>, generic: G) 
                -> impl std::future::Future<Output = String>
                where G: std::future::Future<Output = String>{
                
                let into_g = generic.into();
                into_g.run();

                /* since gen param is bounded to Into trait we can call into() method on it */
                // let gen_into = gen.into();

                /* since gen param is bounded to AsRef trait we can call the as_ref() method to get the reference to Callback instance */
                let gen_as_ref = gen.as_ref();

                async {
                    "wildonion".to_string()
                }
            
            }

            /*
                since &mut "wildonion".to_string() is being dropped once we used it to pin it 
                we can't use it to get the bytes, we must create a type with longer lifetime
                to avoid this 
            */
            let string = &mut "wildonion".to_string();
            let pinned = std::pin::Pin::new(string);
            let p_bytes = pinned.as_bytes();

        }

        fn implant(&mut self, mut imp: impl Interface + Send + Sync + 'static){
            imp.check(); /* since check() is a mutable method the impl must be passed mutably */
        }
    }

    trait Interface{
        fn check(&mut self);
    }


    // **************************************************************************
    // **************************************************************************
    // **************************************************************************
    // --------------------------------------------------------------------------
    ///////////////////////// CLOSURE TYPES EXAMPLE /////////////////////////////
    // we cant only use impl Trait syntax in function return type
    // calling the boxed closure traits by unboxing or dereferencing the boxed value
    // --------------------------------------------------------------------------
    let workers = 10;
    type Job = Box<dyn Fn() -> () + Send + Sync>;
    for worker in 0..workers{
        let job: Job = Box::new(||{});
        let thread = std::thread::spawn(move ||{
            // job()
            (*job)() //// job ca be a socket connection
        });
    }
    struct RunnerTar<C=std::pin::Pin<Box<dyn Interface>>>{
        data: C ////
    }
    //// since we have a closure inside the Box which is of 
    //// type trait, thus we can call it in a different ways
    //// like the following
    //
    //// if we have closure inside the Box since Box is a 
    //// pointer we can deref it first like (*boxed)() 
    //// then call it or we can call it directly like boxed() 
    let mut d_boxed = Box::new(||{
        let name = String::from("wildonion");
        name
    });
    //// we can call the d_boxed since the closure trait
    //// is behind a pointer which is Box in our case thus 
    //// we can call the closure directly by calling the 
    //// d_boxed type, means we're calling the Boxed type
    //// which is a pointer to a closure
    d_boxed();
    //// since d_boxed is a Boxed type which is a pointer 
    //// to a heap data which is a closure tratit (traits are ?Sized) thus in order
    //// to call the closure directrly we can deref the Boxed type 
    //// then call the closure. 
    (*d_boxed)();
    //// as_mut() will convert the d_boxed() into 
    //// the type inside the Box which is &mut dyn FnMut(String) -> String
    //// then we can call the trait using ()
    d_boxed.as_mut()(); 

    // we can't have Pin<Box<impl Future<Output = i32>>> 
    // the impl Trait will be added by compiler 
    // - return traits from method using -> impl TraitLikeClosure, Box<dyn Trait> or &'valid dyn Trait which implements trait for the return type
    // - use traits like closures in struct field using where or Box and method param using impl Trait
    // - pin async block into the ram so we can await on it in future 
    // - since async blocks are future objects we must put them behind a pointer thus we must pin the boxed future object
    // - also the future object must be valid across threads thus we must bound the Future object to Send + Sync + 'static
    let dejavo = Box::pin(async move{
        32
    });

    dejavo.await;



    trait Message{}
    trait Handler<M: Message + Send + Sync + 'static>{
        type Result;
        fn handle(&mut self, msg: M) -> String;
    }
    struct Actor;
    impl<M: Message + Send + Sync + 'static> Handler<M> for Actor{
        type Result = String;
        fn handle(&mut self, msg: M) -> Self::Result{

            "message".to_string()
        }
    }


    struct Link<'link, D, F: Send + Sync> 
    where D: Send + Sync,
    F: FnOnce() -> String{
        pub data: F,
        pub c: &'link dyn FnOnce() -> String, // Or Box<dyn FnOnce() -> String> | Box has its own lifetime
        pub link: &'link D
    }
    impl<D: Send + Sync, F: Send + Sync> Link<'_, D, F> 
        where F: FnOnce() -> String{
        
        fn run() -> impl FnMut() -> u32{
            ||{
                {
                    let item_remaining = 32; 
                    item_remaining
                }
            }
        }

        /* we can impl traits for the method param and bound its type to that trait */
        fn start(cmd: impl FnOnce() -> ()){
            cmd();
        }

        fn another_start_here(cmd: impl Fn(String) -> ()){
            
        }

        /*  

            if we want to take a mutable pointer to a future object then the future object 
            must implements the Unpin trait, because pinning will lock the object into the ram
            and makes its location stable in there and doesn't allow to move or mutate that object
            also it doesn't allow to obtain Box<T> or &mut T thus we must unpin the object first
            the following is the general explanation of this:

            By default, all types in Rust are movable. Rust allows passing all types by-value, 
            and common smart-pointer types such as Box<T> and &mut T allow replacing and moving 
            the values they contain: you can move out of a Box<T>, or you can use mem::swap. 
            Pin<P> wraps a pointer type P, so Pin<Box<T>> functions much like a regular Box<T>: 
            when a Pin<Box<T>> gets dropped, so do its contents, and the memory gets deallocated. 
            Similarly, Pin<&mut T> is a lot like &mut T. However, Pin<P> does not let clients 
            actually obtain a Box<T> or &mut T to pinned data, which implies that you cannot use 
            operations such as mem::swap


            future objects are traits and can be implemented for other types to convert those
            types into a future objects, also they can return any type as their result of solving
            process by default rust moves heap data types when we go to new scopes unless we borrow 
            them thus for future objects which are of type traits and traits are heap data types 
            we must pin their Box into memory since we don't know when they will be solved and 
            where the scope will be hence, we must bound the pointer of the pinned object to the 
            Unpin trait if we want to move it or take a pointer to it 

            we can get the result of impl Future<Output=String> simply by awaiting on the future 
            object but we can't await on an immutable pointer to a future object or &impl Future<Output=String>
            because :
                Unpin can makes type to be moved safely after being pinned for example if we want to 
                use mem::replace on T which has been pinned into the ram, the T must be bounded to 
                Unpin, also mem::replace works for any !Unpin data and any &mut T not just when T: Unpin
                in essence we can't use mem::replace on a pinned object like the pointer of future objects
                cause we can't get &mut T from it which means we must unpin the future object first then
                put it behind a mutable reference finally we can use mem::replace over that, remember that 
                using mem::replace over types requires that both of them be behind a &mut pointer since 
                this method will replace the location of both types in ram which a mutable process, 
                based on above notes, awaiting on an immutable pointer to a future object requires some 
                kinda replacing and moving operations which forces the object to behind a mutable pointer
                to it's type and be bounded to Unpin trait if we need to access the pinned value outside 
                of the current scope that we're awaiting on the object 

        */
        /* 
            since the future is behind a mutable pointer which can mutate the future 
            itself inside the ram thus it must be first unpinned 
        */
        async fn create_component(async_block: &mut (impl futures::Future<Output=String> + std::marker::Unpin)){
            
            let a = async_block;
            a.await;

            trait HealCheck{
                fn healtcheck(&self){}
            }
            async fn do_health_check(hc: impl HealCheck + Send + Sync + 'static){
                hc.healtcheck(); /* we can call the HealthCheck trait method since the hc is bounded to this trait already */
            }
         
         
            let nature = async{
                'out:{
                    break 'out ();
                }

                'outer: loop{ // outter labeled block 
                    println!("this is the outer loop");
                    'inner: loop{ // inner labeled block 
                        println!("this is the inner loop");
                        // break; // only the inner loop
            
                        break 'outer;
                    }
            
                    println!("this print will never be reached"); //// this is an unreachable code
                }
            
            
                'outer: for x in 0..5 {
                    'inner: for y in 0..5 {
                        println!("{},{}", x, y);
                        if y == 3 {
                            break 'outer;
                        }
                    }
                }
            };

        }

        /* ------------------------------------------ 
            in the following example:
            we can't use impl Trait in function param 
            since we have to give an explicit type to the 
            describe which rust doesn't accept impl Trait 
            in fn() pointer param, compiler will set the type 
            of describe to fn col<impl Trait>() later
        ------------------------------------------ */

        
        pub const FUNC: fn() -> i32 = {
            const fn resize() -> i32{
                32
            }   

            resize
        }; 

        fn callmehore(){
            fn colided<Z>(cls: Z) -> () 
            where Z: FnOnce(String) -> ()
            {
                let name = "wildonion".to_string();

                let callback = cls(name);
                let res = match callback{
                    () => false, /* matches any value this will be matched since the return type of closure is () */
                    _ => true, /* _ means any value */
                    |_| () => true, /* |_|() matches any result of calling the callback */
                    |_| () | _ => false, /* matches any result of calling the callback or _ */
                    _ | _ => true /* matches any value or any value */
                };
            }
            
            let describe = colided;
            describe(|name: String|{
                ()
            });

            /* 
                bounding the param type to closure trait directly 
                without using where but by using impl Trait, 
                in this case we didn't store the bolided into 
                a new type thus we can call it directly with 
                no problem
            */
            fn bolided(cls: impl FnOnce(String) -> ()){
                let name = "wildonion".to_string();
                cls(name);
            }
            bolided(|name: String|{
                ()
            });

        } 


        //////-------------------------------------------------------------------------------
        ////// we can't return impl Trait as the return type of fn() pointer or as its param
        //////-------------------------------------------------------------------------------
        // type Function = fn() -> impl futures::Future<Output=String>;
        // async fn create_component_method<L>(async_block: fn() -> impl futures::Future<Output=String>) {
        //     async_block().await;
        // }
        //////--------------------------------------------------------------
        ////// we can't return impl Trait as the return type of traits 
        //////--------------------------------------------------------------
        // async fn create_component_method<L>(async_block: L) where L: Fn() -> impl futures::Future<Output=String>{
        //     async_block().await;
        // }
        //////--------------------------------------------------------------
        ////// we can't .await after calling async_block() since future objects 
        ////// in Box must be pinned to the ram to be valid across scopes for later solves
        //////--------------------------------------------------------------
        // async fn create_component_method<L>(async_block: L) where L: Fn() -> Box<dyn futures::Future<Output=String>>{
        //     let res = async_block();
        // }
        //////--------------------------------------------------------------
        ////// generic type L is a trait which its return type is an async block, the reason
        ////// we put the future object inside a pinned boxed is because we can't simply return
        ////// a trait as the return type of another trait thus we have to put it inside the Box
        ////// or behind a valid pointer like &'valid dyn Trait (because traits are dynamic sized types)
        ////// also in order to solve the boxed future after calling the async_block trait, the future 
        ////// object must be valid across scopes since we don't know the exact place of its solving 
        ////// which it might be inside different scopes other than where it has initialized and because
        ////// of this reason we must also pin the pointer of the future object into the ram to prevent 
        ////// its location from moving (or replacing by another type) until we await on it to get the result
        ////// since once we move into other scopes its lifetime will be checked by the borrow checker
        ////// and if it has already pinned the rust can't move it and drop its lifetime
        /// 
        ////// since future objects are traits and traits don't have fixed size (since they're heap data types) thus we must
        ////// put them inside the Box and pin that Box to the ram, by pinning them we can go to other scopes without losing 
        ////// their ownership (since the're pinned to ram and can't be moved) and await on the pinned boxed future whenever
        ////// and where ever we want 
        //////--------------------------------------------------------------
        async fn create_component_method<L>(async_block: L) where L: Fn() -> std::pin::Pin<Box<dyn futures::Future<Output=String>>>{
            let res = async_block().await;
        }
    }

    // ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
    struct Useram{
        pswd: String
    }
    impl Useram{
        pub fn get_token(&self) -> &str{
            "token"
        }
    }
    let user = Useram{pswd: "wildonion".to_string()};
    /* 
        since we're putting the return type of the Box which is
        another Box contains a future object inside the Pin thus 
        all the types inside the second Box must be live long anough
        and be valid across .await until the value gets unpinned from
        the ram. we can solve this by moving the that type into the 
        async block or the future object. 
    */
    let token: Box<dyn FnOnce() -> 
        Arc<std::pin::Pin<Box<dyn std::future::Future<Output=String> + Send + Sync + 'static>>> //// a shared pinned box object 
            + Send + Sync + 'static> = 
            Box::new(|| Arc::new(Box::pin(
                    /* 
                        by using move keyword we can move the user into this scope so it can 
                        have a valid lifetime across .await since the following async block will
                        be pinned into the ram which all the types inside the async block must be 
                        valid until the future object gets unpinned.  
                    */
                    async move{
                        user.get_token().to_string()
                    }
                ))
            );
    /* 
        here we can't deref the token object to call it 
        and it MUST be remained behind a pointer since 
        by derefing it we'll get a trait object which 
        has defined by ourselves in the code, not the 
        compiler, that it's size is not known at compile 
        time thus we can't allow it to be inside the Box
        or behind a pointer to use it
    */
    // let get_token = (*token)().await; 
    let get_token = token(); /* here token is callable since it's just only behind a heap data pointer which is Box */
    // ||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||


    //// closure traits can't be defined as a type
    //// since they are heap data which their size
    //// are unknown at compile time and must be 
    //// behind a pointer like &'valid dyn or inside the 
    //// Box with a valid lifetime 
    // type ClosureTrait = FnOnce(String) -> String; 
    struct Run<F, T = fn() -> String> //// T has a default type parameter
        where F: FnOnce(String) -> String{
        data: F,
        another_data: T,
        third_data: fn() -> ()
    }
    trait InterfaceExt{}
    impl<F, T> InterfaceExt for Run<F, T> where F: FnOnce(String) -> String{}
    
    // -> impl Trait only allowed in function not in trait return type
    // since we can't impl a trait for the return type of another trait!!
    fn runYours() -> impl FnOnce(String) -> String{ //// return closure using -> impl Trait 
        |name: String|{
            name
        }
    } 

    fn runOurs() -> Box<impl FnOnce(String) -> String>{
        Box::new(
            |name:String|{
                name
            }
        )
    }

    fn runYours_() -> &'static dyn FnOnce(String) -> String{ //// return closure using -> &dy Trait
        &|name: String|{
            name
        }
    }

    fn run_() -> impl InterfaceExt{
        fn catch(name: String) -> String{name}
        fn catch_me(){}
        let instance = Run{
            data: |you|{
                you
            },
            another_data: catch,
            third_data: catch_me
        };
        /* 
            returning the instance of the Run struct 
            since the return type is InterfaceExt means
            we must return a type that this trait is already 
            implemented for it, we can't return the trait 
            directly with this syntax inside the function
            signature, we have to put it inside the Box
            which has its own lifetime or put it behind 
            &dyn with a valid lifetime like 'static 
        */
        instance 
    }

    //// Box<dyn Trait> has its own lifetime since Box
    //// since Box has its own lifetime but &dyn Trait 
    //// needs a valid lifetime like 'static
    fn run__() -> Box<dyn FnOnce(String) -> String>{ //// return closure using -> Box<dyn Trait>
        Box::new(
            |name: String|{
                name
            }
        )
    }

    fn start<'lifetime, F>(cls: F) -> () where F: FnOnce(String) -> String + 'lifetime{ /* ... */ } 
    fn start_(cls: Box<dyn FnOnce(String) -> String>){ /* ... */ }
    fn start__(cls: fn() -> String){ /* ... */ }
    fn start___(cls: impl FnOnce(String) -> String){ /* ... */ }
    // **************************************************************************
    // **************************************************************************
    // **************************************************************************
    
}