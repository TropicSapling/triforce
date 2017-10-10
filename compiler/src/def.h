#ifndef DEF_INCLUDED
#define DEF_INCLUDED

void preprocess(FILE **input, char **processed_input, size_t input_size, char *path[], char **exports, size_t *exports_size, size_t *ekey);

void lex_parse(char *input, char ***keywords, char ***pointers);

char *parse(char **keywords, char *filename);

void addSpaceForKeys(char ***keywords, size_t *keywords_size);

extern const char* const restrict specials;

extern size_t keywords_size;
extern size_t key;

extern size_t pointers_size;
extern size_t pkey;

extern size_t pos;

#endif