#include <stdio.h>
#include <errno.h>

void println(char str[]) {
	printf("%s\n", str);
}

int main(int argc, char *argv[]) {
	if(argc != 2) {
		println("Invalid usage. Please specify a file as the first and only argument.");
		return 1;
	}
	
	println("Compiling...");
	
	FILE *file = fopen(argv[1], "r"); // Will be "r+" if automatic compiled GC gets added in the future
	
	if(file == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		return 1;
	}
	
	char buf[255];
	
	while(fgets(buf, 255, file) != NULL) {
		println(buf);
	}
	
	fclose(file);
	
	return 0;
}