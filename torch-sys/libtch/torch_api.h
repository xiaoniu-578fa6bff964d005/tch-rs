#ifndef __TORCH_API_H__
#define __TORCH_API_H__
#include<stdint.h>

#ifdef __cplusplus
thread_local char *torch_last_err = nullptr;

extern "C" {
typedef torch::Tensor *tensor;
typedef torch::Scalar *scalar;
typedef torch::optim::Optimizer *optimizer;
typedef torch::jit::script::Module *module;
typedef torch::jit::IValue *ivalue;
#define PROTECT(x) \
  try { \
    x \
  } catch (const exception& e) { \
      torch_last_err = strdup(e.what()); \
  }
#else
typedef void *tensor;
typedef void *optimizer;
typedef void *scalar;
typedef void *module;
typedef void *ivalue;
#endif

char *get_and_reset_last_err(); // thread-local
void at_manual_seed(int64_t);
tensor at_new_tensor();
tensor at_tensor_of_data(void *vs, int64_t *dims, size_t ndims, size_t element_size_in_bytes, int type);
void at_copy_data(tensor tensor, void *vs, size_t numel, size_t element_size_in_bytes);
tensor at_shallow_clone(tensor);

int at_defined(tensor);
int at_device(tensor);
size_t at_dim(tensor);
void at_shape(tensor, int64_t *);
int at_scalar_type(tensor);

void at_backward(tensor, int, int);
int at_requires_grad(tensor);
int at_grad_set_enabled(int);

tensor at_get(tensor, int index);
void at_fill_double(tensor, double);
void at_fill_int64(tensor, int64_t);

double at_double_value_at_indexes(tensor, int64_t *indexes, int indexes_len);
int64_t at_int64_value_at_indexes(tensor, int64_t *indexes, int indexes_len);
void at_set_double_value_at_indexes(tensor, int *indexes, int indexes_len, double v);
void at_set_int64_value_at_indexes(tensor, int *indexes, int indexes_len, int64_t v);

void at_copy_(tensor dst, tensor src);

void at_print(tensor);
char *at_to_string(tensor, int line_size);
void at_save(tensor, char *filename);
tensor at_load(char *filename);
tensor at_load_image(char *filename);
int at_save_image(tensor, char *filename);
tensor at_resize_image(tensor, int w, int h);

void at_save_multi(tensor *tensors, char **tensor_names, int ntensors, char *filename);
/* [at_load_multi] takes as input an array of nullptr for [tensors]. */
void at_load_multi(tensor *tensors, char **tensor_names, int ntensors, char *filename);
/* [at_load_multi_] takes as input an array of allocation [tensors]. */
void at_load_multi_(tensor *tensors, char **tensor_names, int ntensors, char *filename);

void at_load_callback(char *filename, void *data, void (*f)(void *, char *, tensor));

void at_free(tensor);

void at_run_backward(tensor *tensors,
                      int ntensors,
                      tensor *inputs,
                      int ninputs,
                      tensor *outputs,
                      int keep_graph,
                      int create_graph);

optimizer ato_adam(double learning_rate,
                   double beta1,
                   double beta2,
                   double weight_decay);
optimizer ato_rms_prop(double learning_rate,
                       double alpha,
                       double eps,
                       double weight_decay,
                       double momentum,
                       int centered);
optimizer ato_sgd(double learning_rate,
                  double momentum,
                  double dampening,
                  double weight_decay,
                  int nesterov);
void ato_add_parameters(optimizer, tensor *, int ntensors);
void ato_set_learning_rate(optimizer, double learning_rate);
void ato_set_momentum(optimizer, double momentum);
void ato_zero_grad(optimizer);
void ato_step(optimizer);
void ato_free(optimizer);

scalar ats_int(int64_t);
scalar ats_float(double);
int64_t ats_to_int(scalar);
double ats_to_float(scalar);
char *ats_to_string(scalar);
void ats_free(scalar);

int atc_cuda_device_count();
int atc_cuda_is_available();
int atc_cudnn_is_available();
void atc_set_benchmark_cudnn(int b);

module atm_load(char *);
tensor atm_forward(module, tensor *tensors, int ntensors);
ivalue atm_forward_(module,
                    ivalue *ivalues,
                    int nivalues);
void atm_free(module);

ivalue ati_tensor(tensor);
ivalue ati_int(int64_t);
ivalue ati_double(double);
ivalue ati_tuple(ivalue *, int);

tensor ati_to_tensor(ivalue);
int64_t ati_to_int(ivalue);
double ati_to_double(ivalue);
int ati_tuple_length(ivalue);
void ati_to_tuple(ivalue, ivalue *, int);

int ati_tag(ivalue);

void ati_free(ivalue);
void ati_free_deep(ivalue);

#include "torch_api_generated.h"

#ifdef __cplusplus
};
#endif

#endif
