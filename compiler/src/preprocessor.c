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
	if(exports_count + (size) > exports_size && addSpaceForKeys(&exports, &exports_size) == NULL) { \
		return 1; \
	} \
} while(0)

#define INCR_EXPORTS2_MEM(size) do { \
	if(*ekey + (size) > *exports_str_size && addSpaceForFileChars(exports_str, exports_str_size) == NULL) { \
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

int preprocess(FILE **input, char **processed_input, size_t input_size, char specials[], char *path, char **exports_str, size_t *exports_str_size, size_t *ekey) {
	char buf[65536];
	size_t input_item = 0;
/*	size_t exports_count = 0;
	size_t exports_exported = 0; */
	
/*	size_t exports_size = sizeof(char*) * 32;
	char **exports = malloc(exports_size); // Array of functions to be exported
	if(exports == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
	} */
	
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
				if(trimmed_buf[c] == '<') {
					// Import standard library
					
					char full_path[256]; 
					strcpy(full_path, path); // Path to executable
					
					unsigned short i = strlen(full_path) - 1;
					for(unsigned short s = 0; s < 3; s++) {
						do {
							i--;
						} while(full_path[i] != '/' && i > 0);
						
						if(i == 0) break;
					}
					full_path[i] = '\0';
					
					strcat(full_path, "/lib/");
					char lib_path[128];
					
					c++;
					i = 0;
					for(; trimmed_buf[c + i] != '>'; i++) {
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
						free(lib_contents); // We don't need this
						
						if(ec_key > exported_content_size && addSpaceForFileChars(&exported_content, &exported_content_size) == NULL) return 1;
						exported_content[ec_key] = '\0';
						
						char *org_exported_content = exported_content;
						while(*exported_content != '\0') {
							INCR_MEM(1);
							(*processed_input)[input_item] = *exported_content;
							exported_content++;
						}
						
						free(org_exported_content);
						
						// TODO: Add support for importing specific functions
					}
					
					fclose(lib);
				} else {
					// Import custom library
				}
			} else if(strcmp(skey, "endexp") == 0) {
				exporting = false;
			} else if(exporting) {
				while(*trimmed_buf != '\0') {
					INCR_EXPORTS2_MEM(1);
					(*exports_str)[*ekey] = *trimmed_buf;
					*ekey++;
					trimmed_buf++;
				}
			} else if(exports_str && strcmp(skey, "export") == 0) {
/*				for(; trimmed_buf[c] != '\n'; exports_count++) {
					INCR_EXPORTS_MEM(1);
					exports[exports_count] = &trimmed_buf[c];
					
					while(trimmed_buf[c] != ',') {
						c++;
					}
					
					trimmed_buf[c] = '\0';
					c++;
					while(trimmed_buf[c] == ' ') c++;
				}
				
				exports_count++; */
				
				exporting = true;
			}
			
			continue;
		}
		
/*		if(exports_count > 0 && !exporting) {
			while(exports_exported < exports_count && *trimmed_buf != '\0') {
				if(!inStr2 && *trimmed_buf == '\'') {
					if(inStr) {
						inStr = false;
						break;
					} else {
						inStr = true;
					}
				} else if(!inStr && *trimmed_buf == '"') {
					if(inStr2) {
						inStr2 = false;
						break;
					} else {
						inStr2 = true;
					}
				}
				
				if(!inStr && !inStr2) {
					for(size_t exportID = 0; exportID < exports_count; exportID++) {
						if(strncmp(trimmed_buf, exports[exportID], strlen(exports[exportID])) == 0) {
							exporting = true;
							break;
						}
					}
				}
				
				if(exporting) break;
				
				trimmed_buf++;
			}
		} else if(exporting) {
			while(*trimmed_buf != '\0') {
				if(!inStr2 && *trimmed_buf == '\'') {
					if(inStr) {
						inStr = false;
						break;
					} else {
						inStr = true;
					}
				} else if(!inStr && *trimmed_buf == '"') {
					if(inStr2) {
						inStr2 = false;
						break;
					} else {
						inStr2 = true;
					}
				}
				
				if(!inStr && !inStr2) {
					INCR_EXPORTS2_MEM(1);
					(*exports_str)[ekey] = *trimmed_buf;
					ekey++;
					
					if(*trimmed_buf == specials[5]) {
						exporting = false;
						break;
					}
				}
			}
			
			exports_exported++;
		} else { */
			while(*trimmed_buf != '\0') {
				INCR_MEM(1);
				(*processed_input)[input_item] = *trimmed_buf;
				
				input_item++;
				trimmed_buf++;
			}
//		}
	}
	
	INCR_MEM(1);
	(*processed_input)[input_item] = '\0';
	
	return 0;
}