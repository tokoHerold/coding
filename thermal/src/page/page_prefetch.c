#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>
#include <errno.h>

#ifndef INSTR
#define INSTR "prefetchnta"
#endif

// How it works:
// Find present page (either by passing an argument or using a stack page)
// Otherwise use /proc/<pid>/maps for regions
// Let process run and see how much heat it produces

void run(uintptr_t addr) {
  __asm__ volatile (
        // Use 16x loop unrolling
        ".loop:\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        INSTR " (%0)\n\t"
        "jmp .loop\n\t"
        :
        : "r" (addr)
        : "memory"
    );

    // char *a = (char *) addr;
    // __builtin_prefetch(a, 0, 0); // prefetch the next data in advance
}

int main(int argc, char** argv) {
    uintptr_t page_start;

    if (argc == 1) {
        char date;
        page_start = (uintptr_t) &date;
        page_start &= ~0xFFF;
        // Make sure page is present
        *((char *) page_start) = 'a';
    } else {
        errno = 0;
        long long addr = strtoull(argv[1], NULL, 16);
        if (errno != 0) {
          perror("Invalid argument:");
          exit(-1);
        }
        page_start = (uintptr_t) addr & ~0xFFF;
    }

    printf("Adress to be tested: %#llx\n", page_start);
    fflush(stdout);
    run(page_start);
}
