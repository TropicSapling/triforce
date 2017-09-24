#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(*pos + (size) > output_size) addSpaceForChars(&output, &output_size); \
} while(0)

#define typeToOutput(str) do { \
	typeTo(&output, &output_size, str, pos); \
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
		exit(EXIT_FAILURE);
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

void *typeTo(char **output, size_t *output_size, char *str, size_t *pos) {
	for(size_t i = 0; str[i] != '\0'; i++) {
		if(*pos + 1 > *output_size) addSpaceForChars(output, output_size);
		(*output)[*pos] = str[i];
		(*pos)++;
	}
}

void addIteratorID(char *str_end, size_t *iterator_count) {
	char *chars = "abcdefghijklmnopqrstuvwxyz";
	
	str_end[0] = chars[(*iterator_count / (26 * 26)) % 26];
	str_end[1] = chars[(*iterator_count / 26) % 26];
	str_end[2] = chars[*iterator_count % 26];
	str_end[3] = '\0';
	
	(*iterator_count)++;
}

char *parse(char **keywords, size_t keys, size_t *pos, char specials[]) {
	char types[22][8] = {"bool", "chan", "char", "clang", "const", "fraction", "func", "heap", "int", "list", "noscope", "number", "only", "pointer", "register", "signed", "stack", "static", "unique", "unsigned", "void", "volatile"};
	char reserved_keys[19][8] = {"async", "break", "case", "continue", "default", "do", "else", "eval", "export", "foreach", "goto", "if", "import", "in", "repeat", "return", "switch", "type", "while"};
	
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	size_t iterators = 0;
	
	for(size_t i = 0; i < keys; i++) {
		if(keywords[i][0] == '@') {
			// POINTER ACCESS
			
			INCR_MEM(1);
			
			output[*pos] = '*';
			(*pos)++;
		} else if(keywords[i][0] == '\'') {
			// STRINGS (without null termination)
			
			if(keywords[i][2] == '\0') {
				INCR_MEM(3);
				
				output[*pos] = '\'';
				(*pos)++;
				output[*pos] = keywords[i][1];
				(*pos)++;
				output[*pos] = '\'';
				(*pos)++;
				
				continue;
			}
			
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
			typeToOutput("argv");
		} else if(strcmp(keywords[i], "__argc") == 0) {
			typeToOutput("argc");
		} else if(strcmp(keywords[i], "__line") == 0) {
			typeToOutput("__LINE__");
		} else if(strcmp(keywords[i], "__path") == 0) {
			typeToOutput("__PATH__");
		} else if(i + 1 < keys && keywords[i + 1][0] == '[') {
			// LISTS
			
			if(strcmp(keywords[i + 2], "when") == 0) {
				// WIP
			} else {
				for(unsigned int i_pos = 2; keywords[i + i_pos][0] != ']'; i_pos++) {
					if(keywords[i + i_pos][0] == '>' && keywords[i + i_pos + 1][0] == '>' && keywords[i + i_pos + 2][0] == '>') {
						unsigned int st_pos = 0;
						if(keywords[i][0] == ')') {
							while(keywords[i - st_pos][0] != '(') {
								st_pos++;
							}
						}
						
						if(keywords[i - st_pos - 1][0] == '=' && strstr(specials, keywords[i - st_pos - 2]) == NULL) {
							// WIP
							break;
						} else if(keywords[i - st_pos - 2][0] == '+' && keywords[i - 1][0] == '=') {
							// WIP
							break;
						} else if(keywords[i - st_pos - 1][0] == '>' || keywords[i - st_pos - 1][0] == '<' || keywords[i - st_pos - 1][0] == '=' || keywords[i - st_pos - 1][0] == '!' || keywords[i - st_pos - 1][0] == '&' || keywords[i - st_pos - 1][0] == '|') {
							// keywords[i - 1] is a comparison operator
							
							while(*pos >= 0 && output[*pos - 1] != ';' && output[*pos - 1] != '{' && output[*pos - 1] != '}') {
								(*pos)--;
							}
							
							INCR_MEM(8);
							
							// Create iterator
							typeToOutput("size_t ");
							
							char it_name[11] = "ppl_it_";
							addIteratorID(it_name + 7, &iterators);
							
							typeToOutput(it_name);
							output[*pos] = '=';
							(*pos)++;
							
							// Get sublist start pos
							if(keywords[i + i_pos - 1][0] == '[') { // Use default
								INCR_MEM(1);
								output[*pos] = '0';
								(*pos)++;
							} else {
								for(unsigned int sp_pos = 2; keywords[i + sp_pos][0] != '>'; sp_pos++) {
									typeToOutput(keywords[i + sp_pos]);
								}
							}
							
							i_pos += 3;
							
							// Create for loop
							typeToOutput(";for(;");
							
							typeToOutput(it_name);
							output[*pos] = '<';
							(*pos)++;
							
							char *max_it_val = &output[*pos];
							size_t max_it_val_len = 0;
							
							// Get sublist end pos
							if(keywords[i + i_pos][0] == ']') { // Use default
//								typeToOutput(list_length); // TODO: Define 'list_length'
								break; // TMP
							} else {
								unsigned int ep_pos = 0;
								for(; keywords[i + i_pos + ep_pos][0] != ']'; ep_pos++) {
									for(unsigned int en_pos = 0; keywords[i + i_pos + ep_pos][en_pos] != '\0'; en_pos++) {
										INCR_MEM(1);
										
										output[*pos] = keywords[i + i_pos + ep_pos][en_pos];
										(*pos)++;
										
										max_it_val_len++;
									}
								}
								
								i_pos += ep_pos;
							}
							
							i_pos++;
							
							output[*pos] = ';';
							(*pos)++;
							
							typeToOutput(it_name);
							typeToOutput("++){if(!(");
							
							st_pos++;
							while(keywords[i - st_pos][0] == '>' || keywords[i - st_pos][0] == '<' || keywords[i - st_pos][0] == '=' || keywords[i - st_pos][0] == '!' || keywords[i - st_pos][0] == '&' || keywords[i - st_pos][0] == '|') {
								st_pos++;
							}
							
							unsigned int st_pos_bef = st_pos;
							while(keywords[i - st_pos_bef][0] != '[') {
								st_pos_bef++;
							}
							st_pos_bef++;
							
							if(keywords[i - st_pos_bef][0] == ')') {
								while(keywords[i - st_pos_bef][0] != '(') {
									st_pos_bef++;
								}
							}
							
							// Type expression before comparison operator
							for(; keywords[i - st_pos_bef][0] != '['; st_pos_bef--) {
								typeToOutput(keywords[i - st_pos_bef]);
							}
							
							output[*pos] = '[';
							(*pos)++;
							typeToOutput(it_name);
							output[*pos] = ']';
							(*pos)++;
							
							// Type comparison operator
							unsigned int st_pos_c = st_pos;
							for(unsigned int st_pos2 = st_pos - 1; st_pos2 > 0; st_pos2--) {
/*								if(st_pos2 <= 3 && keywords[i - st_pos2][0] == '[') {
									INCR_MEM(2);
									output[*pos] = '[';
									(*pos)++;
									typeToOutput(it_name);
									output[*pos] = ']';
									(*pos)++;
									
									st_pos2--;
									typeToOutput(keywords[i - st_pos2]);
									st_pos2--;
									if(st_pos2 > 0) typeToOutput(keywords[i - st_pos2]);
									
									break;
								} */ // WIP
								
								typeToOutput(keywords[i - st_pos2]);
							}
							
							// Type expression after comparison operator
							typeToOutput(keywords[i]);
							output[*pos] = '[';
							(*pos)++;
							typeToOutput(it_name);
							typeToOutput("])){break;}}");
							
							while(keywords[i - st_pos][0] != ';' && keywords[i - st_pos][0] != '{' && keywords[i - st_pos][0] != '}') {
								st_pos++;
							}
							st_pos--;
							
							// Type statement before comparison
							for(; st_pos > st_pos_c; st_pos--) {
								typeToOutput(keywords[i - st_pos]);
							}
							
							// Include comparison results
							output[*pos] = '(';
							(*pos)++;
							typeToOutput(it_name);
							output[*pos] = '<';
							(*pos)++;
							for(unsigned int miv_pos = 0; miv_pos < max_it_val_len; miv_pos++) {
								INCR_MEM(1);
								
								output[*pos] = max_it_val[miv_pos];
								(*pos)++;
							}
							typeToOutput("?1:0)");
							
							// Type statement after comparison
							i += i_pos;
							for(; keywords[i][0] != ';' && keywords[i][0] != '{' && keywords[i][0] != '}'; i++) {
								typeToOutput(keywords[i]);
							}
							
							break;
						}
					} else if(keywords[i + i_pos][0] == '<' && keywords[i + i_pos + 1][0] == '<' && keywords[i + i_pos + 2][0] == '<') {
						break; // TMP, WIP
					} else {
						for(int it = 0; keywords[i][it] != '\0'; it++) {
							INCR_MEM(1);
							
							output[*pos] = keywords[i][it];
							(*pos)++;
						}
						
						typeToOutput("_ppl");
					}
				}
			}
		} else if(keywords[i][0] != '"' && keywords[i][0] != '\'' && !isNumber(keywords[i]) && !isReserved(types, keywords[i], 22) && !isReserved(reserved_keys, keywords[i], 19) && strstr(specials, keywords[i]) == NULL) {
			for(int it = 0; keywords[i][it] != '\0'; it++) {
				INCR_MEM(1);
				
				output[*pos] = keywords[i][it];
				(*pos)++;
			}
			
			typeToOutput("_ppl");
		} else {
			// DEBUG; will be replaced later
			int it = 0;
			for(; keywords[i][it] != '\0'; it++) {
				INCR_MEM(1);
				
				output[*pos] = keywords[i][it];
				(*pos)++;
			}
			
			if(it > 1) {
				INCR_MEM(1);
				
				output[*pos] = ' ';
				(*pos)++;
			}
		}
	}
	
	INCR_MEM(1);
	output[*pos] = '\0';
	
	return output;
}