#ifndef DEF_INCLUDED
	#define DEF_INCLUDED

	extern const char* const restrict specials;

	extern size_t keywords_size;
	extern size_t key;

	extern size_t pointers_size;
	extern size_t pkey;

	extern size_t pos;

	void preprocess(FILE **input, char **processed_input, size_t input_size, char *path[static 2], char **exports, size_t *exports_size, size_t *ekey, char defs[128][2][128], size_t *defID);

	void lex_parse(char *input, char ***keywords, char ***pointers);

	char *parse(char **keywords, char *filename);
#endif