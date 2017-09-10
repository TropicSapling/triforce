#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>

#include "def.h"

#define RED   "\x1B[31m"
#define GREEN   "\x1B[32m"
#define YELLOW   "\x1B[33m"
#define BLUE   "\x1B[34m"
#define RESET "\x1B[0m"

char *addSpaceForChars(char **keywords, size_t *keywords_size) {
	*keywords_size *= 2;
	
	char *res = realloc(*keywords, *keywords_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
	} else {
		*keywords = res;
	}
	
	return res;
}

bool arrContains(char arr[][8], char *str, unsigned int size){
	for (unsigned int i = 0; i < size; i++) {
		if(strcmp(arr[i], str) == 0) return true;
	}
	
	return false;
}

void typeTo(char *output, char *str, size_t *pos) {
	for(unsigned int i = 0; str[i] != '\0'; i++) {
		output[*pos] = str[i];
		(*pos)++;
	}
}

char *parse(char **keywords, size_t key, size_t *pos, char specials[]) {
	char types[22][8] = {"bool", "chan", "char", "clang", "const", "fraction", "func", "heap", "int", "list", "noscope", "number", "only", "pointer", "register", "signed", "stack", "static", "unique", "unsigned", "void", "volatile"};
	char reserved_keys[19][8] = {"async", "break", "case", "continue", "default", "do", "else", "eval", "export", "foreach", "goto", "if", "import", "in", "repeat", "return", "switch", "type", "while"};
	
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	for(size_t i = 0; i < key; i++) {
		if(keywords[i] != NULL) {
			if(keywords[i][0] == specials[22]) {
				// POINTER ACCESS
				
				if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				}
				
				output[*pos] = '*';
				(*pos)++;
			} else if(keywords[i][0] == '\'') {
				// STRINGS (without null termination)
				
				if(keywords[i][2] == '\0') {
					if(*pos + 2 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					}
					
					output[*pos] = '\'';
					(*pos)++;
					output[*pos] = keywords[i][1];
					(*pos)++;
					output[*pos] = '\'';
					(*pos)++;
					
					continue;
				}
				
				if(*pos + (strlen(keywords[i]) - 2) * 4 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				}
				
				output[*pos] = '{';
				(*pos)++;
				
				for(unsigned int c = 1; keywords[i][c] != '\0'; c++) {
					output[*pos] = '\'';
					(*pos)++;
					output[*pos] = keywords[i][c];
					(*pos)++;
					output[*pos] = '\'';
					(*pos)++;
					if(keywords[i][c + 1] != '\0') {
						output[*pos] = ',';
						(*pos)++;
					}
				}
				
				output[*pos] = '}';
				(*pos)++;
			} else if(keywords[i][0] == '"') {
				// STRINGS (with null termination)
				
				if(*pos + strlen(keywords[i]) >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				}
				
				for(unsigned int c = 0; keywords[i][c] != '\0'; c++) {
					output[*pos] = keywords[i][c];
					(*pos)++;
				}
				
				output[*pos] = '"';
				(*pos)++;
			} else if(strcmp(keywords[i], "clang") == 0) {
				// INLINE C
				
				for(unsigned int j = 1; j < 9; j++) {
					unsigned int k = 0;
					for(; k < 22; k++) {
						if(strcmp(keywords[i + j], types[k]) == 0) {
							break;
						}
					}
					
					if(k == 22) {
						i = i + j + 2;
						break;
					}
				}
				
				puts("----------------------------------------------------------------");
				printf(YELLOW "[WARNING]" RESET " 'clang' is not implemented yet.\n"); // WIP
				puts("----------------------------------------------------------------");
			} else if(keywords[i][0] == specials[2]) {
				// LISTS
				
				if(strcmp(keywords[i + 1], "when") == 0) {
					// WIP
				} else {
/*					for(unsigned int j = 1; keywords[i + j] != specials[3]; j++) {
						if(keywords[i + j] != '>' || keywords[i + j + 1] != '>' || keywords[i + j + 2] != '>') {
							typeTo(output, "for(unsigned int ", pos);
							typeTo(output, ITERATOR_NAME, pos); // REPLACE ITERATOR_NAME
							output[*pos] = '=';
							(*pos)++;
							
							unsigned int k = 1;
							for(; k < j; k++) {
								output[*pos] = keywords[i + k];
								(*pos)++;
							}
							
							if(k == 1) {
								output[*pos] = '0';
								(*pos)++;
							}
							
							output[*pos] = ';';
							(*pos)++;
							
							typeTo(output, ITERATOR_NAME, pos);
							output[*pos] = '<';
							(*pos)++;
							
							unsigned int l = k + 3;
							for(; keywords[l] != specials[3]; l++) {
								output[*pos] = keywords[i + l];
								(*pos)++;
							}
							
							if(l == 1) {
								typeTo(output, NAME_SIZE, pos); // REPLACE NAME_SIZE
							}
							
							output[*pos] = ';';
							(*pos)++;
							
							typeTo(output, ITERATOR_NAME, pos);
							typeTo(output, "++", pos);
							
							break;
						} else if(keywords[i] != '<' || keywords[i + j + 1] != '<' || keywords[i + j + 2] != '<') {
							break;
						}
					} */
				}
			} else if(!arrContains(types, keywords[i], 22) && !arrContains(reserved_keys, keywords[i], 19) && strstr(specials, keywords[i]) == NULL) {
				for(int it = 0; keywords[i][it] != '\0'; it++) {
					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					}
					
					output[*pos] = keywords[i][it];
					(*pos)++;
				}
				
				typeTo(output, "_ppl", pos);
			} else {
				// DEBUG; will be replaced later
				for(int it = 0; keywords[i][it] != '\0'; it++) {
					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					}
					
					output[*pos] = keywords[i][it];
					(*pos)++;
				}
				
				if(strlen(keywords[i]) > 1) {
					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					}
					
					output[*pos] = ' ';
					(*pos)++;
				}
			}
		}
	}
	
	if(*pos > output_size && addSpaceForChars(&output, &output_size) == NULL) {
		return NULL;
	}
	
	output[*pos] = '\0';
	
	return output;
}