#include <bfni.h>

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

static const bf_size_t DEFAULT_TAPE_LEN = 4096;

struct BFRuntimeEnv {
    bf_size_t tape_len;
    bf_byte_t* tape;
};

static bf_bool_t env_init(BFRuntimeEnv* env, bf_size_t len) {
    void* tape = calloc(len, sizeof(bf_byte_t));
    if (NULL == tape) return BF_FALSE;
    
    *env = (BFRuntimeEnv){
        .tape = tape,
        .tape_len = len,
    };

    return BF_TRUE;
}

static void env_dtor(BFRuntimeEnv* env) {
    free(env->tape);
}

static bf_size_t tape_len(BFRuntimeEnv* env) {
    return env->tape_len;
}

static bf_byte_t* get_tape(BFRuntimeEnv* env) {
    return env->tape;
}

static bf_bool_t host_getchar(BFRuntimeEnv* _, bf_byte_t* dst) {
    int c = getchar();
    if (EOF == c) return BF_FALSE;
    
    *dst = c;
    
    return BF_TRUE;
}

static bf_bool_t host_putchar(BFRuntimeEnv* _, bf_byte_t c) {
    if (EOF == putchar(c)) return BF_FALSE;

    return BF_TRUE;
}

static const BFCalls CALLS = (BFCalls){
    .get_tape = get_tape,
    .tape_len = tape_len,
    .getchar = host_getchar,
    .putchar = host_putchar,
};

extern bf_bool_t bf_entry(BFCalls, BFRuntimeEnv*, BFRuntimeReport*);

static bf_bool_t parse_tape_len(
    const char* raw,
    bf_size_t* result
) {
    if (NULL == raw) {
        return BF_FALSE;
    }

    for (const char* iter = raw; '\0' != *iter; ++iter) {
        char c = *iter;
        if (c < '0' || c > '9') {
            return BF_FALSE;
        }
    }
    
    errno = 0;

    char* end = NULL;
    unsigned long value = strtoul(raw, &end, 10);

    if (ERANGE == errno) {
        return BF_FALSE;
    }

    if (end == raw) {
        return BF_FALSE;
    }

    if (0 == value) {
        return BF_FALSE;
    }

    *result = value;
    return BF_TRUE;
}

int main(int argc, const char** argv) {
    bf_size_t tape_len = DEFAULT_TAPE_LEN;
    if (argc == 2) {
        if (!parse_tape_len(argv[1], &tape_len)) {
            fprintf(stderr, "Invalid tape length: %s\n", argv[1]);
            return 1;
        }
    } else if (argc > 2) {
        fprintf(stderr, "Too many arguments: %d\n", argc - 1);
        return 1;
    }

    BFRuntimeEnv env = { 0 };
    if (!env_init(&env, tape_len)) {
        fprintf(stderr, "Out of memory.\n");
        return 1;
    }

    BFRuntimeReport report = { 0 };

    int ret = 0;
    if (!bf_entry(CALLS, &env, &report)) {
        fprintf(stderr, "An error occurred in function '%s' of file '%s': ", report.func_name, report.file_name);

        switch (report.error_kind) {
        case BFErrorInStream:
            fprintf(stderr, "bad input stream.\n");
            break;
            
        case BFErrorOutStream:
            fprintf(stderr, "bad output stream.\n");
            break;
            
        case BFErrorUnderflow:
            fprintf(stderr, "tape underflow.\n");
            break;
            
        case BFErrorOverflow:
            fprintf(stderr, "tape overflow.\n");
            break;
            
        case NoBFError:
            fprintf(stderr, "unreachable.\n");
        }
        
        ret = 1;
    }

    if (EOF == fflush(stdout)) {
        fprintf(stderr, "Failed to flush output stream.\n");
        ret = 1;
    }

    env_dtor(&env);
    
    return ret;
}
