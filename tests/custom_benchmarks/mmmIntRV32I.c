/**
 * mmm.c
 *
 * integer matrix-mult benchmark
 **/
__asm__(".include \"runtime/custom_instrs.S\"\n");
#define MSIZE 64

#define TEST 0

#define MAX_THREADS 256

static inline unsigned int tid() {
  unsigned int x;
  __asm__ __volatile__ ("tid %0" : "=r"(x));
  return x;
}

static inline unsigned int bid() {
  unsigned int x;
  __asm__ __volatile__ ("bid %0" : "=r"(x));
  return x;
}

static inline unsigned int bdim() {
  unsigned int x;
  __asm__ __volatile__ ("bdim %0" : "=r"(x));
  return x;
}

static inline unsigned int gdim() {
  unsigned int x;
  __asm__ __volatile__ ("gdim %0" : "=r"(x));
  return x;
}

/*
 * A and B are predefined by a generated header and placed in .data.
 *
 * The generated header should define:
 *
 *   unsigned int A[MSIZE][MSIZE] = { ... };
 *   unsigned int B[MSIZE][MSIZE] = { ... };
 */
#include "mmm_matrices.h"

/*
 * C does not need to be initialized because each output element is assigned
 * exactly once by exactly one thread.
 */
unsigned int C[MSIZE][MSIZE];

/* Performs matrix-matrix multiplication of A and B, storing the result in C. */
static void mmm(unsigned int A[MSIZE][MSIZE], unsigned int B[MSIZE][MSIZE],
                unsigned int C[MSIZE][MSIZE]) {
  int A_rows = MSIZE, A_cols = MSIZE, B_cols = MSIZE;
  int output_row, output_col, input_dim;

  unsigned int global_tid = bid() * bdim() + tid();
  unsigned int total_threads = bdim() * gdim();

  for (output_row = 0; output_row < A_rows; output_row++) {
    for (output_col = 0; output_col < B_cols; output_col++) {
      if (((output_row * B_cols + output_col) % total_threads) != global_tid) {
        continue;
      }

      unsigned int acc = 0;

      for (input_dim = 0; input_dim < A_cols; input_dim++) {
        acc += A[output_row][input_dim] * B[input_dim][output_col];
      }

      C[output_row][output_col] = acc;
    }
  }
}

int main()
{
  mmm(A, B, C);
  return 0;
}