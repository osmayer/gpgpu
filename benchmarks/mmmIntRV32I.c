/**
 * mmm.c
 *
 * integer matrix-mult benchmark
 *
 *
 * NOTE: With -march=rv32i, multiply is emulated using rv32i instructions only.
 *       With -march=rv32im, the m extension needs to be implemented.
 **/

/*----------------------------------------------------------------------------
 * Internal Definitions
 *----------------------------------------------------------------------------*/

#define MSIZE 16

#define TEST 0

#define MAX_THREADS 4096

volatile unsigned int init_done = 0;
volatile unsigned int thread_done[MAX_THREADS];

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

// The matrices used for matrix multiplication
unsigned int A[MSIZE][MSIZE];
unsigned int B[MSIZE][MSIZE];
unsigned int C[MSIZE][MSIZE];

#if TEST
unsigned int I[MSIZE][MSIZE];
unsigned int Row[MSIZE][MSIZE];
unsigned int Col[MSIZE][MSIZE];
#endif

/*----------------------------------------------------------------------------
 * Functions
 *----------------------------------------------------------------------------*/

static void init() {
  int i,j ;
  for(i=0;i<MSIZE;i++) {
    for(j=0;j<MSIZE;j++) {
      A[i][j]=i*MSIZE+j;
      B[i][j]=((i+1)<<16)+(j+1);
      C[i][j]=0;
    }
  }

#if TEST
  for(i=0;i<MSIZE;i++) {
    for(j=0;j<MSIZE;j++) {
      I[i][j]=0;
      Col[i][j]=0;
      Row[i][j]=0;
      if (i==j) I[i][j]=1;
      if (i==(MSIZE/2)) Row[i][j]=1;
      if (j==(MSIZE/2)) Col[i][j]=1;
    }
  }
#endif
}

/* Performs matrix-matrix multiplication of A and B, storing the result in the
 * matrix C. */
static void mmm(unsigned int A[MSIZE][MSIZE], unsigned int B[MSIZE][MSIZE],
			unsigned int C[MSIZE][MSIZE]) {
  int A_rows=MSIZE, A_cols=MSIZE, B_cols=MSIZE;
  int output_row, output_col, input_dim;
  unsigned int global_tid = bid() * bdim() + tid();
  unsigned int total_threads = bdim() * gdim();
  
  for (output_row = 0; output_row < A_rows; output_row++) {
    for (output_col = 0; output_col < B_cols; output_col++) {
      if (((output_row * B_cols + output_col) % total_threads) != global_tid) {
        continue;
      }

      for (input_dim = 0; input_dim < A_cols; input_dim++) {
	C[output_row][output_col] += 
	  A[output_row][input_dim] * B[input_dim][output_col];
      }
    }
  }
}

// Sums all the elements in the given matrix together
unsigned int  matrix_add_reduce(int rows, int cols, unsigned int M[rows][cols]) {
  unsigned int sum = 0;
  int row, col;

  for (row = 0; row < rows; row++) {
    for (col = 0; col < cols; col++) {
      sum += M[row][col];
    }
  }
  
  return sum;
}

// Main method for the program
int main()
{
  unsigned int i;
  unsigned int global_tid = bid() * bdim() + tid();
  unsigned int total_threads = bdim() * gdim();

  if (global_tid == 0) {
    for (i = 0; i < total_threads; i++) {
      thread_done[i] = 0;
    }

    init();

    init_done = 1;
  } else {
    while (!init_done) {}
  }

  mmm(A, B, C);

  thread_done[global_tid] = 1;
  
  // having strict memory ordering is amazing...
  if (global_tid == 0) {
    for (i = 0; i < total_threads; i++) {
      while (!thread_done[i]) {}
    }

    unsigned int sum = matrix_add_reduce(MSIZE, MSIZE, C);
    return sum;
  }

  return 0;
}