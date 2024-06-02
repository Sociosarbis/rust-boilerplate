use std::sync::Arc;

macro_rules! type_erase {
  ($v: vis trait $t_name:ident => $erased_t:ident {
    $(fn $f:ident(&self $(,$p:ident: $t:ty $(,)?)*) $(-> $r:ty)?;)*
    $(
      impl $impl_name:path {
        $(
          fn $f_impl:ident(
            &self $(, $p_impl:ident: $t_impl:ty $(,)?)*
          ) $(-> $r_impl:ty)?;
        )*
      }
    )*
  }) => {

    $v struct $erased_t {
      ptr: *const (),
      vtable: *const(),
    }

    const _: () = {
      struct VTable {
        $($f: fn(*const(), $($p:$t),*) $(-> $r)?,)*
        $($($f_impl: fn(*const(),$($p_impl:$t_impl),*) $(-> $r_impl)?,)*)*
        __type_id: fn() -> std::any::TypeId,
        __type_name: fn() -> &'static str,
        __incref: fn(*const()),
        __decref: fn(*const()),
      }

      fn vt(e: &$erased_t) -> &VTable {
        unsafe { &*(e.vtable as *const VTable) }
      }

      impl $erased_t {
        $v fn new<T: $t_name + 'static>(v: std::sync::Arc<T>) -> Self {
          let ptr = std::sync::Arc::into_raw(v) as * const T as *const ();
          let vtable = &VTable {
            $($f: |ptr, $($p,)*| unsafe {
              let arc = std::mem::ManuallyDrop::new(std::sync::Arc::<T>::from_raw(ptr as *const T));
              <T as $t_name>::$f(&arc, $($p,)*)
            },)*
            // 这里用了双层$()是因为f_impl本身是在impl_name这个捕获之下
            $($(
              $f_impl: |ptr, $($p_impl,)*| unsafe {
                let arc = std::mem::ManuallyDrop::new(std::sync::Arc::<T>::from_raw(ptr as *const T));
                // Arc<T>的Deref会返回&T，但是解引用操作符会存在隐式转换
                // 相当于*Deref::deref(&self)，所以这里是&T -> T，最后又通过引用操作符转为&T
                <T as $impl_name>::$f_impl(&*arc, $($p_impl,)*)
              },
            )*)*
            __type_id: || std::any::TypeId::of::<T>(),
            __type_name: || std::any::type_name::<T>(),
            __incref: |ptr| unsafe {
              std::sync::Arc::<T>::increment_strong_count(ptr as *const T);
            },
            __decref: |ptr| unsafe {
              std::sync::Arc::from_raw(ptr as *const T);
            }
          };

          Self { ptr, vtable: vtable as *const VTable as *const () }
        }

        $($v fn $f(&self, $($p:$t),*) $(-> $r)? {
          (vt(self).$f)(self.ptr, $($p),*)
        })*

        $v fn type_name(&self) -> &'static str {
          (vt(self).__type_name)()
        }

        $v fn downcast_ref<T: 'static>(&self) -> Option<&T> {
          if ((vt(self).__type_id)() == std::any::TypeId::of::<T>()) {
            unsafe {
              return Some(&*(self.ptr as *const T));
            }
          }
          None
        }

        $v fn downcast<T: 'static>(&self) -> Option<Arc<T>> {
          if ((vt(self).__type_id)() == std::any::TypeId::of::<T>()) {
            unsafe {
              std::sync::Arc::<T>::increment_strong_count(self.ptr as *const T);
              return Some(std::sync::Arc::<T>::from_raw(self.ptr as *const T));
            }
          }
          None
        }
      }

      impl Clone for $erased_t {
        fn clone(&self) -> Self {
          (vt(self).__incref)(self.ptr);
          Self {
            ptr: self.ptr,
            vtable: self.vtable,
          }
        }
      }

      impl Drop for $erased_t {
        fn drop(&mut self) {
          (vt(self).__decref)(self.ptr);
        }
      }

      $(
        impl $impl_name for $erased_t {
          $(
            fn $f_impl(&self, $($p_impl,)*) $(-> $r_impl)? {
              (vt(self).$f_impl)(self.ptr, $($p_impl),*)
            }
          )*
        }
      )*
    };
  };
}

trait Time {
  fn now(&self) -> std::time::Instant;
}

// 因为receiver是&Arc<Self>所以不是object-safe
// object-safe表示可以声明trait object
trait Logger: Time {
  fn log(self: &Arc<Self>, msg: String);
  fn error(self: &Arc<Self>, msg: String, trace: Option<String>, context: String);
}

type_erase!(
  trait Logger => DynamicLogger {
    fn log(&self, msg: String);
    fn error(&self, msg: String, trace: Option<String>, context: String);

    impl Time {
      fn now(&self) -> std::time::Instant;
    }
  }
);

struct CustomLogger {}

impl Time for CustomLogger {
  fn now(&self) -> std::time::Instant {
    std::time::Instant::now()
  }
}

impl Logger for CustomLogger {
  fn log(self: &Arc<Self>, msg: String) {
    println!("log:{:?}", msg);
  }

  fn error(self: &Arc<Self>, msg: String, trace: Option<String>, context: String) {
    println!(
      "[{:?}]error:{:?}{:?}",
      context,
      msg,
      if let Some(t) = trace {
        format!("\n{t}")
      } else {
        String::default()
      }
    );
  }
}

trait NonDispatchable {
  fn method_a(&self)
  where
    Self: Sized,
  {
    println!("NonDispatchable: default impl");
  }
}

trait Dispatchable {
  fn method_a(&self) {
    println!("Dispatchable: default impl");
  }
}

struct A {}

impl NonDispatchable for A {
  fn method_a(&self) {
    println!("NonDispatchable: A impl");
  }
}

impl Dispatchable for A {
  fn method_a(&self) {
    println!("Dispatchable: A impl");
  }
}

impl<T: NonDispatchable + ?Sized> NonDispatchable for Box<T> {}

#[test]
fn test() {
  let logger_instance = DynamicLogger::new(Arc::new(CustomLogger {}));
  logger_instance.log("hello".to_string());
  logger_instance.error("hello".to_string(), None, "test".to_string());
  logger_instance.now();

  let a = Box::new(A {}) as Box<dyn NonDispatchable>;
  // 这个不会执行A impl的方法
  a.method_a();
  let a = Box::new(A {}) as Box<dyn Dispatchable>;
  // 这个会执行A impl的方法
  a.method_a();
}
