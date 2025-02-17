use libc::{c_char, c_int, c_void, size_t};

#[repr(C)]
pub struct C_scalar {
    _private: [u8; 0],
}

extern "C" {
    pub fn ats_int(v: i64) -> *mut C_scalar;
    pub fn ats_float(v: f64) -> *mut C_scalar;
    pub fn ats_to_int(arg: *mut C_scalar) -> i64;
    pub fn ats_to_float(arg: *mut C_scalar) -> f64;
    pub fn ats_to_string(arg: *mut C_scalar) -> *mut c_char;
    pub fn ats_free(arg: *mut C_scalar);
}

#[repr(C)]
pub struct C_tensor {
    _private: [u8; 0],
}

extern "C" {
    pub fn at_new_tensor() -> *mut C_tensor;
    pub fn at_shallow_clone(arg: *mut C_tensor) -> *mut C_tensor;
    pub fn at_copy_(dst: *mut C_tensor, src: *mut C_tensor);
    pub fn at_defined(arg: *mut C_tensor) -> c_int;
    pub fn at_backward(arg: *mut C_tensor, keep_graph: c_int, create_graph: c_int);
    pub fn at_print(arg: *mut C_tensor);
    pub fn at_to_string(arg: *mut C_tensor, line_size: c_int) -> *mut c_char;
    pub fn at_dim(arg: *mut C_tensor) -> size_t;
    pub fn at_get(arg: *mut C_tensor, index: c_int) -> *mut C_tensor;
    pub fn at_requires_grad(arg: *mut C_tensor) -> c_int;
    pub fn at_shape(arg: *mut C_tensor, sz: *mut i64);
    pub fn at_double_value_at_indexes(arg: *mut C_tensor, idx: *const i64, idx_len: c_int) -> f64;
    pub fn at_int64_value_at_indexes(arg: *mut C_tensor, idx: *const i64, idx_len: c_int) -> i64;
    pub fn at_free(arg: *mut C_tensor);
    pub fn at_run_backward(
        arg: *const *mut C_tensor,
        ntensors: c_int,
        inputs: *const *mut C_tensor,
        ninputs: c_int,
        outputs: *mut *mut C_tensor,
        keep_graph: c_int,
        create_graph: c_int,
    );
    pub fn at_copy_data(
        arg: *mut C_tensor,
        vs: *const c_void,
        numel: size_t,
        elt_size_in_bytes: size_t,
    );
    pub fn at_scalar_type(arg: *mut C_tensor) -> c_int;
    pub fn at_device(arg: *mut C_tensor) -> c_int;
    pub fn at_tensor_of_data(
        vs: *const c_void,
        dims: *const i64,
        ndims: size_t,
        elt_size_in_bytes: size_t,
        kind: c_int,
    ) -> *mut C_tensor;
    pub fn at_grad_set_enabled(b: c_int) -> c_int;
    pub fn at_save(arg: *mut C_tensor, filename: *const c_char);
    pub fn at_load(filename: *const c_char) -> *mut C_tensor;
    pub fn at_save_multi(
        args: *const *mut C_tensor,
        names: *const *const c_char,
        n: c_int,
        filename: *const c_char,
    );
    pub fn at_load_callback(
        filename: *const c_char,
        data: *mut c_void,
        f: extern "C" fn(*mut c_void, name: *const c_char, t: *mut C_tensor),
    );

    pub fn at_manual_seed(seed: i64);
}

pub mod c_generated;

extern "C" {
    pub fn atc_cuda_device_count() -> c_int;
    pub fn atc_cuda_is_available() -> c_int;
    pub fn atc_cudnn_is_available() -> c_int;
    pub fn atc_set_benchmark_cudnn(b: c_int);
}

extern "C" {
    pub fn get_and_reset_last_err() -> *mut c_char;
}

#[repr(C)]
pub struct C_optimizer {
    _private: [u8; 0],
}

extern "C" {
    pub fn ato_adam(lr: f64, beta1: f64, beta2: f64, wd: f64) -> *mut C_optimizer;
    pub fn ato_rms_prop(
        lr: f64,
        alpha: f64,
        eps: f64,
        wd: f64,
        momentum: f64,
        centered: c_int,
    ) -> *mut C_optimizer;
    pub fn ato_sgd(
        lr: f64,
        momentum: f64,
        dampening: f64,
        wd: f64,
        nesterov: c_int,
    ) -> *mut C_optimizer;
    pub fn ato_add_parameters(arg: *mut C_optimizer, ts: *const *mut C_tensor, n: c_int);
    pub fn ato_set_learning_rate(arg: *mut C_optimizer, lr: f64);
    pub fn ato_set_momentum(arg: *mut C_optimizer, momentum: f64);
    pub fn ato_zero_grad(arg: *mut C_optimizer);
    pub fn ato_step(arg: *mut C_optimizer);
    pub fn ato_free(arg: *mut C_optimizer);
}

extern "C" {
    pub fn at_save_image(arg: *mut C_tensor, filename: *const c_char) -> c_int;
    pub fn at_load_image(filename: *const c_char) -> *mut C_tensor;
    pub fn at_resize_image(arg: *mut C_tensor, out_w: c_int, out_h: c_int) -> *mut C_tensor;
}

#[repr(C)]
pub struct CIValue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CModule_ {
    _private: [u8; 0],
}

extern "C" {
    // Constructors
    pub fn ati_int(v: i64) -> *mut CIValue;
    pub fn ati_double(v: f64) -> *mut CIValue;
    pub fn ati_tensor(v: *mut C_tensor) -> *mut CIValue;
    pub fn ati_tuple(v: *const *mut CIValue, n: c_int) -> *mut CIValue;

    // Type query
    pub fn ati_tag(arg: *mut CIValue) -> c_int;

    // Getters
    pub fn ati_to_int(arg: *mut CIValue) -> i64;
    pub fn ati_to_double(arg: *mut CIValue) -> f64;
    pub fn ati_to_tensor(arg: *mut CIValue) -> *mut C_tensor;
    pub fn ati_tuple_length(arg: *mut CIValue) -> c_int;
    pub fn ati_to_tuple(arg: *mut CIValue, outputs: *mut *mut CIValue, n: c_int) -> c_int;

    pub fn ati_free(arg: *mut CIValue);
    pub fn ati_free_deep(arg: *mut CIValue);

    pub fn atm_load(filename: *const c_char) -> *mut CModule_;
    pub fn atm_forward(m: *mut CModule_, args: *const *mut C_tensor, n: c_int) -> *mut C_tensor;
    pub fn atm_forward_(m: *mut CModule_, args: *const *mut CIValue, n: c_int) -> *mut CIValue;
    pub fn atm_free(m: *mut CModule_);
}
