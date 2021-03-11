
#include <stdio.h>
#include "include/hello.h"
#define CGLTF_IMPLEMENTATION
#include "include/cgltf.h"

// This is the main function.
int main()
{
	// We're printing from main now.
	printf("HELLO WORLD FROM MAIN");
	{
		// Now printing from inner
		printf("Inner PRINT!");
	}
	say_hello();
	return 0;
}