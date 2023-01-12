#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
//#include "my_header.h"

struct c_string_vec {
  const char **ptr;
  uint64_t len;
  uint64_t cap;
};

extern char *read_name();

extern void read_name_to_buf();
extern uint64_t read_names_to_bufs();
extern struct c_string_vec allocate_names();
extern void free_names(struct c_string_vec);

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
    // allocate and free in a loop so we can see that we don't leak memory (and
    // if you comment out free_names(names) you can see that we do leak memory)
    while (1) {
        struct c_string_vec names = allocate_names();
        for (int i = 0; i < q.len; i++) {
            printf("%s\n", q.ptr[i]);
        }
        free_names(names);
    }
    //
    return 0;
}
