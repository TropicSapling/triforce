#include <stdio.h>
#include <errno.h>
#include <string.h>

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
		char filename[strlen(argv[1])];
		strcpy(filename, argv[1]);
		
		char c = 0;
		while(filename[c] != '.') {
			c++;
		}
		
		memset(filename, 'c', c + 1);
		memset(filename, '\0', c + 2);
		
		output = fopen(filename, "w");
	} else {
		output = fopen(argv[2], "w");
	}
	
	char buf[128];
	
	while(fscanf(input, "%s", buf) != EOF) {
		printf("%s ", buf);
	}
	
	fclose(input);
	fclose(output);
	
	return 0;
}