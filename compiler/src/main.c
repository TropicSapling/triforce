#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

#include "def.h"

int main(int argc, char *argv[]) {
	
	//////////////// PREPARE FOR LEXING ////////////////
	
	if(argc < 2 || argc > 3) {
		puts("Invalid usage. Please specify an input file as the first argument and an output file as the second argument.");
		return 1;
	}
	
	FILE *input = fopen(argv[1], "r"); // Will be "r+" if automatic compiled GC gets added in the future and/or if needed to fix 'read-dir' bug
	
	if(input == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		return 1;
	}
	
	// Get file size to be able to print lexing progress
	fseek(input, 0L, SEEK_END);
	size_t file_size = ftell(input);
	rewind(input);
	
	/////////////////// START LEXING ///////////////////
	
	size_t keywords_size = (sizeof(char*) + 1) * 32;
	char **keywords = malloc(keywords_size); 
	size_t key = 0;
	
	if(lex_parse(input, &keywords, keywords_size, &key, file_size)) {
		return 1;
	}
	
	fclose(input);
	puts("Reading file... 100.00%");
	
	/////////////////// START PARSING //////////////////
	
	char *parsed_output = parse(keywords, key);
	if(parsed_output == NULL) {
		return 1;
	}
	
	puts("Parsing... 100.00%");
	
	//////////////// PREPARE FOR OUTPUT ////////////////
	
	FILE *output;
	
	if(argc < 3) {
		size_t file_length = strlen(argv[1]);
		char filename[file_length];
		strcpy(filename, argv[1]);
		
		size_t c = file_length - 1;
		while(filename[c] != '.') {
			c--;
		}
		
		memset(&filename[c + 1], 'c', 1);
		memset(&filename[c + 2], '\0', 1);
		
		while(filename[c] != '/' && c > 0) {
			c--;
		}
		
		char finalname[strlen(filename) + 4];
		char success;
		
		if(c == 0) {
			success = mkdir("bin", 0777);
			
			strcpy(finalname, "bin/");
			strcat(finalname, filename);
		} else {
			strncpy(finalname, filename, c + 1);
			finalname[c + 1] = '\0';
			strcat(finalname, "bin/");
			success = mkdir(finalname, 0777);
			strcat(finalname, &filename[c + 1]);
		}
		
		if(success != 0 && errno != 17) {
			perror("ERROR");
			fprintf(stderr, "ID: %d\n", errno);
			return 1;
		}
		
		output = fopen(finalname, "w");
	} else {
		output = fopen(argv[2], "w");
	}
	
	/////////////////// PRINT OUTPUT ///////////////////
	
	fprintf(output, "#include <stdio.h>\nint main(int argc, char *argv[]) {\n");
	
	// For debugging; will be replaced in the future
	for(size_t i = 0; i < key; i++) {
		if(keywords[i] != NULL) {
			fprintf(output, "%s ", keywords[i]);
		}
		
		printf("Printing output... %.2Lf%%\r", (((long double) i + 1) / key) * 100);
	}
	
	fprintf(output, "}");
	
	fclose(output);
	puts("Printing output... 100.00%");
	
	/////////////////// FREE MEMORY ////////////////////
	
	free(parsed_output);
	
	for(size_t i = 0; i < key; i++) {
		if(keywords[i] == NULL) {
			free(keywords[i + 1]);
			i++;
		}
	}
	
	free(keywords);
	
	return 0;
}