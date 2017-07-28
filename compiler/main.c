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
		short file_length = strlen(argv[1]);
		char filename[file_length];
		strcpy(filename, argv[1]);
		
		char c = file_length - 1;
		while(filename[c] != '.') {
			c--;
		}
		
		memset(&filename[c + 1], 'c', 1);
		memset(&filename[c + 2], '\0', 1);
		
		while(filename[c] != '/' && c > 0) {
			c--;
		}
		
		char finalname[strlen(filename) + 4];
		
		if(c == 0) {
			strcpy(finalname, "bin/");
			strcat(finalname, filename);
		} else {
			strncpy(finalname, filename, c + 1);
			finalname[c + 1] = '\0';
			strcat(finalname, "bin/");
			strcat(finalname, &filename[c + 1]);
		}
		
		output = fopen(finalname, "w");
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