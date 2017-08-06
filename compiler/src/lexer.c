#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#include "def.h"

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

int lex_parse(FILE *input, char ***keywords, size_t keywords_size, size_t *key, size_t file_size) {
	char buf[65536];
	char extra_buf[16] = "\0";
	char *c;
	
	long double progress = 0.0;
	
	while(fgets(buf, 65536, input) != NULL) {
		if(strcmp(buf, "\n") == 0 || strcmp(buf, "\r\n") == 0) {
			continue;
		}
		
		size_t new_size = strlen(buf) + 1;
		if(extra_buf != NULL) {
			new_size += strlen(extra_buf);
		}
		
		char *tmp = malloc(new_size);
		if(tmp == NULL) {
			perror("ERROR");
			fprintf(stderr, "ID: %d\n", errno);
		} else {
			c = tmp;
		}
		
		if(extra_buf[0] != '\0') {
			strcpy(c, extra_buf);
			strcat(c, buf);
			
			extra_buf[0] = '\0';
		} else {
			strcpy(c, buf);
		}
		
		if(*key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(keywords, &keywords_size) == NULL) {
			return 1;
		}
		
		(*keywords)[*key] = NULL; // This is used to mark where memory was allocated for 'c'
		(*key)++;
		
		if(*key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(keywords, &keywords_size) == NULL) {
			return 1;
		}
		
		(*keywords)[*key] = c;
		(*key)++;
		
		size_t row_len = 0;
		
		while(row_len < 65521) {
			char *special = calloc(2, 1);
			
			while(*c != ' ' && *c != '\0') {
				c++;
				row_len++;
				
				if(*c == ';' || *c == ',' || *c == '[' || *c == ']' || *c == '{' || *c == '}' || *c == '(' || *c == ')' || *c == '?' || *c == '>' || *c == '<' || *c == '=' || *c == '+' || *c == '-' || *c == '*' || *c == '/' || *c == '%' || *c == '!' || *c == '&' || *c == '|' || *c == '^' || *c == '~' || *c == '\\') {
					special[0] = *c;
					break;
				}
			}
			
			if(*c == '\0') {
				c++;
				break;
			}
			
			*c = '\0';
			
			c++;
			row_len++;
			
			if(row_len < 65521) {
				if(special[0] != '\0') {
					if(*key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(keywords, &keywords_size) == NULL) {
						return 1;
					}
					
					(*keywords)[*key] = special;
					(*key)++;
				}
				
				if(*key > keywords_size / (sizeof(char*) + 1) && addSpaceForKeys(keywords, &keywords_size) == NULL) {
					return 1;
				}
				
				(*keywords)[*key] = c;
				(*key)++;
			}
		}
		
		if(row_len > 65520) {
			strcpy(extra_buf, c);
		}
		
		progress += row_len;
		printf("Reading file... %.2Lf%%\r", (progress / file_size) * 100);
		fflush(stdout);
	}
	
	
	return 0;
}