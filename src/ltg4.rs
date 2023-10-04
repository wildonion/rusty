



use crate::*;



async fn test(){

    fn init_vm(){

        let datarefcell: Rc<RefCell<&'static [u8; 64]>> = Rc::new(RefCell::new(&[0u8; 64]));
        let lam = **datarefcell.borrow_mut(); //// double dereference to get the [0u8l 64] which has 64 bytes data 
    
        #[derive(Debug, Clone)]
        enum Chip{
            Intel{version: String},
            M1
        }
        let cmd = Chip::Intel{version:"wildonion".to_string()};
        let Chip::Intel{version: esm} = cmd else{
            panic!("no");
        };
    
        struct Runtime;
        trait RuntimeExt{}
        struct ByteCode<'b>{
            pub bytes: &'b [u8]
        };
        struct VirtualMachine<'Exectuor, 'b, Runtime: Send + Sync + 'static, const SIZE: usize>
            where Runtime: RuntimeExt,
            ByteCode<'b>: Send + Sync + 'static{
           pub rt: &'Exectuor Runtime,
           pub bytecodes: &'b [ByteCode<'b>; SIZE]
        }
        
        #[derive(Debug, Clone)]
        struct Executor;
        #[derive(Debug, Clone)]
        enum Cost{
            Runtime{executor: Executor},
            Vm,
        }
        let cost = Cost::Runtime { executor: Executor };
        match cost{
            Cost::Runtime { executor: executor_instance } => {
                let ext = executor_instance;
                todo!()
            },
            Cost::Vm =>{
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    /* ------------------------------------------------------- */
    /* ------------------------------------------------------- */
    let (ref name, age) = ("wildonion", 23);
    trait Interface{
        fn call(&self){}
    }
    struct Struct;
    impl Interface for Struct{
        fn call(&self){}
    }
    impl Interface for (){
        fn call(&self){}
    }
    let _ = match (name, age){
        (&"onion", 12) | (&"dewo", 45) => {
            let res = {
                
                async fn run<'v, 'a, V>(param: impl Interface) -> &'a &'v str 
                    where V: Send + Sync + 'static + Interface{
                    
                    trait Interface{}
                    struct U8<'u>(pub &'u [u8]);
                    let async_res = async{

                        /* 
                            unwrap() takes ownership so use as_ref() or clone() before calling it 
                            also if the type is behind a shared pointer we can't move it since pointer will
                            be a dangling one thus we have to borrow it or clone it and move the clone 
                            or borrow version of it between scopes
                        */
                        struct Execute<'lifetime, G>(pub &'lifetime [G]) where G: AsRef<[u8]>;
                        impl<'a, G: AsRef<[u8]>> Execute<'a, G>{
                            fn run(g: G) -> G{
                                let as_ref_g = g.as_ref(); /* this works since g is of type G which is bounded to AsRef trait */
                                g
                            }
                            /* -------------------- */
                            /* new as_ref() example */
                            /* -------------------- */
                            fn program<'u, P>(p: P, s: impl Interface) 
                                -> Result<(), ()> where P: AsRef<&'u Option<U8<'u>>>{

                                /* 
                                    can't move out of a shared reference since p.as_ref()
                                    is a shared reference which can't be moved because 
                                    unwrap() takes the ownership of the self thus we have
                                    to call another as_ref() on the first as_ref() to borrow
                                    the ownership of the first as_ref() and then unwrap 
                                    it to take the first element of the unit like struct which 
                                    is &[u8]
                                */
                                let p_ref = p.as_ref().as_ref().unwrap().0;
                                Ok(())
                            }
                        }
                    };

                    let ref name = "";
                    name
                    
                }
                /* 
                    when the return type is a trait means that 
                    the trait must be implemented for the return
                    type which in our case is ()
                */
                async fn execute(param: Struct) -> impl Interface{
                    param.call()
                }
                execute(Struct{})
            }.await;
            res
        },
        _ => {

            (
                || async {}
            )().await;

            todo!()
        }
    };
    /* ------------------------------------------------------- */
    /* ------------------------------------------------------- */

    pub struct Connection;
    #[async_trait::async_trait]
    pub trait RuntimeExecutor<'lifetime, G>{
        async fn invoke(&mut self, connection: &'lifetime mut Connection) -> Connection{
            Connection{}
        }
    }
    pub async fn cpi_transfer<'lifetime, E>(connection: &'lifetime mut Connection, mut runtime: E) 
        where E: RuntimeExecutor<'lifetime, Connection> + Send + Sync{

            /* future are traits that must be behind pointers like Box<dyn> or &dyn */
            type PinnedBoxPointerToFuture<'lifetime> = std::pin::Pin<Box<dyn std::future::Future<Output=Connection> + Send + 'lifetime>>;
            /* e must be mutable since run() method accepts a mutable pointer */
            let r: PinnedBoxPointerToFuture = runtime
                .invoke(connection);
            r.await;
        }

    /* multi types support method */
    fn move_me<T>(param: T) -> () where T: AsRef<[u8]>{
        // T is bounded to AsRef thus we can call the as_ref() method
        // also AsRef can be used for String, vector which convert them
        // into slices
        let a = param.as_ref(); 
    }
    let name = "wildonion".to_string();
    let vec = vec![1];
    move_me(name.clone());
    move_me(&name);
    move_me(&[12]);
    move_me(vec);

    type KKeys = String;
    struct Keccak<KKeys>{
        public_addr: KKeys,
    }
    let instance = Keccak{
        public_addr: String::from("")
    };

    pub async fn race_condition_avoidance(){

        /* ---------------------------------------------------------------------- */
        /* ---------------------- RACE CONDITION AVOIDANCE ---------------------- */
        /*  
                        https://github.com/wildonion/redis4
    
            race conditions means that two threads want to mutate the data 
            at the same time we have to use mutex so thell the other threads
            wait there is a threads that is trying to mutate this type and 
            will update you once the lock gets freed and in order to avoid blockcing 
            issues in the current thread we have to lock inside a separate thread 
            and mutate the type then send it through the jobq channel to the other 
            threads for reading
        */
        #[derive(Clone)]
        pub struct Data{
            /* we're using tokio mutex to avoid blocing issues inside the current thread since it locks asycnly */
            pub actual: std::sync::Arc<tokio::sync::Mutex<String>>
        }
        let mut data_instance = Data{
            actual: std::sync::Arc::new(tokio::sync::
                Mutex::new(
                    String::from("a mutexed data")
                )
            ),
        };
        
        println!("data instance actual value before getting mutated >>> [{}]", data_instance.actual.lock().await.to_owned());
        
        /* reading from the channel is a mutable process thus receiver must be mutable */
        let (data_sender, mut data_receiver) = 
            tokio::sync::mpsc::channel::<Data>(1024);
        /*
            since tokio spawn takes a closure which captures the env vars 
            we have to use the cloned form of those types and pass them into
            the closure scopes so we can use them in later scopes 
        */
        let sender = data_sender.clone();
        tokio::spawn(async move{
            
            let new_string = String::from("an updated mutexed");
            /* 
                we're cloning data_instance and data_instance_cloned.actual to create a 
                longer lifetime value to use the cloned form to mutate, since by sending 
                data_instance_cloned to the channel its lifetime will be dropped and its 
                ownership will be moved because we're borroing the actual field by locking 
                on it so we can't move the data_instance_cloned into the mpsc channel using 
                the sender, in other words we can't move out of the type if it's behind a 
                shared reference we have to either pass a reference or clone the type and 
                work on the cloned form like the followings which we're cloning the actual 
                field to lock on its mutex and send the data_instance_cloned into 
                the downside of the channel
            */
            let data_instance_cloned = data_instance.clone();
            let data_instance_cloned_actual = data_instance_cloned.actual.clone();
            let mut data_string = data_instance_cloned_actual.lock().await; /* lock the mutex to mutate it */
            
            /* 
                mutating the locked mutex is done by dereferencing the guard 
                we're mutating data string inside the actual field in data_instance_cloned
                this will mutate the actual field inside data_instance_cloned 
            */
            *data_string = new_string;
    
            if let Err(why) = sender.send(data_instance_cloned).await{
                println!("why can't send {:?}", why.to_string());
            }
    
        });
    
        /* receiving asyncly inside other threads to avoid blocking issues on heavy computations */
        tokio::spawn(async move{
            /* receving data asyncly while they're comming to the end of mpsc jobq channle */
            while let Some(data) = data_receiver.recv().await{
                
                let new_data_string = data.actual.lock().await.to_owned();
                println!("data instance actual value after getting mutated >>> [{}]", new_data_string);
        
            }
        });
    
    }


    
    /* ------------------------------------------------------------------ */
    /* ---------------------- ACTOR IMPLEMENTATION ----------------------
        a handler needs to be implemented for each message type 
        since a message may return a type which can be caught as
        the return type of the send() method once it gets called 
    */
    /* ------------------------------------------------------------------ */
    /* ------------------------------------------------------------------ */
    struct SomeServerActor<M: Send + Sync + 'static>{
        sender: tokio::sync::mpsc::Sender<M>,
        mailbox: tokio::sync::mpsc::Receiver<M>,
    }

    impl<M: Send + Sync + 'static> SomeServerActor<M>{

        pub async fn send(&mut self, m: M) -> () {
            self.sender.send(m).await;
            
        }

        async fn receive(&mut self){
            
            let m = self.mailbox.recv().await;

        }

        async fn execute(&mut self, f: fn() -> ()){

            tokio::spawn(async move{
                f()
            });
        }

    }
    /* ------------------------------------------------------------------ */
    /* ------------------------------------------------------------------ */

    trait InterfaceMe{}
    impl InterfaceMe for () {}

    pub struct Req;
    pub struct Res;
    let req = Req;
    let res = Res;
    pub struct Test<T, R: std::future::Future<Output=Res> + Send + Sync +'static> 
        where T: FnMut(Req, Res) -> R{
        pub f: T //// f is a FnMut closure which accepts Req and Res instances as its params 
    }
    let cb = |req, res| { async {res} /* the return type of the closure must be future object */};
    let mut instance = Test{f: cb};
    let response = (instance.f)(req, res).await;

    //// traits as a field (param) or return type must be behind a 
    //// pointer using Box or &dyn but as the type of a passed in 
    //// param the generic type of the param must be bounded to that trait.
    // 
    //// stack pinning can be a captured state of async block or 
    //// function which can be done using pin!{} which constructs 
    //// Pin<&mut T> and is cheaper than heap pinning or Box::pin()
    // fn run() -> impl Generator<Yield = i32, Return = ()>{} //// default type parameter
    // /// Runs a future to completion.
    // fn block_on<F: Future>(future: F) -> F::Output {
    //     let waker_that_unparks_thread = todo!();  
    //     let mut cx = Context::from_waker(&waker_that_unparks_thread);
    //     // Pin the future into the ram so it can be polled later whenever it gets ready
    //     let mut pinned_future = pin!(future);
    //     loop {
    //         match pinned_future.as_mut().poll(&mut cx) {
    //             Poll::Pending => thread::park(), //// block_on method will block the current thread by parking it
    //             Poll::Ready(result) => return result,
    //         }
    //     }
    // }


    pub async fn return_vec_of_box_traits<G>(c: 
            Box<dyn InterfaceMe + Send + Sync + 'static>, 
            //// if we want to use generic in rust we have to specify the generic name in function signature  
            //// since G is a closure that is bounded to FnMut we have to define it a mutable type 
            mut b: G) 
        -> Vec<Box<dyn InterfaceMe + Send + Sync + 'static>>
        where G: FnMut(u8) -> (){
 
            let mut n_c = 2; //// since the closure is bounded to FnMut thus we have to define teh cores as mutable since it'll get a mutable borrow
            b(n_c); //// we're calling the closure here and pass the mutable n_c param
            let mut vector_of_boxed = vec![];
            vector_of_boxed.push(c);
            vector_of_boxed

    } 
    //// first param of the `return_vec_of_box_traits` function
    //// is a type that accepts a Box of `InterfaceMe` trait 
    //// whence the `InterfaceMe` trait is implemented for () or
    //// the empty type, we can create a Box of () or Box::new(())
    //// and pass it as the first param, for the second param 
    //// we've passed a closure with empty return body
    //
    //// we'll pass the u8 value when we're calling the 
    //// closure but we can use it here and store it in 
    //// cores variable
    return_vec_of_box_traits(Box::new(()), |cores|{ 
        println!("number of cores is : {}", cores);
    }).await;


    //----------------------------
    let clsMe = |name: String| { //// we can also put the closure body inside a curly braces

        let mut val = "wildonion".to_string(); /* creating longer lifetime by binding the val into let */
        let mut boxed = Box::pin(&mut val);
        let ref_ = &mut boxed;
       
        Box::pin(async {
            name
        })
    };
    let clsMe = |name: String| Box::pin(async{ //// since the return type is a Pin<Box<T>> there is no need to put the Box::pin inside curly braces since it's a single line code logic
        name
    });
    //----------------------------

    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    struct Round{
        val_idx: u8,
        values: Vec<u8>
    }
    let announced_values: Vec<Round> = vec![];

    ///////------ closures can capture their env vars into their scopes but function can't 
    ///////------ in function we must pass the actual type as the param to mutate it or read it
    
    /* 
        closures can capture env vars so we can access them inside the closure method, with 
        function we can't do that, since functions have their own scopes, we could either pass 
        the type by value if we don't need its ownership (specially for heap data) or reference 
        if we don't want to lose its ownership inside the caller scope of the method also to mutate 
        the content of the type inside the function without mutating the actual type we 
        must pass a mutable reference to it like for mutating announced_values we must pass 
        the mutable reference to announced_values type to the is_duplicate_fn function, 
        since by mutating the mutable pointer of the main type the actual type will be mutated too, 
    */

    fn is_duplicate_fn(val: u16, val_idx: u16, announced_values: &mut Vec<Round>) -> bool{
        for av_idx in 0..announced_values.len(){
            if (announced_values[av_idx].values[val_idx as usize]) as u16 == val{
                return true;
            } else{
                return false;
            }
        }
        return false;
    }

    /*
        the following closure will borrow and capture the result_announced_values var
        as immutable, thus we can't push into the result_announced_values vector later
        on if we're going to use this method, since rust doesn't allow to borrow the 
        type as mutable if it's borrowed as immutable already in a scope, instead we 
        can use FnMut closure to capture vars mutablyو also announced_values must be 
        initialized in order the closure to be able to capture it into its env
    */
    let is_duplicate = |val: u16, val_idx: u16|{
        for av_idx in 0..announced_values.len(){
            if (announced_values[av_idx].values[val_idx as usize]) as u16 == val{
                return true;
            } else{
                return false;
            }
        }
        return false;
    };
    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= */
    
    clsMe("wildonion".to_string());
	
	
    pub struct Complex{
        pub callback: Box<dyn FnOnce(Option<String>) -> u8>,
        pub labeled_block: bool,
        pub long_block: Option<u8>,
        pub callback_result: u8,
    }
    
    let comp = Complex{
        callback: Box::new(
            |_: Option<String>| 13
        ),
        labeled_block: 'block:{
            if 22 % 2 == 0{
                break 'block true; // it'll break the 'labeled block with a true return
            } else{
                break 'block false; // it'll break the 'labeled block with a false return
            }
        },
        long_block: {
            let mut x = 0;
            while 2 % x > 2{
                x+=1;
            }
            let somed = Some(x);
            match somed{ // we must cover all the match arms if we have if in one of the arm 
                Some(n) if n == 2 => Some(n as u8), // in those case that n must be 2 
                Some(n) => Some(n), // if this arm was the first arm then above arm will be unreachable since this arm has no condition in it thus definitely will be the matched one
                None => None
            }
        },
        callback_result: ( // building and calling the closure at the same time inside the struct field
            |_| 254
        )(Some("wildonion".to_string())),
    };

    //// if let unpacking
    // if let Complex{ 
    //     callback, 
    //     labeled_block,
    //     long_block,
    //     callback_result 
    // } = comp{
    // 	println!("unpacking is ok!");
    // }
    //// let else example
    let Complex{ 
        callback, 
        labeled_block,
        long_block,
        callback_result 
    } = comp else{ // the else part is not needed since the unpacking process will be matched always
        panic!("can't unpack");
    }; // struct unpacking


    // let Complex{..} = com else{ // .. means all the fields
    //     panic!("can't unpack");
    // };

    pub async fn do_it<F>(callback: F) // callback is of type F
        -> u8 where 
                F: FnOnce(Option<String>) -> u8 + Send + Sync + 'static
        { // where F is a closure which is bounded to Send Sync traits and have a valid static lifetime
        callback(Some("wildonion".to_string())) // by calling the passed in closure we can have the u8 as the result of calling which must be returned from this function
    }
    do_it(|name|{
        let Some(some_u8_number) = Some(24) else{
            panic!("can't get out of Some");
        };
        some_u8_number // the some_u8_number scope is still valid in here and we can return
    }).await;
    
    ( // building and calling the closure at the same time; the return type of this closure is a future which must be awaited later on
        |age| async move{ 
            age
        }
    )(32).await;

    

    let names =  //// building and calling the async closure at the same time
        (|x| async move{ //// the return body is a future object which must be solved later using .await and move will move everything from the last scope into its scope  
            let names = (0..x)
                .into_iter()
                .map(|index|{
                    let name: String = "wildonion".to_string();
                    name
                })
                .collect::<Vec<String>>();
            names
        })(23).await;



    let statement = |x: u32| Some(2);
    let Some(3) = statement(3) else{ // in else part there must be panic message
        panic!("the else part");
    };

    // a function is created using () also
    // calling a function is done by using ()
    // thus by using ()() we're building and calling
    // the function at the same time
    let res = { // res doesn't have any type
        ( // building and calling at the same time inside the res scope
            |x| async move{
                x
            }
        )(34).await; 
    };


    // nodejs like function call
    fn sayHelloAgain<C>(call: u8, callback: C) // C is the callback type which is a FnOnce trait
        where C: FnOnce(Option<u8>, HashMap<String, String>){
        callback(None, HashMap::new());
    }


    sayHelloAgain(23, |n_c, m|{
        let inputs: Vec<Vec<f64>> = vec![vec![5.6, 5.3]];
        for index in 0..inputs.len(){
            let row = &inputs[index]; //// inputs in the first iteration will be moved from the memory thus we have to borrow it or clone it
        }
        let map = m;
        let none_call = n_c;

        pub struct Nft{
            pub id: u16,
            pub title: String,
            pub royalties: Vec<Royalty>,
        }
        pub struct Royalty{
            pub receiver: String,
            pub amount: u128,
        }
        let nfts: Vec<Nft> = Vec::new();
        nfts.into_iter().map(|nft| {
            for r in nft.royalties{
                let who = r.receiver;
                let much = r.amount;
            }
        });
    });

    let callback = |_| Some(1); // |_| means that the param name can be anything  
    let (
        |callback| callback // the return type is the callback which is a closure
    ) = match callback(..){ // callback(..) means that it'll take anything as the range - when we're do a matching on the callback(..) means that by calling the callback(..) we should get a closure in its return type which this is not the case hence this code is unreachable 
        |_| Some(2) => |_| Some(3), // |_| Some(2) is the other syntax for calling the x closure - the or pattern: it can also be _ | Some(2) since _ means the value can be anything thus one of side can only be executed (either _ or Some(2))  
        |_| _ => unreachable!(), // |_| _ is the other syntax for calling the x closure - the or pattern: it can also be _ | _ since _ means the value can be anything thus one of side can only be executed (either _ or _)
    };
    // the return type of calling callback(..) is not a closure hence we can't do a matching on closures and because of that the code will be unreachabled
    assert!(matches!(callback(..), |_| Some(4))); // it'll be unreachable since the first arm of the match is not match with this
    
}