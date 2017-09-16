#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <stdbool.h>

#include "def.h"

#define INCR_MEM(size) do { \
	if(*key + (size) > keywords_size / sizeof(char*) && addSpaceForKeys(keywords, &keywords_size) == NULL) { \
		return 1; \
	} \
} while(0)

#define INCR_MEM2(size) do { \
	if(*pkey + (size) > pointers_size / sizeof(char*) && addSpaceForKeys(pointers, &pointers_size) == NULL) { \
		return 1; \
	} \
} while(0)

bool inStr = false;
bool inStr2 = false;
bool escaping = false;
bool ignoring = false;

char *addSpaceForKeys(char ***keywords, size_t *keywords_size) {
	*keywords_size *= 2;
	
	char *res = realloc(*keywords, *keywords_size);
	if(res == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
	} else {
		*keywords = (char**) res;
	}
	
	return res;
}

bool isSpecial(char c, char specials[]) {
	for(unsigned int i = 0; specials[i] != '\0'; i++) {
		if(c == specials[i]) return true;
	}
	
	return false;
}

int lex_parse(char *input, char ***keywords, size_t keywords_size, size_t *key, char ***pointers, size_t pointers_size, size_t *pkey, char specials[]) {
	char *org_input = input;
	
	INCR_MEM(1);
	(*keywords)[*key] = input;
	(*key)++;
	
	while(*input != '\0') {
		char *special;
		bool foundSpecial = false;
		
		if(ignoring) {
			*input = '\0';
			input++;
		}
		
		while((ignoring || inStr || inStr2 || *input != ' ') && *input != '\0') {
			if(ignoring) {
				if(*input == '*' && *(input + 1) == '/') {
					ignoring = false;
					input++;
					
					INCR_MEM(1);
					
					(*keywords)[*key] = input + 1;
					(*key)++;
				}
			} else if(!inStr && !inStr2 && isSpecial(*input, specials)) {
				special = calloc(2, 1);
				special[0] = *input;
				foundSpecial = true;
				
				break;
			} else if(escaping) {
				escaping = false;
			} else if(!inStr2 && *input == '\'') {
				if(inStr) {
					inStr = false;
					break;
				} else {
					inStr = true;
				}
			} else if(!inStr && *input == '"') {
				if(inStr2) {
					inStr2 = false;
					break;
				} else {
					inStr2 = true;
				}
			} else if(*input == '\\') {
				escaping = true;
			}
			
			input++;
		}
		
		if(*input == specials[0] || *input == specials[4] || *input == specials[5]) {
			*input = '\0';
			input++;
			
			while(*input == ' ') input++;
			if(*input == '/' && *(input + 1) == '/') while(*input != '\n') input++;
			
			if(*input == '\r') {
				input++;
			} else if(*input == '/' && *(input + 1) == '*') {
				ignoring = true;
				input++;
				
				continue;
			} else if(*input != '\n') {
				input--;
			}
		}
		
		if(input == org_input || *(input - 1) != '\0') {
			*input = '\0';
		}
		
		input++;
		
		if(foundSpecial) {
			INCR_MEM(1);
			INCR_MEM2(1);
			
			(*pointers)[*pkey] = special; // This is used to mark where memory was allocated for 'special'
			(*pkey)++;
			
			(*keywords)[*key] = special;
			(*key)++;
		}
		
		while(*input == ' ') input++;
		
		if(!isSpecial(*input, specials)) {
			INCR_MEM(1);
			
			(*keywords)[*key] = input;
			(*key)++;
		}
	}
	
	return 0;
}