#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

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
/*	char types[][2][12][8] = {
		{{"array", "bool", "chan", "func", "list", "pointer", "var", "void"}, {}},
		{{"char", "int", "number"}, {"array", "list", "pointer"}},
		{{"const", "only"}, {"array", "char", "fraction", "int", "list", "number", "pointer", "signed", "unsigned", "var"}},
		{{"fraction"}, {"number"}},
		{{"noscope"}, {"array", "char", "const", "fraction", "int", "list", "number", "only", "pointer", "signed", "unsigned", "var"}},
		{{"signed", "unsigned"}, {"char", "int", "number"}}
	}; */
	
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	for(size_t i = 0; i < key; i++) {
		if(keywords[i] != NULL) {
			if(strcmp(keywords[i], "@") == 0) {
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
				if(*pos + strlen(keywords[i]) >= output_size && addSpaceForChars(&output, &output_size) == NULL) {
					return NULL;
				}
				
				for(unsigned int c = 0; keywords[i][c] != '\0'; c++) {
					output[*pos] = keywords[i][c];
					(*pos)++;
				}
				
				output[*pos] = '"';
				(*pos)++;
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