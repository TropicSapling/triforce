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

char *parse(char **keywords, size_t key) {
	size_t output_size = 256;
	char *output = malloc(output_size);
	
	size_t pos = 0;
	for(size_t i = 0; i < key; i++) {
		if(pos > output_size && addSpaceForChars(&output, &output_size) == NULL) {
			return NULL;
		}
		
		output[pos] = 't'; // DEBUG; will be removed in the future
		
		pos += 1;
	}
	
	if(pos > output_size && addSpaceForChars(&output, &output_size) == NULL) {
		return NULL;
	}
	
	output[pos] = '\0';
	
	puts(output);
	
	return output;
}