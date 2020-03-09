#![feature(decl_macro)]

pub use typed_arena;

pub trait Intern<'a>: Sized {
    type Key;
    
    fn intern<I>(self, i: &I) -> Self::Key
    where
        I: Interner<'a, Self> + 'a,
        Self: 'a;
}

pub trait Interner<'a, I: Intern<'a> + 'a> {
    fn intern(&self, value: I) -> I::Key;
}

pub trait InternerExt<'a, I: Intern<'a> + 'a>: Interner<'a, I> {
    type IterMut: Iterator<Item=&'a mut I>;
    
    fn iter_mut(&'a mut self) -> Self::IterMut;
}

#[macro_export]
macro_rules! interner {
    ($name:ident, $type:ident<$lt:lifetime>, $key:ident<$lt2:lifetime>) => {
        $crate::interner!(imp $name, $type<$lt>, $key<'a>);
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $key<'a>(*mut $type<'a>, ::std::marker::PhantomData<&'a mut $type<'a>>);
        
        impl<'a> $key<'a> {
            pub fn new(val: &'a mut $type<'a>) -> $key<'a> {
                $key(val, ::std::marker::PhantomData)
            }
        }
        
        impl<'a> ::std::default::Default for $key<'a> {
            fn default() -> $key<'a> {
                $key(::std::ptr::null_mut(), ::std::marker::PhantomData)
            }
        }
        
        impl<'a> ::std::ops::Deref for $key<'a> {
            type Target = $type<'a>;
            
            fn deref(&self) -> &$type<'a> {
                unsafe { &*self.0 }
            }
        }
        
        impl<'a> ::std::ops::DerefMut for $key<'a> {
            fn deref_mut(&mut self) -> &mut $type<'a> {
                unsafe { &mut *self.0 }
            }
        }

        impl<'a> ::std::hash::Hash for $key<'a> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                (&**self).hash(state);
            }
        }
    };
    
    ($name:ident, $type:ident, $key:ident<$lt2:lifetime>) => {
        $crate::interner!(imp $name, $type, $key<'a>);
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $key<$lt2>(*mut $type, ::std::marker::PhantomData<&$lt2 mut $type>);
        
        impl<'a> $key<'a> {
            pub fn new(val: &'a mut $type) -> $key<'a> {
                $key(val, ::std::marker::PhantomData)
            }
        }
        
        impl<'a> ::std::default::Default for $key<'a> {
            fn default() -> $key<'a> {
                $key(::std::ptr::null_mut(), ::std::marker::PhantomData)
            }
        }
        
        impl<'a> ::std::ops::Deref for $key<'a> {
            type Target = $type;
            
            fn deref(&self) -> &$type {
                unsafe { &*self.0 }
            }
        }
        
        impl<'a> ::std::ops::DerefMut for $key<'a> {
            fn deref_mut(&mut self) -> &mut $type {
                unsafe { &mut *self.0 }
            }
        }

        impl<'a> ::std::hash::Hash for $key<'a> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                (&**self).hash(state);
            }
        }
    };
    
    ($name:ident, $type:ident $(<$lt:lifetime>)?, $key:ident) => {
        $crate::interner!(imp $name, $type $(<$lt>)?, $key);
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $key(*mut $type, ::std::marker::PhantomData<$type>);
        
        impl $key {
            pub fn new(val: &mut $type) -> $key {
                $key(val, ::std::marker::PhantomData)
            }
        }
        
        impl ::std::default::Default for $key {
            fn default() -> $key {
                $key(::std::ptr::null_mut(), ::std::marker::PhantomData)
            }
        }
        
        impl ::std::ops::Deref for $key {
            type Target = $type;
            
            fn deref(&self) -> &$type {
                unsafe { &*self.0 }
            }
        }
        
        impl ::std::ops::DerefMut for $key {
            fn deref_mut(&mut self) -> &mut $type {
                unsafe { &mut *self.0 }
            }
        }

        impl ::std::hash::Hash for $key {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                (&**self).hash(state);
            }
        }
    };
    
    (imp $name:ident, $type:ident $(<$lt:lifetime>)?, $key:ty) => {
        pub struct $name $(<$lt>)? {
            storage: $crate::typed_arena::Arena<$type $(<$lt>)?>,
        }
        
        impl $(<$lt>)? $name $(<$lt>)? {
            pub fn new() -> $name $(<$lt>)? {
                $name {
                    storage: $crate::typed_arena::Arena::new(),
                }
            }
        }
        
        $crate::interner!(imp2 $name, $type $(, $lt)?, $key);
    };
    
    (imp2 $name:ident, $type:ident, $lt:lifetime, $key:ty) => {
        impl<'a> $crate::Intern<'a> for $type<'a> {
            type Key = $key;
            
            fn intern<I: $crate::Interner<'a, Self> + 'a>(self, i: &I) -> $key {
                i.intern(self)
            }
        }
    
        impl<'a> $crate::Interner<'a, $type<'a>> for $name<'a> {
            fn intern(&self, value: $type<'a>) -> $key {
                <$key>::new(self.storage.alloc(value))
            }
        }
    };
    
    (imp2 $name:ident, $type:ident, $key:ty) => {
        impl<'a> $crate::Intern<'a> for $type {
            type Key = $key;
            
            fn intern<I: $crate::Interner<'a, Self> + 'a>(self, i: &I) -> $key {
                i.intern(self)
            }
        }
    
        impl<'a> $crate::Interner<'a, $type> for $name {
            fn intern(&self, value: $type) -> $key {
                <$key>::new(self.storage.alloc(value))
            }
        }
    };
}
