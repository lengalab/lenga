#include <stdio.h>

int
first(int a, int b, int c)
{
    int first_parameter = a;
    return first_parameter;
}

int
first(int a, int b, int c);

int
main()
{
    int a = 1;
    int b = 2;
    int c = 3;
    {
        int a = b + c;
        printf("inner a: %d\n", a);
    }
    if (a < b) {
        int result = first(a, b, c);
        printf("result: %d", result);
    } else {
        int result = first(b, c, a);
        printf("result: %d", result);
    }

}

