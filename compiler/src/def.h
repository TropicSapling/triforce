#ifndef DEF_INCLUDED
#define DEF_INCLUDED

void preprocess(FILE **input, char **processed_input, size_t input_size, char *path[], char **exports, size_t *exports_size, size_t *ekey);

void lex_parse(char *input, char ***keywords, size_t keywords_size, size_t *key, char ***pointers, size_t pointers_size, size_t *pkey, char specials[]);

char *parse(char **keywords, size_t keys, size_t *pos, char specials[]);

void *addSpaceForKeys(char ***keywords, size_t *keywords_size);

#endif