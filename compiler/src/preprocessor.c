#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdio.h>
#include <errno.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(input_item + (size) > input_size && addSpaceForFileChars(processed_input, &input_size) == NULL) { \
		return 1; \
	} \
} while(0)

#define INCR_EXPORTS_MEM(size) do { \
	if(*ekey + (size) > *exports_size && addSpaceForFileChars(exports, exports_size) == NULL) { \
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

int preprocess(FILE **input, char **processed_input, size_t input_size, char specials[], char *path[], char **exports, size_t *exports_size, size_t *ekey) {
	char buf[65536];
	size_t input_item = 0;
	
	bool ignoring = false;
	bool inStr = false;
	bool inStr2 = false;
	bool exporting = false;
	
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
			
			while(trimmed_buf[c] != ' ' && trimmed_buf[c] != '\r' && trimmed_buf[c] != '\n' && trimmed_buf[c] != '\0') {
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
				char full_path[256];
				unsigned short i;
				
				if(trimmed_buf[c] == '<') {
					// Import standard library
					
					strcpy(full_path, path[0]); // Path to executable
					
					i = strlen(full_path) - 1;
					for(unsigned short s = 0; s < 3; s++) {
						do {
							i--;
						} while(full_path[i] != '/' && i > 0);
						
						if(i == 0) break;
					}
					full_path[i] = '\0';
					
					strcat(full_path, "/lib/");
				} else {
					// Import custom library
					
					strcpy(full_path, path[1]); // Path to P+ file
				}
				
				char lib_path[128];
				c++;
				i = 0;
				for(; trimmed_buf[c + i] != '>' && trimmed_buf[c + i] != '\'' && trimmed_buf[c + i] != '"'; i++) {
					lib_path[i] = trimmed_buf[c + i];
				}
				
				lib_path[i] = '\0';
				strcat(full_path, lib_path);
					
				FILE *lib = fopen(full_path, "r");
				if(lib == NULL) {
					perror("ERROR");
					fprintf(stderr, "ID: %d\n", errno);
					return 1;
				}
				
				if(trimmed_buf[c + i + 2] == 'a') {
					// WIP
				} else {
					size_t exported_content_size = 256;
					char *exported_content = malloc(exported_content_size);
					char *lib_contents = malloc(exported_content_size);
					
					size_t ec_key = 0;
					
					if(preprocess(&lib, &lib_contents, exported_content_size, specials, path, &exported_content, &exported_content_size, &ec_key)) return 1;
					free(lib_contents);
					
					if(ec_key > exported_content_size && addSpaceForFileChars(&exported_content, &exported_content_size) == NULL) return 1;
					exported_content[ec_key] = '\0';
					
					char *org_exported_content = exported_content;
					while(*exported_content != '\0') {
						INCR_MEM(1);
						(*processed_input)[input_item] = *exported_content;
						exported_content++;
						input_item++;
					}
					
					free(org_exported_content);
					
					// TODO: Add support for importing specific functions
				}
				
				fclose(lib);
			} else if(strcmp(skey, "endexp") == 0) {
				exporting = false;
			} else if(exports_size && strcmp(skey, "export") == 0) {
				exporting = true;
			}
			
			continue;
		}
		
		if(exporting) {
			while(*trimmed_buf != '\0') {
				INCR_EXPORTS_MEM(1);
				(*exports)[*ekey] = *trimmed_buf;
				(*ekey)++;
				trimmed_buf++;
			}
		} else {
			while(*trimmed_buf != '\0') {
				INCR_MEM(1);
				(*processed_input)[input_item] = *trimmed_buf;
				
				input_item++;
				trimmed_buf++;
			}
		}
	}
	
	INCR_MEM(1);
	(*processed_input)[input_item] = '\0';
	
	return 0;
}