#ifndef DEF_INCLUDED
	#define DEF_INCLUDED

	extern const char pointer const unique specials;

	extern size_t keywords_size;
	extern size_t key;

	extern size_t pointers_size;
	extern size_t pkey;

	extern size_t pos;

	void preprocess(FILE pointer*2 input, char pointer*2 processed_input, size_t input_size, char pointer path[static 2], char pointer*2 exports, size_t pointer exports_size, size_t pointer ekey, char defs[128][2][128], size_t pointer defID);

	void lex_parse(char pointer input, char pointer*3 keywords, char pointer*3 pointers);

	char pointer parse(char pointer*2 keywords, char pointer filename);
#endif