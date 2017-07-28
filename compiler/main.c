#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

void println(char str[]) {
	printf("%s\n", str);
}

int main(int argc, char *argv[]) {
	if(argc != 2) {
		println("Invalid usage. Please specify a file as the first and only argument.");
		return 1;
	}
	
	FILE *input = fopen(argv[1], "r"); // Will be "r+" if automatic compiled GC gets added in the future
	
	if(input == NULL) {
		perror("ERROR");
		fprintf(stderr, "ID: %d\n", errno);
		return 1;
	}
	
	FILE *output;
	
	if(argc < 3) {
		short file_length = strlen(argv[1]);
		char filename[file_length];
		strcpy(filename, argv[1]);
		
		char c = file_length - 1;
		while(filename[c] != '.') {
			c--;
		}
		
		memset(&filename[c + 1], 'c', 1);
		memset(&filename[c + 2], '\0', 1);
		
		while(filename[c] != '/' && c > 0) {
			c--;
		}
		
		char finalname[strlen(filename) + 4];
		char success;
		
		if(c == 0) {
			success = mkdir("bin", 0777);
			
			strcpy(finalname, "bin/");
			strcat(finalname, filename);
		} else {
			strncpy(finalname, filename, c + 1);
			finalname[c + 1] = '\0';
			strcat(finalname, "bin/");
			success = mkdir(finalname, 0777);
			strcat(finalname, &filename[c + 1]);
		}
		
		if(success != 0 && errno != 17) {
			perror("ERROR");
			fprintf(stderr, "ID: %d\n", errno);
			return 1;
		}
		
		output = fopen(finalname, "w");
	} else {
		output = fopen(argv[2], "w");
	}
	
	char buf[128];
	float i = 0.0;
	
	while(fgets(buf, 128, input) != NULL) {
		char *cout_pos = strstr(buf, "cout");
		
		if(cout_pos != NULL) {
			fprintf(output, "printf(\"%%s\", %s);", cout_pos + 5);
		}
		
		printf("Compiling... %.2f%%\r", (i / 500) * 100); // Made up progress for now
		fflush(stdout);
		i++;
	}
	
	fclose(input);
	fclose(output);
	
	println("Compiling... 100.00%");
	
	return 0;
}