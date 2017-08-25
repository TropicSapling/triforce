#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#include "def.h"

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

char *parse(char **keywords, size_t key, size_t *pos, char specials[]) {
	char ****types = {
		{{"array", "bool", "chan", "func", "list", "pointer", "var", "void"}, {}},
		{{"char", "int", "number"}, {"array", "list", "pointer"}},
		{{"const", "only"}, {"array", "char", "decimal", "int", "list", "number", "pointer", "signed", "string", "unsigned", "var"}},
		{{"decimal"}, {"number"}},
		{{"noscope"}, {"array", "char", "const", "decimal", "int", "list", "number", "only", "pointer", "signed", "string", "unsigned", "var"}},
		{{"signed", "unsigned"}, {"char", "int", "number", "string"}}
	};
	
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	for(size_t i = 0; i < key; i++) {
		if(keywords[i] != NULL) {
			if(*pos + 1 >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
				return NULL;
			}
			
			// WIP
			
			output[*pos] = ' ';
			(*pos)++;
		}
	}
	
	if(*pos > output_size && addSpaceForChars(&output, &output_size) == NULL) {
		return NULL;
	}
	
	output[*pos] = '\0';
	
	return output;
}