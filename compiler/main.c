#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

void println(char str[]) {
	printf("%s\n", str);
}

int main(int argc, char *argv[]) {
	if(argc < 2 || argc > 3) {
		println("Invalid usage. Please specify an input file as the first argument and an output file as the second argument.");
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
	double d = 0.0;
	
	while(fgets(buf, 65536, input) != NULL) {
		char *c = buf;
		char *keywords[] = {c};
		size_t row_len = 0;
		size_t keywords_size = 1;
		
		while(*c != '\0') {
			while(*c != ' ' && *c != '\0') {
				c++;
				row_len++;
			}
			
			*c = '\0'; // use memset if this doesn't work
			
			c++;
			row_len++;
			keywords_size++;
			
			char *res = realloc(keywords, keywords_size);
			if(res == NULL) {
				perror("ERROR");
				fprintf(stderr, "ID: %d\n", errno);
				return 1;
			}
			
			keywords[keywords_size - 1] = c;
		}
		
/*		for(int i = 0; i < strlen(keywords); i++) {
			fprintf(output, "%s", *keywords[i]);
		} */
		
		d += row_len;
		printf("Compiling... %.2f%%\r", (d / file_size) * 100);
		fflush(stdout);
	}
	
	fclose(input);
	
	fprintf(output, "}");
	
	fclose(output);
	
	println("Compiling... 100.00%");
	
	return 0;
}