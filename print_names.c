#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
//#include "my_header.h"

extern char *read_name();

extern void read_name_to_buf();
extern uint64_t read_names_to_bufs();
extern uint64_t allocate_names();

int main() {
    //
    char * s = read_name();
    printf("%s\n", s);
    //
    char buf[200];
    read_name_to_buf(buf);
    printf("%s\n", buf);
    //
    char * bufs[100];
    for (int i = 0 ; i < 100; i++) {
        bufs[i] = malloc(100*sizeof(char));
    }
    uint64_t num = read_names_to_bufs(bufs);
    printf("--\n");
    for (int i = 0; i < num; i++) {
        printf("%s\n", bufs[i]);
    }
    //
    
    return 0;
}
