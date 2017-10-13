#ifndef DEF_INCLUDED
	#define DEF_INCLUDED

	extern const char pointer const unique specials;

	extern size_t keywords_size;
	extern size_t key;

	extern size_t pointers_size;
	extern size_t pkey;

	extern size_t pos;
	
	struct ErrInfo {
		char pointer msg;
		
		char pointer filename;
		size_t lineno;
		size_t linecol;
		
		char pointer data;
		char pointer*2 data2;
		unsigned int i;
	};
	
	void printErr(struct ErrInfo Error);
	void printWarning(struct ErrInfo Error);

	void preprocess(FILE pointer*2 input, char pointer*2 processed_input, size_t input_size, char pointer path[static 2], char pointer*2 exports, size_t pointer exports_size, size_t pointer ekey, char defs[128][2][128], size_t pointer defID);

	void lex_parse(char pointer input, char pointer*3 keywords, char pointer*3 pointers);

	char pointer parse(char pointer*2 keywords, char pointer filename);
	
	#define RED   "\x1B[31m"
	#define GREEN   "\x1B[32m"
	#define YELLOW   "\x1B[33m"
	#define BLUE   "\x1B[34m"
	#define WHITE   "\x1B[37;1m"
	#define RESET "\x1B[0m"
#endif