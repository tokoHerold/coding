#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

#define SIZE (size_t)4 * 1024 * 1024 * 1024

int main() {

  // Allocate memory
  int *array = (int *)malloc(SIZE * sizeof(int));
  if (array == NULL) {
    fprintf(stderr, "Could not allocate memory!");
    return 1;
  }

  // Measure for five minutes
  struct timeval time;
  if (gettimeofday(&time, NULL) != 0) {
    perror("gettimeofday");
    return -1;
  }

  time_t start = time.tv_sec;
  const size_t STEP = 256;
  const size_t INNER = SIZE / STEP;
  do {
    // Iterate in a bad order
    for (int i = 0; i < STEP; ++i) {
      for (int j = 0; j < SIZE; j += STEP) {
        array[i + j] = 0xf00ba7;
      }
    }
    if (gettimeofday(&time, NULL) != 0) {
      perror("gettimeofday");
      return -1;
    }
  } while (time.tv_sec - start < 300);

  return 0;
}
