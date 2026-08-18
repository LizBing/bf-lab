#ifndef BF_RUNTIME_H_
#define BF_RUNTIME_H_

typedef unsigned long bf_size_t;
typedef unsigned char bf_byte_t;
typedef unsigned long bf_off_t;
typedef char bf_bool_t;

static const bf_bool_t BF_TRUE = 1;
static const bf_bool_t BF_FALSE = 0;

static const bf_byte_t* BF_NULL = ((bf_byte_t*)0);

typedef struct BFRuntimeEnv BFRuntimeEnv;

typedef enum {
    NoBFError,
    BFErrorOutOfMemory,
    BFErrorUnderflow,
    BFErrorOverflow,
    BFErrorInStream,
    BFErrorOutStream,
} BFErrorKind;

typedef struct BFRuntimeReport BFRuntimeReport;
struct BFRuntimeReport {
    const char* file_name;
    const char* func_name;
    BFErrorKind error_kind;
};

typedef struct BFCalls BFCalls;
struct BFCalls {
    bf_size_t(*tape_len)(BFRuntimeEnv*);

    // Tape memory should be zeroed.
    bf_byte_t*(*alloc_tape)(BFRuntimeEnv*, bf_size_t);
    void(*free_tape)(BFRuntimeEnv*, bf_byte_t*);
    
    // If stream operations fail, do handle the error and return false.
    // Otherwise, return true.
    bf_bool_t(*getchar)(BFRuntimeEnv*, bf_byte_t* dst);
    bf_bool_t(*putchar)(BFRuntimeEnv*, bf_byte_t);
};

// bf_bool_t bf_code_entry_example(BFCalls, BFRuntimeEnv*, BFRuntimeReport* report);

#endif /* BF_RUNTIME_H_ */
