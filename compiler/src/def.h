#ifndef DEF_INCLUDED
#define DEF_INCLUDED

int preprocess(FILE **input, char **processed_input, size_t input_size, char specials[], char *path, char **exports, size_t *exports_size, size_t *ekey);

int lex_parse(char *input, char ***keywords, size_t keywords_size, size_t *key, char ***pointers, size_t pointers_size, size_t *pkey, char specials[]);

char *parse(char **keywords, size_t key, size_t *pos, char specials[]);

char *addSpaceForKeys(char ***keywords, size_t *keywords_size);

#endif