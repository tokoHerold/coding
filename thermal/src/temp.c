#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define BUFSIZE 10
#define DELIM "\t"

#ifndef N_MEASUREMENTS
#define N_MEASUREMENTS 500
#endif

#ifndef MILLIS
#define MILLIS 50
#endif


void die(const char * msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

/**
 * Accesses and reads the current content of a HWMON file.
 * @param hwmon_path Path to the desired HWMON file
 * @param buf Buffer to store the read content in
 * @param bufsize Size of buf
 * @return A string containing the current value of the HWMON file
 */
char * read_hwmon(const char * hwmon_path, char * buf, int bufsize) {
    // Read file content
    FILE * hwmon_file = fopen(hwmon_path, "r");
    if (hwmon_file == NULL) die("fopen");
    if (fgets(buf, bufsize, hwmon_file) == NULL) die("fgets");
    if (fclose(hwmon_file) == EOF) die("fclose");

    // Remove newline character at end of line
    char* newline = strchr(buf, '\n');
    if(newline != NULL) {
        *newline = '0';
    }
    return buf;
}

int main(int argc, char **argv) {

    if (argc == 1) {
        fprintf(stderr, "Usage: %s <hwmon_file 1> [hwmon_file 2] ...", argv[0]);
        exit(-1);
    }

    char *buffers[argc - 1];
    for (int i = 0; i < argc - 1; ++i) {
        buffers[i] = calloc(BUFSIZE, sizeof(char *));
        if (buffers[i] == NULL) die("malloc");
    }

    for (int i = 0; i < N_MEASUREMENTS; ++i) {

        int i = 0;
        char * res;
        goto delim;
        for (; i < argc - 1; ++i) {
            printf(DELIM);
            delim:
            res = read_hwmon(argv[i + 1], buffers[i], BUFSIZE);
            printf("%s", res);
        }

        printf("\n");
        fflush(stdout);
        usleep(MILLIS * 1000); // 200 milliseconds
    }
    return 0;
}
