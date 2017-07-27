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
	
	FILE *input = fopen(argv[1], "r"); // Will be "r+" if automatic compiled GC gets added in the future
	
	if(input == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		return 1;
	}
	
	FILE *output;
	
	if(argc < 3) {
		// WIP
	} else {
		output = fopen(argv[2], "w");
	}
	
	char buf[128];
	
	while(fscanf(file, "%s", buf) != EOF) {
		printf("%s ", buf);
	}
	
	fclose(input);
	fclose(output);
	
	return 0;
}