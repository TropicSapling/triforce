#ifndef DEF_INCLUDED
#define DEF_INCLUDED

int lex_parse(FILE *input, char ***keywords, size_t keywords_size, size_t *key, size_t file_size);

char *parse(char **keywords, size_t key, size_t *pos);

#endif