#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

char *addSpaceForKey(char ***keywords, size_t *keywords_size) {
	*keywords_size += sizeof(char*) + 1;
	
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
	
	char buf[4294967296];
	double d = 0.0;
	
	char **keywords = malloc(sizeof(char*) + 1); 
	size_t keywords_size = sizeof(char*) + 1;
	
	size_t i = 0;
	size_t line = 0;
	
	char *c = malloc(1);
	char *tobefreed[536870912];
	
	while(fgets(buf, 4294967296, input) != NULL) {
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
		
		tobefreed[line] = c;
		
		keywords[(keywords_size / (sizeof(char*) + 1)) - 1] = c; // FIX KEYWORDS ITEMS POINT TO NEWLY ALLOCATED 'c' POINTER (maybe, not sure) (actually malloc a new c could work better)
		
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
				char *res = addSpaceForKey(&keywords, &keywords_size);
				if(res == NULL) {
					return 1;
				}
				
				keywords[(keywords_size / (sizeof(char*) + 1)) - 1] = special;
			}
			
			char *res = addSpaceForKey(&keywords, &keywords_size);
			if(res == NULL) {
				return 1;
			}
			
			keywords[(keywords_size / (sizeof(char*) + 1)) - 1] = c;
		}
		
		for(; i < keywords_size / (sizeof(char*) + 1); i++) {
			fprintf(output, "%s ", keywords[i]);
		}
		
		char *res = addSpaceForKey(&keywords, &keywords_size);
		if(res == NULL) {
			return 1;
		}
		
		d += row_len;
		printf("Compiling... %.2f%%\r", (d / file_size) * 100);
		fflush(stdout);
		
		line++;
	}
	
	fclose(input);
	
	for(size_t it = 0; it < 65536; it++) {
		free(tobefreed[it]);
	}
	
	free(keywords);
	
	fprintf(output, "}");
	
	fclose(output);
	
	puts("Compiling... 100.00%");
	
	return 0;
}