#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <stdio.h>
#include <errno.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(input_item + (size) > input_size) addSpaceForFileChars(processed_input, &input_size); \
} while(0)

#define INCR_EXPORTS_MEM(size) do { \
	if(*ekey + (size) > *exports_size) addSpaceForFileChars(exports, exports_size); \
} while(0)

static void addSpaceForFileChars(char **str, size_t *str_size) {
	*str_size *= 2;
	
	char *res = realloc(*str, *str_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		exit(EXIT_FAILURE);
	} else {
		*str = res;
	}
}

static unsigned int replaceIfDefined(char **exports, size_t *ekey, size_t *exports_size, char **str, char defs[128][2][128], size_t defs_len) {
	for(size_t i = 0; i < defs_len; i++) {
		unsigned short def_len = strlen(defs[i][0]);
		if(strncmp(*str, defs[i][0], def_len) == 0) {
			for(unsigned short s = 0, defr_len = strlen(defs[i][1]); s < defr_len; s++) {
				INCR_EXPORTS_MEM(1);
				(*exports)[*ekey] = defs[i][1][s];
				
				(*ekey)++;
			}
			
			*str += def_len;
			
			return 1;
		}
	}
	
	return 0;
}

void preprocess(FILE **input, char **processed_input, size_t input_size, char *path[static 2], char **exports, size_t *exports_size, size_t *ekey, char defs[128][2][128], size_t *defID) {
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
			INCR_MEM(1);
			(*processed_input)[input_item] = *trimmed_buf;
			
			input_item++;
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
		
		if(*trimmed_buf == '/' && *(trimmed_buf + 1) == '/') {
			INCR_MEM(1);
			(*processed_input)[input_item] = '\n';
			input_item++;
			
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
			
			if(strcmp(skey, "def") == 0) {
				c++;
				
				// Get what to replace
				unsigned int i = 0;
				for(; trimmed_buf[c + i] != '\'' && trimmed_buf[c + i] != '"'; i++) {
					defs[*defID][0][i] = trimmed_buf[c + i];
				}
				defs[*defID][0][i] = '\0';
				
				c += i + 6;
				
				// Get what to replace with
				unsigned int r_pos = 0;
				for(; trimmed_buf[c + r_pos] != '\'' && trimmed_buf[c + r_pos] != '"'; r_pos++) {
					defs[*defID][1][r_pos] = trimmed_buf[c + r_pos];
				}
				defs[*defID][1][r_pos] = '\0';
				
				(*defID)++;
				
				continue;
			} else if(strcmp(skey, "ifdef") == 0) {
				// WIP
				
				continue;
			} else if(strcmp(skey, "import") == 0) {
				char full_path[256];
				unsigned short i;
				
				if(trimmed_buf[c] == '<') {
					// Import standard library
					
					strcpy(full_path, path[0]); // Path to executable
					
					i = strlen(full_path) - 1;
					for(unsigned short s = 0; s < 4; s++) {
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
					
					char lib_path[128];
					c++;
					i = 0;
					for(; trimmed_buf[c + i] != '>' && trimmed_buf[c + i] != '\'' && trimmed_buf[c + i] != '"'; i++) {
						lib_path[i] = trimmed_buf[c + i];
					}
					
					lib_path[i] = '\0';
					
					char *new_lib_path = "";
					unsigned short levels = 1;
					i = strlen(lib_path) - 1;
					do {
						i--;
						if(i > 0 && lib_path[i] == '.' && lib_path[i - 1] == '.') {
							if(levels == 1) new_lib_path = &lib_path[i + 1];
							levels++;
							i--;
						} else if(levels > 1 && lib_path[i] != '/') {
							break;
						}
					} while(i > 0);
					
					i = strlen(full_path) - 1;
					for(unsigned short s = 0; s < levels; s++) {
						do {
							i--;
						} while(full_path[i] != '/' && i > 0);
						
						if(i == 0) break;
					}
					
					if(levels == 1 && lib_path[0] != '/') i++;
					
					full_path[i] = '\0';
					
					if(new_lib_path[0] == '\0') {
						strcat(full_path, lib_path);
					} else {
						strcat(full_path, new_lib_path);
					}
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
					exit(EXIT_FAILURE);
				}
				
				if(trimmed_buf[c + i + 2] == 'a') {
					// WIP
				} else {
					size_t exported_content_size = 256;
					char *exported_content = malloc(exported_content_size);
					char *lib_contents = malloc(exported_content_size);
					
					size_t ec_key = 0;
					
					preprocess(&lib, &lib_contents, exported_content_size, path, &exported_content, &exported_content_size, &ec_key, defs, defID);
					free(lib_contents);
					
					if(ec_key > exported_content_size) addSpaceForFileChars(&exported_content, &exported_content_size);
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
				
				continue;
			} else if(strcmp(skey, "endexp") == 0) {
				exporting = false;
				
				continue;
			} else if(exports_size && strcmp(skey, "export") == 0) {
				exporting = true;
				
				continue;
			}
		}
		
		if(exporting) {
			while(*trimmed_buf != '\0') {
				if(!replaceIfDefined(exports, ekey, exports_size, &trimmed_buf, defs, *defID)) {
					INCR_EXPORTS_MEM(1);
					(*exports)[*ekey] = *trimmed_buf;
					(*ekey)++;
					trimmed_buf++;
				}
			}
		} else {
			while(*trimmed_buf != '\0') {
				if(!replaceIfDefined(processed_input, &input_item, &input_size, &trimmed_buf, defs, *defID)) {
					INCR_MEM(1);
					(*processed_input)[input_item] = *trimmed_buf;
					
					input_item++;
					trimmed_buf++;
				}
			}
		}
	}
	
	INCR_MEM(1);
	(*processed_input)[input_item] = '\0';
}