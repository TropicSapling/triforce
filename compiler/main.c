#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

char *addSpaceForKeys(char ***keywords, size_t *keywords_size) {
	*keywords_size *= 2;
	
	char *res = realloc(*keywords, *keywords_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
	} else {
		*keywords = (char**) res;
	}
	
	return res;
}

int main(int argc, char *argv[]) {
	if(argc < 2 || argc > 3) {
		puts("Invalid usage. Please specify an input file as the first argument and an output file as the second argument.");
		return 1;
	}
	
	FILE *input = fopen(argv[1], "r"); // Will be "r+" if automatic compiled GC gets added in the future
	
	if(input == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		return 1;
	}
	
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
	
	fprintf(output, "#include <stdio.h>\nint main(int argc, char *argv[]) {\n");
	
	fseek(input, 0L, SEEK_END);
	size_t file_size = ftell(input);
	rewind(input);
	
	char buf[65536];
	double progress = 0.0;
	
	size_t keywords_size = (sizeof(char*) + 1) * 32;
	char **keywords = malloc(keywords_size); 
	
	size_t i = 0; // Will (most likely) be removed in the future
	size_t key = 0;
	
	char *c;
	
	while(fgets(buf, 65536, input) != NULL) {
		if(strcmp(buf, "\n") == 0 || strcmp(buf, "\r\n") == 0) {
			continue;
		}
		
		char *tmp = malloc(strlen(buf) + 1);
		if(tmp == NULL) {
			perror("ERROR");
			fprintf(stderr, "ID: %d\n", errno);
		} else {
			c = tmp;
		}
		
		strcpy(c, buf);
		
		if(key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(&keywords, &keywords_size) == NULL) {
			return 1;
		}
		
		keywords[key] = NULL; // This is used to mark where memory was allocated for 'c'
		key++;
		
		if(key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(&keywords, &keywords_size) == NULL) {
			return 1;
		}
		
		keywords[key] = c;
		key++;
		
		size_t row_len = 0;
		
		while(1) {
			char *special = calloc(2, 1);
			
			while(*c != ' ' && *c != '\0') {
				c++;
				row_len++;
				
				if(*c == ';' || *c == ',' || *c == '[' || *c == ']' || *c == '{' || *c == '}' || *c == '(' || *c == ')' || *c == '?' || *c == '>' || *c == '<' || *c == '=' || *c == '+' || *c == '-' || *c == '*' || *c == '/' || *c == '%' || *c == '!' || *c == '&' || *c == '|' || *c == '^' || *c == '~' || *c == '\\') {
					special[0] = *c;
					break;
				}
			}
			
			if(*c == '\0') {
				c++;
				break;
			}
			
			*c = '\0';
			
			c++;
			row_len++;
			
			if(special[0] != '\0') {
				if(key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(&keywords, &keywords_size) == NULL) {
					return 1;
				}
				
				keywords[key] = special;
				key++;
			}
			
			if(key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(&keywords, &keywords_size) == NULL) {
				return 1;
			}
			
			keywords[key] = c;
			key++;
		}
		
		// For debugging; will be removed (or possibly replaced) in the future
		for(; i < key; i++) {
			if(keywords[i] != NULL) {
				fprintf(output, "%s ", keywords[i]);
			}
		}
		
		progress += row_len;
		printf("Compiling... %.2f%%\r", (progress / file_size) * 100);
		fflush(stdout);
	}
	
	fclose(input);
	
	fprintf(output, "}");
	
	fclose(output);
	
	for(i = 0; i < key; i++) {
		if(keywords[i] == NULL) {
			free(keywords[i + 1]);
			i++;
		}
	}
	
	free(keywords);
	
	puts("Compiling... 100.00%");
	
	return 0;
}