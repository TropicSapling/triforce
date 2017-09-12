#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(*pos + (size) - 1 >= output_size && addSpaceForChars(&output, &output_size) == NULL) { \
		return NULL; \
	} \
} while(0)

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

bool isReserved(char arr[][8], char *str, unsigned int size) {
	for (unsigned int i = 0; i < size; i++) {
		if(strcmp(arr[i], str) == 0) return true;
	}
	
	return false;
}

bool isNumber(char *str) {
	for (unsigned int i = 0; str[i] != '\0'; i++) {
		if(!isdigit(str[i])) return false;
	}
	
	return true;
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
				
/*				if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				} */
				INCR_MEM(1);
				
				output[*pos] = '*';
				(*pos)++;
			} else if(keywords[i][0] == '\'') {
				// STRINGS (without null termination)
				
				if(keywords[i][2] == '\0') {
/*					if(*pos + 2 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					} */
					INCR_MEM(3);
					
					output[*pos] = '\'';
					(*pos)++;
					output[*pos] = keywords[i][1];
					(*pos)++;
					output[*pos] = '\'';
					(*pos)++;
					
					continue;
				}
				
/*				if(*pos + (strlen(keywords[i]) - 2) * 4 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				} */
				INCR_MEM(1);
				
				output[*pos] = '{';
				(*pos)++;
				
				for(unsigned int c = 1; keywords[i][c] != '\0'; c++) {
					INCR_MEM(4);
					
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
				
/*				if(*pos + strlen(keywords[i]) >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				} */
				
				for(unsigned int c = 0; keywords[i][c] != '\0'; c++) {
					INCR_MEM(1);
					output[*pos] = keywords[i][c];
					(*pos)++;
				}
				
				INCR_MEM(1);
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
			} else if(strcmp(keywords[i], "__args") == 0) {
				INCR_MEM(4);
				typeTo(output, "argv", pos);
			} else if(strcmp(keywords[i], "__argc") == 0) {
				INCR_MEM(4);
				typeTo(output, "argc", pos);
			} else if(strcmp(keywords[i], "__line") == 0) {
				INCR_MEM(8);
				typeTo(output, "__LINE__", pos);
			} else if(strcmp(keywords[i], "__path") == 0) {
				INCR_MEM(8);
				typeTo(output, "__PATH__", pos);
			} else if(keywords[i][0] == specials[2]) {
				// LISTS
				
				if(strcmp(keywords[i + 1], "when") == 0) {
					// WIP
				} else {
					for(unsigned int j = 1; keywords[i + j][0] != specials[3]; j++) {
						if(keywords[i + j][0] != '>' || keywords[i + j + 1][0] != '>' || keywords[i + j + 2][0] != '>') {
/*							if(*pos + 16 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
								return NULL;
							} */
							INCR_MEM(22);
							
							typeTo(output, "for(unsigned int ", pos);
							
							char it_name[64];
							size_t it_name_len;
							if(!isReserved(types, keywords[i - 1], 22) && !isReserved(reserved_keys, keywords[i - 1], 19) && strstr(specials, keywords[i - 1]) == NULL) {
								strcpy(it_name, keywords[i - 1]);
								strcat(it_name, "_it");
								
								it_name_len = strlen(it_name);
								INCR_MEM(it_name_len);
								typeTo(output, it_name, pos);
							} else {
//								typeTo(output, it_name, pos); // TODO: Define 'it_name'
								break; // TMP
							}
							
							output[*pos] = '=';
							(*pos)++;
							
							if(j == 0) {
								INCR_MEM(1);
								output[*pos] = '0';
								(*pos)++;
							} else if(isNumber(keywords[i + 1])) {
								for(unsigned int k = 0; keywords[i + 1][k] != '\0'; k++) {
									INCR_MEM(1);
									output[*pos] = keywords[i + 1][k];
									(*pos)++;
								}
							} else {
								break; // TMP, WIP
							}
							
							output[*pos] = ';';
							(*pos)++;
							
							INCR_MEM(it_name_len);
							typeTo(output, it_name, pos);
							output[*pos] = '<';
							(*pos)++;
							
							if(keywords[i + j + 3][0] == specials[3]) {
//								typeTo(output, list_length, pos); // TODO: Define 'list_length'
								break; // TMP
							} else if(isNumber(keywords[i + 1])) {
								for(unsigned int l = j + 3; keywords[l][0] != specials[3]; l++) {
									INCR_MEM(1);
									output[*pos] = keywords[i + j + 3][l];
									(*pos)++;
								}
							} else {
								break; // TMP, WIP
							}
							
							output[*pos] = ';';
							(*pos)++;
							
							INCR_MEM(it_name_len);
							typeTo(output, it_name, pos);
							typeTo(output, "++", pos);
							
							break;
						} else if(keywords[i][0] != '<' || keywords[i + j + 1][0] != '<' || keywords[i + j + 2][0] != '<') {
							break; // TMP, WIP
						}
					}
				}
			} else if(!isNumber(keywords[i]) && !isReserved(types, keywords[i], 22) && !isReserved(reserved_keys, keywords[i], 19) && strstr(specials, keywords[i]) == NULL) {
				for(int it = 0; keywords[i][it] != '\0'; it++) {
/*					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					} */
					INCR_MEM(1);
					
					output[*pos] = keywords[i][it];
					(*pos)++;
				}
				
				typeTo(output, "_ppl", pos);
			} else {
				// DEBUG; will be replaced later
				for(int it = 0; keywords[i][it] != '\0'; it++) {
/*					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					} */
					INCR_MEM(1);
					
					output[*pos] = keywords[i][it];
					(*pos)++;
				}
				
				if(strlen(keywords[i]) > 1) {
/*					if(*pos >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
						return NULL;
					} */
					INCR_MEM(1);
					
					output[*pos] = ' ';
					(*pos)++;
				}
			}
		}
	}
	
/*	if(*pos > output_size && addSpaceForChars(&output, &output_size) == NULL) {
		return NULL;
	} */
	INCR_MEM(1);
	
	output[*pos] = '\0';
	
	return output;
}