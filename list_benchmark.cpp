static void ListAccess(benchmark::State& state) {
  char *arrc[257];
  char arr[768];
  arrc[0] = arr;
  for(int i = 0; i < 256; i++) {
    if(i % 2) {
      *(unsigned short*) arrc[i] = 65535;
      arrc[i + 1] = arrc[i] + 2;
    } else {
      *(unsigned int*) arrc[i] = 4294967295;
      arrc[i + 1] = arrc[i] + 4;
    }
  }

  // Code inside this loop is measured repeatedly
  for (auto _ : state) {
    unsigned short s = 0;
    unsigned int integer = 0;

    for(int i = 0; i < 256; i++) {
      if(arrc[i + 1] - arrc[i] == 2) {
        s += *(unsigned short*) arrc[i];
      } else {
        integer += *(unsigned int*) arrc[i];
      }
    }

    // Make sure the variable is not optimized away by compiler
    benchmark::DoNotOptimize(s);
    benchmark::DoNotOptimize(integer);
  }
}

// Register the function as a benchmark
BENCHMARK(ListAccess);

static void ListAccess2(benchmark::State& state) {
  char arr[1024];
  for(int i = 0; i < 1022; i++) {
    if(i % 2) {
      arr[i] = 2;
      i++;
      *((unsigned short*) (arr + i)) = 65535;
      i++;
    } else {
      arr[i] = 4;
      i++;
      *((unsigned int*) (arr + i)) = 4294967295;
      i += 3;
    }
  }

  // Code inside this loop is measured repeatedly
  for (auto _ : state) {
    unsigned short s = 0;
    unsigned int integer = 0;

    for(int i = 0; i < 1022; i++) {
      if(arr[i] == 2) {
        i++;
        s += *((unsigned short*) (arr + i));
      } else {
        i++;
        integer += *((unsigned int*) (arr + i));
        i += 2;
      }
    }

    // Make sure the variable is not optimized away by compiler
    benchmark::DoNotOptimize(s);
    benchmark::DoNotOptimize(integer);
  }
}

// Register the function as a benchmark
BENCHMARK(ListAccess2);

static void ArrayAccess(benchmark::State& state) {
  // Code before the loop is not measured
  int arr[256] = {1};
  for (auto _ : state) {
    unsigned int integer = 0;

    for(int i = 0; i < 256; i++) {
      integer += arr[i];
    }

    // Make sure the variable is not optimized away by compiler
    benchmark::DoNotOptimize(integer);
  }
}

BENCHMARK(ArrayAccess);