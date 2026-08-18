#ifndef BF_RUNTIME_H_
#define BF_RUNTIME_H_

/*
 * Brainfuck Native Interface (BFNI) ABI
 * =====================================
 *
 * A generated BF function has the following shape:
 *
 *     bf_bool_t name(
 *         BFCalls calls,
 *         BFRuntimeEnv* env,
 *         BFRuntimeReport* report
 *     );
 *
 * The runtime owns the tape, I/O devices, and BFRuntimeEnv. A generated
 * function borrows them for the duration of the call and never allocates,
 * frees, or retains runtime-owned storage.
 *
 * Caller preconditions
 * --------------------
 * Before calling a generated BF function, the runtime must guarantee that:
 *
 *   - report is non-null and points to writable storage.
 *   - Every function pointer in calls is non-null and callable.
 *   - calls.tape_len(env) returns a value greater than zero.
 *   - calls.get_tape(env) returns a non-null pointer to at least
 *     calls.tape_len(env) consecutive, writable bf_byte_t cells.
 *   - The tape pointer and tape length remain valid and unchanged until the
 *     generated function returns.
 *
 * env is opaque to generated code and is passed unchanged to every runtime
 * callback. Whether env itself may be null is defined by the runtime.
 * Violating a caller precondition results in undefined behavior.
 *
 * BF machine semantics
 * --------------------
 * A cell is an unsigned 8-bit value. Addition and subtraction wrap modulo
 * 256. This ABI therefore requires a target on which unsigned char is exactly
 * 8 bits. The data pointer starts at tape offset zero.
 *
 * Initial tape contents are runtime-defined. They may be zeroed, preloaded,
 * or preserved from an earlier invocation. A BF program must not assume
 * zero-initialized cells unless its runtime contract guarantees them. Changes
 * made by a generated function remain in the runtime-owned tape on return.
 *
 * With boundary checks enabled, an attempt to move the data pointer before
 * the first cell or past the last cell fails before the new position is used.
 * With boundary checks disabled (unsafe mode), the BF program must keep the
 * data pointer in bounds; an out-of-bounds access is undefined behavior.
 *
 * Result and failure semantics
 * ----------------------------
 * On entry, generated code initializes report, including setting error_kind
 * to NoBFError. BF_TRUE means the program completed successfully. BF_FALSE
 * means execution stopped at the first reported boundary or I/O failure.
 *
 * A failure is not transactional: tape changes, input consumption, and
 * output completed before the failure are not rolled back.
 */

typedef unsigned long bf_size_t;
typedef unsigned char bf_byte_t;
typedef unsigned long bf_off_t;
typedef char bf_bool_t;

static const bf_bool_t BF_TRUE = 1;
static const bf_bool_t BF_FALSE = 0;

typedef struct BFRuntimeEnv BFRuntimeEnv;

typedef enum {
    /* The generated function has not encountered an error. */
    NoBFError,

    /* A checked move attempted to pass the beginning of the tape. */
    BFErrorUnderflow,

    /* A checked move attempted to pass the end of the tape. */
    BFErrorOverflow,

    /* getchar reported EOF or another input failure. */
    BFErrorInStream,

    /* putchar reported an output failure. */
    BFErrorOutStream,
} BFErrorKind;

typedef struct BFRuntimeReport BFRuntimeReport;
struct BFRuntimeReport {
    /* Static strings owned by the generated code; the runtime must not free. */
    const char* file_name;
    const char* func_name;

    /* NoBFError on entry/success, otherwise the reason for BF_FALSE. */
    BFErrorKind error_kind;
};

typedef struct BFCalls BFCalls;
struct BFCalls {
    /* Return the number of addressable cells. Must return at least one. */
    bf_size_t(*tape_len)(BFRuntimeEnv*);

    /*
     * Return the first cell of the runtime-owned tape. Its initial contents
     * are runtime-defined. The returned storage must satisfy the caller
     * preconditions documented above.
     */
    bf_byte_t*(*get_tape)(BFRuntimeEnv*);

    /*
     * Read one byte into dst. Return BF_TRUE on success. Return BF_FALSE on
     * EOF or any input failure; dst is then unspecified. Generated code maps
     * BF_FALSE to BFErrorInStream and stops execution.
     */
    bf_bool_t(*getchar)(BFRuntimeEnv*, bf_byte_t* dst);

    /*
     * Write one byte. Return BF_TRUE on success or BF_FALSE on any output
     * failure. Generated code maps BF_FALSE to BFErrorOutStream and stops
     * execution.
     */
    bf_bool_t(*putchar)(BFRuntimeEnv*, bf_byte_t);
};

/* Example generated entry-point declaration. */
/* bf_bool_t bf_code_entry(BFCalls, BFRuntimeEnv*, BFRuntimeReport*); */

#endif /* BF_RUNTIME_H_ */
