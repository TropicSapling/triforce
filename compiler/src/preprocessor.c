#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdio.h>
#include <errno.h>

#define INCR_MEM(size) do { \
	if(input_item + (size) > input_size && addSpaceForFileChars(processed_input, &input_size) == NULL) { \
		return 1; \
	} \
} while(0)

char *addSpaceForFileChars(char **str, size_t *str_size) {
	*str_size *= 2;
	
	char *res = realloc(*str, *str_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
	} else {
		*str = (char*) res;
	}
	
	return res;
}

int preprocess(FILE **input, char **processed_input, size_t input_size, char specials[]) {
	char buf[65536];
	size_t input_item = 0;
	
	bool ignoring = false;
	
	while(fgets(buf, 65536, *input) != NULL) {
		if(strlen(buf) == 65535) break; // File is compressed; skip preprocessing
		
		char *trimmed_buf = &buf[0];
		while(*trimmed_buf == '\t' || *trimmed_buf == ' ') {
			trimmed_buf++;
		}
		
		if(ignoring || (*trimmed_buf == '/' && *(trimmed_buf + 1) == '*')) {
			ignoring = true;
			
			while(*trimmed_buf != '\0' && !(*trimmed_buf == '*' && *(trimmed_buf + 1) == '/')) trimmed_buf++;
			
			if(*trimmed_buf != '\0') {
				ignoring = false;
				trimmed_buf += 2;
			} else {
				continue;
			}
		}
		
		if(strcmp(trimmed_buf, "\n") == 0 || strcmp(trimmed_buf, "\r\n") == 0 || (*trimmed_buf == '/' && *(trimmed_buf + 1) == '/')) {
			continue;
		}
		
		if(trimmed_buf[0] == '#') {
			size_t c = 1;
			char skey[8];
			
			while(trimmed_buf[c] != ' ' && trimmed_buf[c] != '\0') {
				skey[c - 1] = trimmed_buf[c];
				c++;
			}
			
			skey[c - 1] = '\0';
			
			c++;
			
			if(strcmp(skey, "redef") == 0) {
				for(unsigned short s = 0; specials[s] != '\0'; s++) {
					if(trimmed_buf[c] == specials[s]) {
						specials[s] = trimmed_buf[c + 5];
						break;
					}
				}
			} else if(strcmp(skey, "def") == 0) {
				// WIP
			} else if(strcmp(skey, "ifdef") == 0) {
				// WIP
			} else if(strcmp(skey, "import") == 0) {
				// WIP
			} else if(strcmp(skey, "export") == 0) {
				// WIP
			}
			
			continue;
		}
		
		while(*trimmed_buf != '\0') {
			INCR_MEM(1);
			(*processed_input)[input_item] = *trimmed_buf;
			
			input_item++;
			trimmed_buf++;
		}
	}
	
	INCR_MEM(1);
	(*processed_input)[input_item] = '\0';
	
	return 0;
}