#include<stdio.h>

extern char *get_string(void);

int main() {
  // gcc main.c -L target/release/ -lhello_ffi
  
  printf("Hello, C-world\n");
  char *string = get_string();

  printf("%s", string);
}
