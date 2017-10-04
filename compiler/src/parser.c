#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(*pos + (size) > *output_size) addSpaceForChars(outputp, output_size); \
} while(0)

#define typeToOutput(str) do { \
	typeTo(outputp, output_size, str, pos); \
} while(0)

#define RED   "\x1B[31m"
#define GREEN   "\x1B[32m"
#define YELLOW   "\x1B[33m"
#define BLUE   "\x1B[34m"
#define RESET "\x1B[0m"

char types[22][8] = {"bool", "chan", "char", "clang", "const", "fraction", "func", "heap", "int", "list", "noscope", "number", "only", "pointer", "register", "signed", "stack", "static", "unique", "unsigned", "void", "volatile"};
char reserved_keys[19][8] = {"async", "break", "case", "continue", "default", "do", "else", "eval", "export", "foreach", "goto", "if", "import", "in", "repeat", "return", "switch", "type", "while"};
size_t iterators = 0;
size_t bools = 0;

void *addSpaceForChars(char **keywords, size_t *keywords_size) {
	*keywords_size *= 2;
	
	char *res = realloc(*keywords, *keywords_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		exit(EXIT_FAILURE);
	} else {
		*keywords = res;
	}
}

bool isReserved(char arr[][8], char *str, unsigned int len) {
	for (unsigned int i = 0; i < len; i++) {
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

void addID(char *str_end, size_t *IDs) {
	char *chars = "abcdefghijklmnopqrstuvwxyz";
	
	str_end[0] = chars[(*IDs / (26 * 26)) % 26];
	str_end[1] = chars[(*IDs / 26) % 26];
	str_end[2] = chars[*IDs % 26];
	str_end[3] = '\0';
	
	(*IDs)++;
}

size_t parseKey(char **keywords, unsigned int i, size_t keys, char **outputp, size_t *output_size, size_t *pos, char specials[], unsigned short status, char *cItem) {
	if(strcmp(keywords[i], "false") == 0) {
		INCR_MEM(1);
		
		(*outputp)[*pos] = '0';
		(*pos)++;
	} else if(strcmp(keywords[i], "true") == 0) {
		INCR_MEM(1);
		
		(*outputp)[*pos] = '1';
		(*pos)++;
	} else if(keywords[i][0] == '-' && keywords[i + 1][0] == '>') {
		// POINTER CREATION
		
		if(!(keywords[i - 1][0] == '=' && strstr(specials, keywords[i - 2]) == NULL)) { // Assignment
			INCR_MEM(1);
			
			(*outputp)[*pos] = '=';
			(*pos)++;
		}
		
		if(keywords[i + 2][0] != '{' && keywords[i + 2][0] != '\'') {
			INCR_MEM(1);
			
			(*outputp)[*pos] = '&';
			(*pos)++;
		}
		
		i++;
	} else if(keywords[i][0] == '@') {
		// POINTER ACCESS
		
		INCR_MEM(1);
		
		(*outputp)[*pos] = '*';
		(*pos)++;
	} else if(keywords[i][0] == '\'') {
		// STRINGS (without null termination)
		
		if(keywords[i][2] == '\0' || (keywords[i][1] == '\\' && keywords[i][2] == '0' && keywords[i][3] == '\0')) {
			INCR_MEM(3);
			
			(*outputp)[*pos] = '\'';
			(*pos)++;
			
			(*outputp)[*pos] = keywords[i][1];
			(*pos)++;
			if(keywords[i][2] != '\0') {
				INCR_MEM(1);
				
				(*outputp)[*pos] = keywords[i][2];
				(*pos)++;
			}
			
			(*outputp)[*pos] = '\'';
			(*pos)++;
		} else {
			INCR_MEM(1);
			
			(*outputp)[*pos] = '{';
			(*pos)++;
			
			for(unsigned int c = 1; keywords[i][c] != '\0'; c++) {
				INCR_MEM(4);
				
				(*outputp)[*pos] = '\'';
				(*pos)++;
				
				(*outputp)[*pos] = keywords[i][c];
				(*pos)++;
				if(keywords[i][c] == '\\' && keywords[i][c + 1] == '0') {
					INCR_MEM(1);
					
					(*outputp)[*pos] = keywords[i][c + 1];
					(*pos)++;
					c++;
				}
				
				(*outputp)[*pos] = '\'';
				(*pos)++;
				if(keywords[i][c + 1] != '\0') {
					(*outputp)[*pos] = ',';
					(*pos)++;
				}
			}
			
			(*outputp)[*pos] = '}';
			(*pos)++;
		}
	} else if(keywords[i][0] == '"') {
		// STRINGS (with null termination)
		
		for(unsigned int c = 0; keywords[i][c] != '\0'; c++) {
			INCR_MEM(1);
			(*outputp)[*pos] = keywords[i][c];
			(*pos)++;
		}
		
		INCR_MEM(1);
		(*outputp)[*pos] = '"';
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
	} else if(strcmp(keywords[i], "__item") == 0) {
		if(cItem == NULL) {
			puts("----------------------------------------------------------------");
			printf(RED "[ERROR]" RESET " Invalid placement of '__item'.\n");
			puts("----------------------------------------------------------------");
			exit(EXIT_FAILURE);
		} else {
			typeToOutput(cItem);
		}
	} else if(i + 1 < keys && keywords[i + 1][0] == '[' && status != 1) {
		// LISTS
		
		bool foundSublist = false;
		
		unsigned int i_pos = 2;
		for(; keywords[i + i_pos][0] != ']'; i_pos++) {
			if(keywords[i + i_pos][0] == '>' && keywords[i + i_pos + 1][0] == '>' && keywords[i + i_pos + 2][0] == '>') {
				unsigned short parentheses = 0;
				unsigned short brackets = 0;
				unsigned short brackets2 = 0;
				
				// Get list start pos
				unsigned int st_pos = 0;
				if(keywords[i][0] == ')') {
					while(keywords[i - st_pos][0] != '(' || parentheses > 0) {
						st_pos++;
						
						if(keywords[i - st_pos][0] == ')') parentheses++;
						if(parentheses && keywords[i - st_pos][0] == '(') parentheses--;
					}
				} else if(keywords[i][0] == ']') {
					while(keywords[i - st_pos][0] != '[' || brackets > 0 || (keywords[i - st_pos - 1][0] == ']' && st_pos++) || (keywords[i - st_pos - 1][0] == ')' && st_pos++)) {
						st_pos++;
						
						if(keywords[i - st_pos][0] == ')') parentheses++;
						if(parentheses && keywords[i - st_pos][0] == '(') parentheses--;
						
						if(keywords[i - st_pos][0] == ']') {
							brackets++;
							brackets2++;
						}
						if(brackets && keywords[i - st_pos][0] == '[') brackets--;
					}
				}
				
				unsigned int st_pos_bef = st_pos;
				
				if(keywords[i - st_pos - 1][0] == '=' && strstr(specials, keywords[i - st_pos - 2]) == NULL) {
					// WIP
					break;
				} else if(keywords[i - st_pos - 2][0] == '+' && keywords[i - 1][0] == '=') {
					// WIP
					break;
				} else if(keywords[i - st_pos - 1][0] == '>' || keywords[i - st_pos - 1][0] == '<' || keywords[i - st_pos - 1][0] == '=' || keywords[i - st_pos - 1][0] == '!' || keywords[i - st_pos - 1][0] == '&' || keywords[i - st_pos - 1][0] == '|') {
					// keywords[i - st_pos - 1] is a comparison operator
					
					foundSublist = true;
					
					while(*pos >= 0 && (*outputp)[*pos - 1] != ';' && (*outputp)[*pos - 1] != '{' && (*outputp)[*pos - 1] != '}') {
						(*pos)--;
					}
					
					INCR_MEM(7);
					
					// Create iterator
					typeToOutput("size_t ");
					
					char it_name[11] = "ppl_it_";
					addID(it_name + 7, &iterators);
					
					typeToOutput(it_name);
					(*outputp)[*pos] = '=';
					(*pos)++;
					
					// Get sublist start pos
					if(keywords[i + i_pos - 1][0] == '[') {
						typeToOutput("0;"); // Use default start pos
					} else if(strcmp(keywords[i + 2], "when") == 0) {
						typeToOutput("0;while(!(");
						
						// Get sublist start pos condition
						for(unsigned int sp_pos = 3; keywords[i + sp_pos][0] != '>'; sp_pos++) {
							parseKey(keywords, i + sp_pos, keys, outputp, output_size, pos, specials, 0, it_name);
						}
						
						typeToOutput(")){");
						typeToOutput(it_name);
						
						// Create while loop
						typeToOutput("++;}");
					} else {
						for(unsigned int sp_pos = 2; keywords[i + sp_pos][0] != '>'; sp_pos++) {
							parseKey(keywords, i + sp_pos, keys, outputp, output_size, pos, specials, 0, it_name);
						}
						
						INCR_MEM(1);
						(*outputp)[*pos] = ';';
						(*pos)++;
					}
					
					i_pos += 3;
					
					// Create condition bool
					char cond_bool[17] = "ppl_condBool_";
					addID(cond_bool + 13, &bools);
					
					typeToOutput("int ");
					typeToOutput(cond_bool);
					typeToOutput("=0;");
					
					// Get sublist end pos
					if(keywords[i + i_pos][0] == ']') { // Use default
//						typeToOutput(list_length); // TODO: Define 'list_length'
						break; // TMP
					} else {
						// Create for loop
						typeToOutput("for(;");
						
						if(strcmp(keywords[i + i_pos], "until") == 0) {
							INCR_MEM(1);
							
							(*outputp)[*pos] = '!';
							(*pos)++;
							
							i_pos++;
						} else {
							typeToOutput(it_name);
							
							INCR_MEM(1);
							(*outputp)[*pos] = '<';
							(*pos)++;
						}
						
						(*outputp)[*pos] = '(';
						(*pos)++;
						
						unsigned int ep_pos = 0;
						for(; keywords[i + i_pos + ep_pos][0] != ']' || brackets > 0; ep_pos++) {
							parseKey(keywords, i + i_pos + ep_pos, keys, outputp, output_size, pos, specials, 0, it_name);
							
							if(keywords[i + i_pos + ep_pos][0] == '[') brackets++;
							if(brackets && keywords[i + i_pos + ep_pos][0] == ']') brackets--;
						}
						
						(*outputp)[*pos] = ')';
						(*pos)++;
						
						i_pos += ep_pos;
					}
					
					i_pos++;
					
					(*outputp)[*pos] = ';';
					(*pos)++;
					
					typeToOutput(it_name);
					typeToOutput("++){if(!(");
					
					// Get start pos of comparison operator
					st_pos++;
					while(keywords[i - st_pos][0] == '>' || keywords[i - st_pos][0] == '<' || keywords[i - st_pos][0] == '=' || keywords[i - st_pos][0] == '!' || keywords[i - st_pos][0] == '&' || keywords[i - st_pos][0] == '|') {
						st_pos++;
					}
					
					unsigned int st_pos_bef_c = st_pos_bef;
					
					// Type expression before comparison operator
					for(; keywords[i - st_pos_bef][0] != '[' || brackets > 0; st_pos_bef--) {
						parseKey(keywords, i - st_pos_bef, keys, outputp, output_size, pos, specials, 0, cItem);
						
						if(keywords[i - st_pos_bef][0] == '[') brackets++;
						if(brackets && keywords[i - st_pos_bef][0] == ']') brackets--;
					}
					
					(*outputp)[*pos] = '[';
					(*pos)++;
					typeToOutput(it_name);
					(*outputp)[*pos] = ']';
					(*pos)++;
					
					// Type comparison operator
					unsigned int st_pos2 = st_pos - 1;
					for(; keywords[i - st_pos2][0] == '>' || keywords[i - st_pos2][0] == '<' || keywords[i - st_pos2][0] == '=' || keywords[i - st_pos2][0] == '!' || keywords[i - st_pos2][0] == '&' || keywords[i - st_pos2][0] == '|'; st_pos2--) {
						typeToOutput(keywords[i - st_pos2]);
					}
					
					// Type expression after comparison operator
					for(; keywords[i - st_pos2][0] != '[' || brackets > 0; st_pos2--) {
						parseKey(keywords, i - st_pos2, keys, outputp, output_size, pos, specials, 1, cItem);
						
						if(keywords[i - st_pos2][0] == '[') brackets++;
						if(brackets && keywords[i - st_pos2][0] == ']') brackets--;
					}
					
					(*outputp)[*pos] = '[';
					(*pos)++;
					typeToOutput(it_name);
					typeToOutput("])){");
					
					typeToOutput(cond_bool);
					typeToOutput("=0;break;}else{");
					typeToOutput(cond_bool);
					typeToOutput("=1;}}");
					
					while(keywords[i - st_pos][0] != ';' && keywords[i - st_pos][0] != '{' && keywords[i - st_pos][0] != '}') {
						st_pos++;
					}
					st_pos--;
					
					// Type statement before comparison
					for(; st_pos > st_pos_bef_c; st_pos--) {
						parseKey(keywords, i - st_pos, keys, outputp, output_size, pos, specials, 0, cItem);
					}
					
					// Include comparison results
					typeToOutput(cond_bool);
					
					i += i_pos - 1;
					break;
				}
			} else if(keywords[i + i_pos][0] == '<' && keywords[i + i_pos + 1][0] == '<' && keywords[i + i_pos + 2][0] == '<') {
				break; // TMP, WIP
			}
		}
		
		if(!foundSublist) {
			typeToOutput(keywords[i]);
//			typeToOutput("_ppl");
		}
	} else {
		if(strcmp(keywords[i], "#include") == 0) { // TMP; makes it possible to include C functions without the need of 'import clib'
			INCR_MEM(2);
			
			if((*outputp)[*pos - 1] != '\n') {
				INCR_MEM(1);
				(*outputp)[*pos] = '\n';
				(*pos)++;
			}
			
			typeToOutput(keywords[i]);
			i++;
			
			(*outputp)[*pos] = ' ';
			(*pos)++;
			
			while(keywords[i][0] != ';') {
				typeToOutput(keywords[i]);
				i++;
			}
			
			(*outputp)[*pos] = '\n';
			(*pos)++;
		} else {
			typeToOutput(keywords[i]);
		}
		
		if(keywords[i][0] != '"' && keywords[i][0] != '\'' && !isNumber(keywords[i]) && !isReserved(types, keywords[i], 22) && !isReserved(reserved_keys, keywords[i], 19) && strstr(specials, keywords[i]) == NULL) {
//			typeToOutput("_ppl");
		}
		
		if(strstr(specials, keywords[i]) == NULL && strstr(specials, keywords[i + 1]) == NULL) {
			INCR_MEM(1);
			
			(*outputp)[*pos] = ' ';
			(*pos)++;
		}
	}
	
	return i;
}

char *parse(char **keywords, size_t keys, size_t *pos, char specials[]) {
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	for(size_t i = 0; i < keys; i++) {
		i = parseKey(keywords, i, keys, &output, &output_size, pos, specials, 0, NULL);
	}
	
	if(*pos + 1 > output_size) addSpaceForChars(&output, &output_size);
	output[*pos] = '\0';
	
	return output;
}
