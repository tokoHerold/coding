#include <bits/types/struct_timeval.h>
#include <stdio.h>
#include <stdlib.h>
// #include <sys/time.h>

#define SIZE (size_t)4 * 1024 * 1024 * 1024

int main() {

  // Allocate memory
  int *array = (int *) malloc(SIZE * sizeof(int));
  if (array == NULL) {
    fprintf(stderr, "Could not allocate memory!");
    return 1;
  }

  // Measure for 5 minutes
  // struct timeval time;
  // if (gettimeofday(&time, NULL) != 0) {
  //   perror("gettimeofday");
  //   return -1;
  // }

  // time_t start = time.tv_sec;

  do {
    for (int i = 0; i < SIZE; ++i) {
      array[i] = 0xf00ba7;
    }
    // if (gettimeofday(&time, NULL) != 0) {
    //   perror("gettimeofday");
    //   return -1;
    // }
  // } while (time.tv_sec - start < 300);
  } while (1);

  return 0;
}
