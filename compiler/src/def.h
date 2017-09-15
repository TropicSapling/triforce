#ifndef DEF_INCLUDED
#define DEF_INCLUDED

int preprocess(FILE *input, char **processed_input, size_t input_size, char specials[]);

int lex_parse(FILE *input, char ***keywords, size_t keywords_size, size_t *key, char ***pointers, size_t pointers_size, size_t *pkey, char specials[]);

char *parse(char **keywords, size_t key, size_t *pos, char specials[]);

#endif