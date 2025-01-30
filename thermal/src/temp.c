#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#define TCTL_PATH  "/sys/class/hwmon/hwmon5/temp1_input"
#define CPUTMP_PATH "/sys/class/hwmon/hwmon6/temp2_input"

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

void read_temp(const char * hwmon_path, char * buf, int bufsize) {
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
}

int main(int argc, char **argv) {



    for (int i = 0; i < N_MEASUREMENTS; ++i) {
        char tctl[10];
        char cpu[10];

        read_temp(TCTL_PATH, tctl, 10);
        read_temp(CPUTMP_PATH, cpu, 10);

        printf("%s\t%s\n", tctl, cpu);
        fflush(stdout);
        usleep(MILLIS * 1000); // 200 milliseconds
    }
    return 0;
}
