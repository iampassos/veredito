#include <stdio.h>
#include <stdlib.h>

int main() {
  int arr[10];

  for (int i = 0; i < 10; i++)
    scanf("%d", &arr[i]);

  for (int i = 0; i < 10; i++)
    printf("%d\n", arr[10 - i - 1]);

  // int *ptr = NULL;
  // printf("%d\n", *ptr);

  return 0;
}
