



use crate::*;


// lifetimes can be passed to method and struct signature to be bounded to generic types for borrowing them or slices

fn test(){

    // no gloabl mutable data we can have mutex data which is thread safe
    pub const DELIM: &[u8; 32] = &[0x01; 32];
    pub static DELIM0: Lazy<std::sync::Arc<tokio::sync::RwLock<HashMap<String, String>>>> 
        = Lazy::new(|| { std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())) });

    // ------------------------------------------------------
    // ----------------------- LTG EX -----------------------
    // ------------------------------------------------------
    /* 
        returning a pointer to a data which is owned by the method body
        regardless of it's type, either stack slice or heap data types 
        is not possible since the data has allocated space on the ram 
        inside the method body and due to future dangling pointer issue 
        rust won't allow us to do this in the first place, we can solve 
        this solution by complying the following notes:
    */
    fn from_string<'valid>(data: [u8; 32]) -> &'valid [u8]{

        // let bytes = "wildonion".as_bytes();
        // bytes

        // data has been moved to this method which is owned by the current function
        // means that it's allocated something in the ram inside the method body 
        // and thus can't return a pointer to that
        // data.as_slice()

        // or 

        "wildonion".as_bytes()

        
    }
    // NOTE1 - can't return pointer from method to stack and specially heap data since they're owned by method and they've allocated space on the ram inside the method body
    // NOTE2 - we can return a pointer to empty struct or an struct that contains stack data types
    // NOTE3 - we can return a pointer to closure traits if they're allocating nothgin in stack in method body (in-place returning pointer) 
    // NOTE4 - we can return a pointer with valid lifetime to heap data slice forms
    // NOTE5 - at runtime all heap data will be coerced to slice form
    
    // but if the struct contains a heap data fields like String and Vec 
    // we can't do that since these heap data will take allocation space 
    // inside the method body and we can't return ref to heap data at all 
    // cause they'll be owned by the function.
    // struct Exe1{pub name: String, pub vec: Vec<String>}
    // fn execute1<'elifetime>() -> &'elifetime Exe1{
    //     &Exe1 {name: "wildonion".to_string(), vec: vec!["now".to_string()]}
    // }
    // this is not ok cause all the strings are allocated
    // inside the function body which are owned by the 
    // function and returning a ref to them is not allowed
    // by rust because of dangling pointer issues, thus 
    // returning pointer to heap data is not ok at all
    // fn execute4<'elifetime>() -> &'elifetime String{
    //     &"&move ||{}".to_string()
    //     &String::from("")
    // }
    // ----------------
    // return a valid ref to struct itself from method is ok cause it 
    // allocates nothing on the stack thus we can ret &'elifetime Exe
    // or if the struct contains stack data we can also return ref to
    // that yet.
    struct Exe{pub id: i32}
    fn execute<'elifetime>() -> &'elifetime Exe{
        &Exe {id: 8}
    }
    // ----------------
    // of course we're ok to return the slice of String or Vec or their coerced types
    // note that everything is in their coerced type of slice type
    struct Exe2<'elifetime>{pub name: &'elifetime str, pub arr: &'elifetime [&'elifetime str]}
    fn execute2<'elifetime>() -> &'elifetime Exe2<'elifetime>{
        &Exe2::<'elifetime>{name: "wildonion", arr: &["now"]}
    }
    // ----------------
    // this is ok since we're allocating nothing on the stack
    // and we're returning the closure in-place although the 
    // closures are traits and they're heap data
    fn execute3<'elifetime>() -> Box<&'elifetime dyn FnMut() -> ()>{
        Box::new(&move ||{})
        // if try to allocate it inside a var then we can't 
        // return a ref to it and the following faces us 
        // a compile time error since the type will be owned
        // by the function and once the method gets executed
        // the type will dropped from the ram and returning a 
        // pointer to that results to have a dangling pointer
        // which rust doesn't allow us to do this in the first place
        // let callback = move || {};
        // &callback
    }
    // ----------------
    // returning a slice of String or Vec is ok since it's a stack data
    // type, note that we have to use a valid lifetime and array 
    // can't contain heap data types
    fn execute4<'elifetime>() -> (&'elifetime str, &'elifetime [&'elifetime str]){
    
        // ("", &[""])

        // or 

        let name = "";
        let arr = &["name"];
        // we can't have the following cause it's temp 
        // value which is owned by the method
        // let arr = &[name];
        (name, arr)

    }
    // ----------------
    struct Type{}
    impl Interface for Type{}
    /* 

        we can't return a pointer to heap data while the data is owned by the method 
        means that if the has been initialized in method body or is a param that has 
        been passed to the method it can't be returned as a pointer, because an space
        inside the ram has been allocated for that type which has a valid lifetime as
        long as the method isn't executed yet and  once the method gets executed the 
        lifetime of the data will be dropped out of the ram and the pointer will be a 
        dangling pointer which rust doesn't allow us to do this in the first place, we 
        could return a pointer to the type of course, only if we're returning the
        initialization of the type in that moment which will allocate nothing on the 
        stack, also we can return the slice form of heap data specially &[] and &str
        for Vec and String respectively but we have to make sure that we're using 
        a valid lifetime for them and they don't contain heap data specially for
        &[] or a data that is owned by the method body, generally we can return pointer
        to heap data if and only if the data has a valid lifetime and:
            - is an slice form of the heap data itself like &'validlifetime [] or &str which: 
                - doesn't contain a heap data as well they must contain stack data like &str
                - doesn't contain a data which is owned by the method (it must be in-place allocation like &[&Type{}])
            - is a field of an instance of a struct (since &self is valid as long as the instance is valid)
                - which can be returned as a pointer using self lifetime
                - which can't be a data which is owned by the struct method
            - is a &str allocated in a var
                - returning &str from method is ok since they're by default are behind pointer
                - putting &str in a var and return the var allocates nothing in the method body 
                - it's ok to return them in-place or a var contains &str

        an slice of String and Vec are &str and &[] which are pointers to a part of String or Vec 
        in either binary, stack or heap data (will be specified at compile time) or an slice is 
        a pointer to the underlying of the data which is stored in either binary, heap or stack
        
        ret ref to heap data from method (structure field and their slice form with vaid lifetime)
        use trait like Box<dyn Trait> or &'t dyn Trait or Box<&'t Trait> in:
            - bounding generic to closure trait like => (param: T) where T: FnMut() -> ()
            - impl Trait for type like => impl Trait for Type{}
            - method param like => param: impl Trait
            - structure fields => param: Box<&'t dyn FnMut() -> ()>

        conclusions:    
            since evrything in rust has lifetime (due to lack of gc) and will be dropped out of the ram 
            once the type gets moved into other scopes thus there is no need to return pointer to types 
            from method specially heap data types cause returning a pointer to them is not possible since 
            they're owned by the method body and once the method gets executed their lifetime will be dropped
            from the ram this this rules is applied to return pointer to structures which contain heap data
            fields, solution to this can be returning their slice form like &'valid mut str and 
            &'valid mut [NoneHeapDataType] or although it's possible to return a pointer to String or Vec
            only if we're returning them from a structure method and they're the structure fileds cause 
            lifetime of &self is valid as long as the instance is valid thus returning pointer to String 
            or Vec which are fields of structure which are not owned by the method body is ok cause their 
            lifetime is valid until the instance gets dropped from the ram.
    
    */
    fn e1<'tlifetime>(param: &'tlifetime Type) -> &'tlifetime dyn Interface
        where Type: Interface{
        &Type{} // in-place allocation which allocates nothing on the stack 
        // &param // Error: can't return data owned by the method since by passing param to method it's lifetime is now owned by the method body
    }
    fn e2<'tlifetime>(param: Type) -> impl Interface
        where Type: Interface{        
        param
    }
    /* 
        why passing heap data into method as trait
        because once the heap data move inside the method its lifetime will be 
        owned by the method and will be dropped once the method gets executed 
        also it's better to use trait for types since there might be other 
        types from other crates that is going to be used as rng param and 
        all they have to do is implement Secure trait so we can call trait 
        methods on them
    */
    fn e3<'tlifetime>(param: &'tlifetime dyn Interface) -> &'tlifetime [&'tlifetime dyn Interface]
        where Type: Interface{      
        /* this is ok to be returned since the array contains no heap data */
        let params = &[];
        params

        /* this is not ok cause the array contains trait which is a heap data type */
        // let params = &[param];
        // params
        // or this:
        // &[param]
    }
    // fn e4<'tlifetime>(param: &'tlifetime dyn Interface) -> &'tlifetime [&'tlifetime String]
    //     where Type: Interface{      

    //         // this is not ok since we're returning a pointer to heap data
    //         &[&String::from("")]

    // }
    fn e5<'tlifetime>(param: impl Interface) -> &'tlifetime [&'tlifetime str]
        where Type: Interface{      

            // this is not ok since we're returning a pointer to heap data
            &[""]

            // this is not ok cause we're allocating an space in method body
            // by putting the &str inside the array 
            // which the data will be owned by the method thus can't return
            // a pointer to that
            // let na = "";
            // &[na]

    }
    // fn e6<'tlifetime>(param: impl Interface) -> &'tlifetime String
    //     where Type: Interface{      
    //         // this is not ok since creating string using String::from()
    //         // or "".to_string() will alway allocate space on the stack 
    //         // thus in both ways can't be acted as an in-place allocation
    //         // thus can't be returned as a pointer
    //         &String::from("")

    // }
    fn e7<'tlifetime>(param: impl Interface) -> &'tlifetime str
        where Type: Interface{      

            // this is ok since str by default are behind pointer 
            // and actually we're allocating nothing in here 
            // thus returning them with a var is ok
            let name = "";
            name 

    }
    fn e8<'tlifetime>(param: impl Interface) -> &'tlifetime str
        where Type: Interface{      

            // this is ok since str by default are behind pointer 
            // and actually we're allocating nothing in here 
            // thus returning them with a var is ok
            ""
    }
    // impl Trait is only allowed in method param and return type
    struct UserData<'v, D: Clone + ?Sized + Fn() -> ()>
    {
        data: D,
        role: &'v str,
        boxed_pin: std::pin::Pin<Box<&'v dyn futures_util::Future<Output=String>>>,
    }
    fn e9<'tlifetime, C: Fn() -> () + Clone>(param: &'tlifetime dyn Interface, cls: C) 
        -> &'tlifetime UserData<'tlifetime, C>
            where Type: Interface{      

            // returning pointer to heap data is allowed only if the data is 
            // a field of an instance cause self is valid as long as the instance
            // is valid, once we move the self into for example a tokio::spawn() which rust
            // won't allow in the first place, we'll lose the ownership of the self and thus 
            // all the fields

            /* 
                both of the ways are wrong since we can't 
                return pointer to a data which is owned by 
                the function, we have heap data in structure
                allocated space on the ram 
            */
            // let d = UserData::<'tlifetime, C>{
            //     data: cls,
            //     role: "user",
            //     boxed_pin: {
            //         Box::pin(&async move{
            //             String::from("")
            //         })
            //     }
            // };
            // &d

            // &UserData::<'tlifetime, C>{
            //     data: cls,
            //     role: "user",
            //     boxed_pin: {
            //         Box::pin(&async move{
            //             String::from("")
            //         })
            //     }
            // }

        todo!()

    }
    
    // fn e8<'tlifetime>(param: impl Interface) -> &'tlifetime u8
    //     where Type: Interface{      

    //         let num = 1;

    //         // this is not ok since num is owned by the method
    //         &num 

    // }
    // ----------------
    struct Ctor{
        pub name: String,
        pub arr: Vec<String>
    }
    impl Ctor{
        pub fn set(&mut self, new_arr: Vec<String>, new_name: String) -> (&Vec<String>, &String){
            
            self.arr = new_arr;
            self.name = new_name;
            
            // it's ok to return this cause we're returning a pointer 
            // to the struct fields itself and since the &self is valid 
            // as long as the object is valid and doesn't gets dropped
            // thus returning a pointer to them is ok
            (&self.arr, &self.name)
            
            // it's not ok to return the following because it's owned 
            // by the method and we can't return pointer to heap data 
            // since once the method gets executed their lifetime will
            // be dropped and no longer will be accessible  
            // (&new_arr, &new_name)
        }
    }

    /*
        every type in rust has lifetime and will be dropped once you move them in other 
        scopes without taking a reference to them or at the end of the scope like metho 
        body and tha't the reason we can't ret a pointer to heap data from the method body
        although slice types of Vec and String heap data are ok to be returned as pointer 
        also in place returning and matching allocates nothing on the stack
    */
    trait Respond{}
    struct Response{}
    struct Request<'elifetime, T: Clone + ?Sized, V = fn() -> ()>{
        pub data: T,
        pub func: V,
        pub boxed_traits_closures: Box<&'elifetime dyn FnMut() -> ()>
    }
    impl Respond for Response{}
    fn execute5<'elifetime, T: Clone, V, C, R: Send + Sync + 'static + Respond, 
        G: Send + Sync + 'static + AsRef<[C]>>
        (res: C, param: impl Respond, anything: G) 
        // the return type must implement the Respond trait
        -> (impl Respond, Box<&'elifetime dyn FnOnce(C) -> ()>,
            std::pin::Pin<Box<dyn std::future::Future<Output=G>>>)
        where C: FnMut(Request<T, V>, Response) -> R,
        R: Clone + Send + Send + 'static{
        
        let res_ = Response{};
        
        let ag = anything.as_ref();
        
        let cls = &move |res: C|{
            let new_res_obj = res;
        };
        
        // calling the callback and pass the res to it as its param cause res is of type C
        // which is a generic bounded to C FnMut trait
        cls(res); 

        // it's ok to return the param itself since it's bounded to Respond trait 
        // and we now that it's imeplemented the Respond trait
        (
            param, // or res_ // it's also ok to return the res object since Response struct implements the Respond trait
            Box::new(
                
                // it's not ok to put cls in here since it's 
                // owned by the function and can't return a ref
                // to that we have to use an in-place returning 
                // pointer which allocates nothing in the stack 
                // inside the method body
                // &cls --------------- ERROR
                &move |res|{
                    ()
                }
            ),
            // this is not ok since async move{anything} is allocated 
            // inside the method body and thus returning a pointer to 
            // that is not ok since it's a heap data
            // Box::pin(&async move{anything})

            // G must be bounded to 'static since we're pinning a future object
            // into ram to gets solved later thus any type that is being used 
            // inside of it must live long enough
            Box::pin(async move{anything})
        )
        
    }
    // --------------------------------------------------------------------
    // --------------------------------------------------------------------
    // --------------------------------------------------------------------

    pub const WO: &str = "widonion";
	impl Interface for Pack{} //// is required for return_box_trait(), return_impl_trait() and return_none_trait() functions in order to work
	pub trait Interface{}
    pub struct Commander{}
	pub struct Pack; //// we've allocated some space inside the stack for this struct when defining it which has long enough lifetime to initiate an instance from it using struct declaration and return a reference to that instance inside any function 

    /////////////////////////////////////////////////////////////////
    //// since rust doesn't have gc thus by moving a type into a new scope 
    //// its lifetime will be dropped unless it implements the Copy trait 
    //// otherwise if it's a heap data we must either clone it or borrow 
    //// it thus we must alwasy pass by reference to the data and note that 
    //// the heap data can be in their borrowed form like &[u8] and &str 
    //// also we can return a pointer to the type from a method by using 
    //// a specific or the self lifetime, note that we can also put the 
    //// sized array behind a pointer like Option<&[u8; 64]>
    /////////////////////////////////////////////////////////////////
    // NOTE - every pointer in rust must have valid lifetime thus if 
    //        we want to use & in return type of a method or struct 
    //        field we should have a valid lifetime for that.
    // https://stackoverflow.com/a/57894943/12132470
    // https://stackoverflow.com/questions/37789925/how-to-return-a-newly-created-struct-as-a-reference
    //// since rust doesn't have gc thus using value in other scopes we must notice that:
    ////     - value will be moved by default if it's a heap data and their previous lifetime will be dropped
    ////     - value will be copied by default if it's a stack data and we have them in other scopes
    ////     - note that we can't borrow the value after it has moved or it can't be any pointer if the value is going to be moved 
    ////     - note that we can't move the value if it 
    ////            - is behind a shared pointer or borrowed since the pointer of that might convert into a dangling pointer once the value gets dropped
    ////            - doesn't implement the Copy trait
    ////     - note that we borrow the value because 
    ////            - its size can't be known at compile time
    ////            - don't want to lose its ownership later
    //// which in order to not to lose the ownership of heap data we can either pass their 
    //// clone or their borrowed form or a pointer of them, note that if we clone them the main 
    //// value won't be updated since clone will create a new data inside the heap also heap 
    //// data sized can be in their borrowed for or behind a pointer like &str for String and 
    //// &[u8] or &[0u8; SIZE] for Vec if we care about the cost of the app.  
    //
    //// based on borrowing and ownership rules in rust we can't move a type into new scope when there
    //// is a borrow or a pointer of that type exists, rust moves heap data types by default since it 
    //// has no gc rules means if the type doesn't implement Copy trait by moving it its lifetime will 
    //// be dropped from the memory and if the type is behind a pointer rust doesn't allow the type to 
    //// be moved, the reason is, by moving the type into new scopes its lifetime will be dropped 
    //// accordingly its pointer will be a dangling one in the past scope, to solve this we must either 
    //// pass its clone or its borrow to other scopes. in this case self is behind a mutable reference 
    //// thus by moving every field of self which doesn't implement Copy trait we'll lose the ownership 
    //// of that field and since it's behin a pointer rust won't let us do that in the first place which 
    //// forces us to pass either its borrow or clone to other scopes. 
	impl Pack{ ////// RETURN BY POINTER EXAMPLE ////// 


	    fn new() -> Self{


            pub struct TupleStudent(pub String, pub u8);

            let stu_info = TupleStudent("wildonion".to_string(), 26);
            let TupleStudent(name, age) = stu_info;

            let name = Some("wildonion".to_string());
            struct User{
                username: String,
                age: u8,
            }

            let user = User{
                username: match name.clone(){ //// clone it here to be able use it in another_user instance
                    Some(name) => name, 
                    None => "".to_string(),
                },
                age: 26,
            };

            let another_user = User{
                username: match name{
                    Some(name) => name,
                    None => "".to_string(),
                },
                ..user //// filling the remaining fields with other User instance
            };

            #[derive(Default)]
            struct FuckMe{
                a: u8,
                b: String
            }

            let instanceFuckMe = FuckMe{
                a: 23,
                ..Default::default() //// fillint the remaining field with default values
            };        

            let FuckMe{a: first_input, ..} = instanceFuckMe;

            // let User{username, age} = user; //// unpacking struct
            let User{username: name, age: sen} = user; //// unpacking struct with arbitrary field names
            // let User{..} = user; //// unpacking struct with `..` since we don't care about all fields

            let hello = "Здравствуйте";
            let s = &hello[0..2];
            // every index is the place of an element inside the ram which has 1 byte size which is taken by that element
            // in our case the first element takes 2 bytes thus the index 0 won't return 3 
            // cause place 0 and 1 inside the ram each takes 1 byte and the size of the
            // first element is two bytes thus &hello[0..2] which is index 0 and 1 both returns 3 
            // and we can't have string indices in rust due to this reason!

            // we don't have string indices instead we have to access it using a range like [0..2] which gives us the first byte of the string
            // because string[1] means that returning the first char of the string that is 1 byte but if we have a utf16 string the first char 
            // is 2 bytes thus string[1] can't return the first char since it thinks that the every char of string is 1 byte hence rust doesn't
            // allow us to this in the first place because String will be coerced into slices or &str in compile time which we don't know where 
            // it will be placed which is either in heap, binary or stack thus the size it's unknown and because of this rust compiler can't know
            // the exact size of string and it's type in first place  


            ///////////////////////////////////////////// ENUM MATCH TEST
            #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
            enum Chie{
                Avali(u8),
                Dovomi(String),
                Sevomi,
                Chaharomi{ //// enum variant can also be a struct
                    name: String,
                    age: u32,
                },
            }

            // ------------------------
            //   matching over struct
            // ------------------------
            #[derive(Clone, Debug, Default)]
            struct OrmModel{
                name: String,
                other_mail: String,
                code: i32,
            }
            let orm_model = OrmModel{
                name: "wildonion".to_string(),
                other_mail: "ea_pain@yahoo.com".to_string(),
                code: 2435
            };
            let _ = match orm_model{
                OrmModel{name, other_mail: themail, code} if code > 2000 => {
                    // returning terminates the method execution and respond the 
                    // caller with the value where it gets called
                    0 
                },
                _ => {
                    1
                }
            };


            let ine = Chie::Avali(12); //// the Dovomi variant is never constructed cause we've used the first variant  

            match ine{
                Chie::Avali(value) if value == 23 => { //// matching on the Avali arm if the value was only 23
                    println!("u8 eeee");

                },
                Chie::Dovomi(value) if value == "wildonion".to_string() => { //// matching on the Dovomi arm if the value was only "wildonion" string
                    println!("stringeeee");
                },
                Chie::Chaharomi{name, ..} => { //// we only care about name and the rest of field will be filled by `..`

                },
                Chie::Chaharomi{name, age} => { //// using its own fields' names for unpacking on struct arm

                },
                Chie::Chaharomi{name: esm, age: sen} => { //// we can also give another names to the current struct fields using `:` for unpacking on struct arm

                },
                Chie::Chaharomi{name: esm, ..} => { //// we can also give another names to the current struct fields using `:` for unpacking on struct arm also we don't care about age field thus we can fill it up using `..`

                },
                _ => { //// for Sevomi fields
                    println!("none of them");
                }
            }

            // --------------- CODEC OPS ON ENUM ---------------
            let encoded = serde_json::to_vec(&Chie::Sevomi); ////// it'll print a vector of utf8 encoded JSON
            let decoded = serde_json::from_slice::<Chie>(&encoded.as_ref().unwrap()); //// as_ref() returns a reference to the original type

            let encoded_borsh = Chie::Sevomi.try_to_vec().unwrap(); ////// it'll print 2 cause this the third offset in memory
            let decoded_borsh = Chie::try_from_slice(&encoded_borsh).unwrap();

            /////////////////////////////////////////////
            Pack{}
	    }

        /*  -------------------------- pointers notes ---------------------------------------------
            can't return pointer to a type from methods if the type is owned by the function body, 
            we can return static lifetime or a pointer to the self.field also can't move data into 
            new scopes if its behind a pointer also we can return a pointer to none allocated stack 
            space data like -> &Struct{} since it didn't allocate nothing on the stack which also 
            is not owned by the method obviously
        -----------------------------------------------------------------------------------------*/
        // we can return a pointer to struct 
        // by returning &Struct{} from the method
        // since by doing this we're allocating 
        // nothing on the stack and the allocation
        // will be done once the caller gets the 
        // returned data from the function.
	    fn ref_struct(num_thread: &u8) -> &Pack{ //// returning ref from function to a pre allocated data type (not inside the function) Pack struct in our case, is ok
            let instance = Pack::new(); //// since new() method of the Pack struct will return a new instance of the struct which is allocated on the stack and is owned by the function thus we can't return a reference to it or as a borrowed type because it's owned by the function scope
            // &instance //// it's not ok to return a reference to `instance` since `instance` is a local variable which is owned by the current function and its lifetime is valid as long as the function is inside the stack and executing which means after executing the function its lifetime will be dropped
            let instance = &Pack{}; //// since we're allocating nothing on the stack inside this function thus by creating the instance directly using the the Pack struct and without calling the new() method (which is already lives in memory with long enough lifetime) we can return a reference to the location of the instance of the pack from the function and the reference will be stored inside the caller (where this function has called)
            instance //// it's ok to return a reference to `instance` since the instance does not allocate anything on the stack thus taking a reference to already allocated memory with long enough lifetime is ok since the allocated memory is happened in struct definition line
	    }

        // struct Taker{}
        // fn run_taker_mut(taker: &mut Taker) -> &mut Taker{
        //     //// for mutable reference the underlying type must be mutable
        //     //// thus rust will allocate mut a temp Taker first in the ram 
        //     //// (stack or heap depends on the types of the Taker struct) and when 
        //     //// we want to return &mut Taker it'll return a mutable pointer
        //     //// to the temp value inside the ram which is owned by the current 
        //     //// function but it's ok to return &Traker since rust allocate no
        //     //// space inside the ram for this and directly will return the Taker
        //     //// struct on the fly to the caller
        //     let oochik = &mut Taker{}; 
        //     oochik
        //     // or
        //     // &mut Taker{} 
        // } 


	    // NOTE - argument can also be &mut u8
	    pub fn ref_str_other_pointer_lifetime(status: &u8) -> &str{ //// in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
            let name = "wildonion";
            name //// name has a lifetime as valid as the passed in status argument lifetime from the caller scope 

	    }

        // - it's ok to return pointer to a struct which has no fields cause it can allocate nothing on the stack and there is no data to be owned by the scope
        fn run_taker(taker: &mut Commander) -> &Commander{
            let instance = &Commander{}; //// instance allocate nothing on the stack since Commander has no fields
            instance
            // or
            // &Commander{} 
        }
        pub fn ref_to_str<'a>() -> &'a str{ //// we can't return &str since we need a lifetime to do so or the &str must be the field of the struct since self has longer lifetime than all the types inside the method 
            let name = "wildonion";
            name
        }

        fn ret_taker_mut(taker: &mut Commander) -> &mut Commander{
            taker //// we're good to return a pointer to the taker since is not owned by the function 
        }  

        
        /*  ----------------------- pointers notes -----------------------
        
            in a scope:
                if there is a mutable pointer of a type we can't have more than one of it
                also we can't have any immutable pointer to the type cause there is already
                a mutable pointer exists and if there is an immutable pointer of a type we 
                can have more than one but we can't have any mutable pointer to the type cause
                only one mutable pointer can be exists in the whole scope.

            lifetime of a heap data will be dropped once it gets moved into a new scope
            cause rust doesn't support gc, now if the type is behind a pointer or shared 
            reference rust won't let us move the type or get its owned version of it in 
            the first place, rust doing this is because we can't shouldn't have dangling pointer 
            which is a pointer is pointing to no where! solution to this is either cloning 
            the type or move the borrow of it, also is good to know that we can't pass a 
            mutable borrow if there is already an immutable borrow of type cause we can 
            only have one mutable reference of the type but multiple immutable ones in each 
            scope but not both at the same time means that:
                1 - if there's an immutable reference to a value, you can't create a mutable 
                    reference to the same value while the immutable reference is still in scope.
                2 - if there's a mutable reference to a value, you can't create an immutable 
                    reference to the same value while the mutable reference is still in scope.
            
            dangling pointer is a pointer which is pointing to no where which might be 
            happened due to moving type into other scopes when it's behind a pointer and 
            because of this rust won't let us to do so.
            
            in the first palce we should borrow it some how using as_ref(), clone() or &
            we can return a reference from a method to a type that allocates
            nothing on the stack like returning Pack{} directly without storing 
            it inside a variable but we can't return a pointer to a type that is 
            owned by the that method since that type is a local variable which 
            has defined inside the method, allocated space on the stack and once 
            the method gets executed its lifetime will be dropped from the ram thus 
            its pointer will be remained a dangling pointer which rust doesn't allow 
            us to return the pointer to the local type in the first place in other 
            words a type can't be moved if it's behind a pointer.

            if we want to return a double pointer the first one can be 
            allocate nothing but in order to pointing to the first one 
            the first one must be allocate space on the ram thus the in 
            following we can't return ref_ since ref_ is a double pointer
            to the self in which self is a reference to the instance in 
            the first param of the method thus returning ref_ from the 
            function is not possible because &Pack allocate nothing on 
            the stack which is self but &&Pack must points to an allocated 
            space on the stack or pointing to &Pack or self on the stack
            or heap thus the self must be stored some where on the stack or 
            heap then do this but rust doesn't allow us to return a pointer 
            to a type which is owned by the function and is on the heap 
            since once the function gets executed the type will be dropped 
            and pointer will be converted into a dangling pointer.  
        
            Pack is zero sized type (ZST) and will be stored on the stack or 
            heap depends on how is being used at runtime
            
            we cannot return a &&Pack directly because the inner reference 
            (&Pack) is bound to the local scope of the function once the function returns, 
            the inner reference would be invalid, and Rust's borrow checker prevents us 
            from doing this to ensure memory safety.
        
        */
        //// in the following the return type is a double pointer
        //// which the first one must gets allocated on the satck 
        //// first in order to return a pointer to that and since
        //// we've allocated something on the ram (stack or heap) 
        //// which is owned by current function thus we can't return
        //// a pointer to the type which that type is owned by the 
        //// function.
        // fn as_ref__(&self) -> &&Pack{ 
        //     let ref ref_ = self; 
        //     // &self //// can't return &self since self is owned by the function also because self is borrowed
        //     ref_ //// can't return ref_ since self is owned by the function also because self is borrowed
        // }

        // ---------------- THUS             
        // - can't move out of a type if it's behind a pointer but we can pass by ref
        // - can't return pointer to a heap data which is owned by the function we can use Box instead but we're okey to return them or their slice with a valid lifetime
        // ----------------
        //// can't move the type if it's behind a pointer and doesn't implement copy trait (heap data) 
        //// since we can borrow mutably and return ref to stack types from function but not heap data 
        //// thus any reference to the instance or the struct itself which contains a heap data field 
        //// is not possible because heap data types are not Copy and once the scope that contains them
        //// gets executed they will be dropped from the ram and any pointer to them will be converted 
        //// into the a dangling pointer which rust doesn't allow us to do this in the first place. 
        fn as_ref(&self) -> &Pack{
            let ref_ = self;
            //// here we're returning the self or ref_ which is an immutable pointer of Pack instance 
            // ref_
            self 
        }

        fn as_ref_(&self) -> &Pack{
            let ref ref_ = self; 
            let ref__ = &self; 
            ref__
        }

        // pub fn ref_to_str() -> HashMap<&str, &str>{ //// we can't return &str since we need a specific lifetime to do so
        //     let names = HashMap::new();
        //     names.insert("wildonion", "another_wildonion");
        //     names
        // }

        //// in here we're actually implementing the trait for the return type
        //// also the return type must implement the Interface trait in order 
        //// to be able to return its instance,  
        pub fn ref_to_trait(&self) -> &dyn Interface{
            &Pack{}
        }

        pub fn ref_to_trait__(&self) -> &dyn Interface{
            self
        }

	    // NOTE - first param can also be &mut self; a mutable reference to the instance and its fields
	    // NOTE - this technique is being used in methods like as_mut() in which it'll return a mutable
        //        reference to the data using the self parameter lifetime.
        pub fn ref_to_str_other_self_lifetime(&self) -> &str{ //// in this case we're good to return the pointer from the function or send a copy to the caller's space since we can use the lifetime of the first param which is &self which is a borrowed type (it's a shared reference means that other methods are using it in their scopes) of the instance and its fields (since we don't want to lose the lifetime of the created instance from the contract struct after calling each method) and have a valid lifetime (as long as the instance of the type is valid) which is generated from the caller scope by the compiler to return the pointer from the function
            let name = "wildonion";
            name //// name has a lifetime as valid as the first param lifetime which is a borrowed type (it's a shared reference means that other methods are using it in their scopes) of the instance itself and its fields and will borrow the instance when we want to call the instance methods
	    }

	    // NOTE - 'a lifetime has generated from the caller scope by the compiler
	    pub fn ref_to_str_specific_lifetime<'a>(status: u8) -> &'a str{ //// in this case we're good to return the pointer from the function or copy to the caller's space since we've defined a valid lifetime for the pointer of the return type to return the pointer from the function which &'a str
            let name = "wildonion";
            name //// name has a lifetime as valid as the generated lifetime from the caller scope by the compiler and will be valid as long as the caller scope is valid
	    }

        // NOTE - use 'static lifetime in order to be able to return &str from the function since rust doesn't allow to return reference by default unless the return type has a valid and defined lifetime
	    // NOTE - 'static lifetime will be valid as long as the whole lifetime of the caller scope (it can be the main function which depends on the whole lifetime of the app)
	    pub fn ref_to_str_static() -> &'static str{
            let name = "wildonion";
            name //// name has static lifetime valid as long as the whol lifetime of the caller scope which can be the main function which will be valid as long as the main or the app is valid
	    }
		
        /*
        
            if we try to return a reference to a local String or Vec or heap data 
            variables within a function, we will run into lifetime issues, since 
            the local variable is dropped when the function goes out of scope.
            thus we can return the them in their slice form like &str for String
            &[u8] for Vec with a specific lifetime or a lifetime which lives long
            enough if the function gets executed like 'static lifetime, note that 
            we can't return them behind a valid reference at all since they're owned
            by the function scope and no matter how they cant be used! a solution to
            that is to define them as the struct field and return the pointer to the 
            self.field_name since self has longer lifetime than all the local types
            inside the method. 
        
        */
        // fn ret<'a>(name: String) -> &'a Vec<String>{
        //     //// this type is owned by the current function 
        //     //// thus if there is any pointer of this type 
        //     //// exists we can't return that pointer since 
        //     //// once the function gets executed all the types
        //     //// inside the function will be dropped from the ram 
        //     //// and any pointer to them will be dangled.
        //     //
        //     //// we can't return a pointer to the String 
        //     //// from the function since Strings or Vecs
        //     //// are heap data types and once the function 
        //     //// gets executed their lifetime will be dropped
        //     //// from the ram to free the allocations and 
        //     //// because of this returning a pointer to them 
        //     //// might be a dangling pointer which rust doesn't
        //     //// allow us to do this in the first place.
        //     // let names = vec!["wildonion".to_string()];
        //     // &names
        // }

	    //// ERROR - can't return a reference to heap allocated data structure from function due to their unknown size at compile time and they are temprary value
	    // pub fn ref_to_string<'s>() -> &'s String{
	    //     let name = &"wildonion".to_string();
	    //     name //// ERROR - we can't return this or &"wildonion".to_string() since they are temporary value due to the fact that heap data structure's size are not specific at compile time and they are some kina a temporary value thus heap data structures can't be returned in their borrowed form from the function since their size are not specific at compile time therefore by taking a pointer to the location of them we might have dangling pointer later once their location gets dropped during the function lifetime body 
	    // }

	    pub fn ref_to_num<'n>() -> &'n i32{
            let num = 23;
            // &num //// ERROR - we can't return this since the num is owned by the current function and returning the reference to the local variable which is owned by the function is denied
            &23 //// we can return &23 since we did allocate nothing on the stack inside the function (which this can be done by creating a local variable inside the function) and we're just returning a pointer to the location of a number directly   

	    }

        // NOTE - here we couldn't return its &str since this is 
        //        owned by the function and its lifetime will be dropped once the function 
        //        gets executed thus we can't return a pointer to &str or its utf8 bytes 
        //        because its pointer might be a dangling one in the caller space since 
        //        we don't have that String anymore inside the function! this is different
        //        about the &str in the first place cause we're cool with returning them
        //        because they are behind a pointer and kinda stack data types.
        pub const fn test(name: &String) -> &str{ // we can return &str in here sicne we're using the lifetime of the passed in param which is &String thus it's ok to use that reference (the reference to the passed in String) to return a &str (since its lifetime is valid as long as the passed in param is valid)
            WO // we must return const value from the constant function
        }

        pub fn closure_are_traits() -> impl FnOnce(String) -> String{ //// returning a closure from the function since closures are traits we can use -> impl ... syntax to implement the FnOnce for the return type 
            |name: String|{
                name
            }
        }

        pub fn run() -> impl std::future::Future<Output=u8>{ //// implementing the Future trait for the return type of the function by doing this we have to return an async block from the function
            async move{ //// returning an async block from the function
                26
            }
            // let res = run.await;
            // let res = Pack::run().await;
        }

        //// the following is another way of defining async method 
        // pub async fn run() -> u8{
        //     26
        //     // let res = run.await;
        //     // let res = Pack::run().await;
        // }

        pub async fn _run() -> u8{ //// above implementation is equivalent to this one 
            26

            // let res = run.await;
        }

        pub async fn unpack_self(&self) -> (){
            let Pack{..} = self; //// unpacking self into the struct itself, there is no explicit field naming and we filled all the fields using `..`
        }

	}


    //-----------

    let mut arr = [1u8; 32];
    let mut slice = &mut arr;
    slice[0] = 3;

    // here arr is mutated too
    // ...

    //-----------


    /* 
    
        a type can't be mvoed if it's being used by other scopes 
        or it's behind a shared pointer like &self thus we must either
        clone it or borrow it 

        a pointer to a type can't be returned from a method if the type
        is owned by current function and is a local variable, solutions 
        to this is using a valid lifetime for the pointer like 'static
        or having the local type as the struct field to return the &self.field
        which uses the self lifetime
    
    */


    trait HasId{
        type Id;
        fn ret_id(&self) -> &Self::Id;
    }

    struct Node{
        pub id: u32,
        pub slice: [u32; 1]
    }

    /* 
        can't return a pointer to heap data like String or Vec from method  
        since heap data will be dropped from the ram once the method gets 
        executed thus returning a pointer to them is useless and a dangling, 
        unless we use their slice form like &str and &[] with either an static 
        or a valid lifetime. 
    */
    impl Node{
        fn get_id_str_(&self) -> &'static str{

            // can't return pointer to String here
            // &self.id.to_string();

            /* 
                we cannot obtain &'static str from a String because Strings may not live 
                for the entire life of our program, and that's what &'static lifetime means. 
                we can only get a slice parameterized by String own lifetime from it, we can 
                obtain a static str but it involves leaking the memory of the String. this is 
                not something we should do lightly, by leaking the memory of the String, this 
                guarantees that the memory will never be freed (thus the leak), therefore, any 
                references to the inner object can be interpreted as having the 'static lifetime.
                
                also here it's ok to return the reference from function since our reference lifetime 
                is static and is valid for the entire life of the app
            */
            pub fn string_to_static_str(s: String) -> &'static str { 
                /* 
                    leaking the memory of the heap data String which allows us to have an 
                    unfreed allocation that can be used to define static str using it since
                    static means we have static lifetime during the whole lifetime of the app
                    and reaching this using String is not possible because heap data types 
                    will be dropped from the heap once their lifetime destroyed in a scope
                    like by moving them into another scope hence they can't be live longer 
                    than static lifetime

                    Note: this will leak memory! the memory for the String will not be freed 
                    for the remainder of the program. Use this sparingly
                */
                Box::leak(s.into_boxed_str()) 
            }

            let string: String = self.id.to_string();
            let new_static_str = string_to_static_str(string);
            new_static_str
       
       
        }
    }

    /* 
        can't return a pointer to a temp slice types from method  
        since in Rust, local variables (like your arr variable) 
        are stored on the stack and are deallocated at the end of 
        the enclosing scope. This means that after get_id_() returns, 
        the memory where arr was stored is reclaimed, and the 
        reference to arr is no longer valid, when we try to return 
        &[u32] from get_id_(), you are returning a reference to a 
        temporary slice that no longer exists after the function call, 
        which is why Rust prevents this with its ownership rules.


        solution to this is:
            - return a pointer to slice with a valid lifetime
            - include a slice as part of Node or return a pointer 
                to slice using &self which is a field of the struct 
                since self has longer lifetime (use the lifetime of the &self)
    */
    impl Node{
        fn get_id_(&self) -> &[u32]{
            
            /* 
                can't retrun ref to a local var or a temp slice owned by the method
                if we have a slice that's not 'static, it means the data it points to 
                could be freed at some point, making it 'static requires ensuring that 
                the data it points to lives for the entirety of the program.
            */
            // let arr = &[self.id];
            // arr

            &self.slice

        }
    }

    impl Node{
        fn get_id__(&self) -> &'static [u32]{
            
            /* 
                we cannot obtain &'static str from a Vec because Vecs may not live 
                for the entire life of our program, and that's what &'static lifetime means. 
                we can only get a slice parameterized by Vec own lifetime from it, we can 
                obtain a static str but it involves leaking the memory of the Vec. this is 
                not something we should do lightly, by leaking the memory of the Vec, this 
                guarantees that the memory will never be freed (thus the leak), therefore, any 
                references to the inner object can be interpreted as having the 'static lifetime.
                
                also here it's ok to return the reference from function since our reference lifetime 
                is static and is valid for the entire life of the app
            */
            pub fn vector_to_static_slice(s: Vec<u32>) -> &'static [u32] { 
                /* 
                    leaking the memory of the heap data Vec which allows us to have an 
                    unfreed allocation that can be used to define static str using it since
                    static means we have static lifetime during the whole lifetime of the app
                    and reaching this using Vec is not possible because heap data types 
                    will be dropped from the heap once their lifetime destroyed in a scope
                    like by moving them into another scope hence they can't be live longer 
                    than static lifetime

                    Note: this will leak memory! the memory for the Vec will not be freed 
                    for the remainder of the program. Use this sparingly
                */
                Box::leak(s.into_boxed_slice()) 
            }

            let vec = vec![self.id];
            let new_static_slice = vector_to_static_slice(vec);
            new_static_slice

        }
    }

    /* 
        the reason that get_id(&self) -> &u32 is allowed is because self.id 
        is owned by self, which presumably exists for longer than the scope 
        of the get_id() call, since self can be accessible out of the method too
        or where the instance has initialized, in this case, returning &u32 
        is safe because self will not be deallocated unless the caller of get_id()
        or the instance of the Node struct gets done with the returned reference
    */
    impl Node{
        fn get_id(&self) -> &u32{
            &self.id
        }

    }

    impl HasId for Node{
        type Id = u32;
        fn ret_id(&self) -> &Self::Id {
            &self.id
        }
    }

    let mut node = Node{
        id: 32,
        slice: [32] /* no need to take a ref to [] since it's sized with 1 element */
    };
    let id = node.get_id();
    node.id = 23;


}