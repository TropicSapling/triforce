cout "Amount of arguments specified: " + __argc;
cout "Arguments specified: " + split(__args, " ");
cout "Line number: " + __line;

byte[] str = "something";

pointer ptr -> str;
byte[] adress = ->str;

cout "String: " + str; // something
cout "Pointer/Reference: " + ptr;
cout "Adress: " + adress;
cout "Pointer value: " + @ptr;

void function {
	cout "This function returns nothing.";
}

unsigned byte function2(byte par1) {
	return par1;
}

unsigned byte[] function3 return [-127, 0, 255];

function(); // This function returns nothing.
cout function2(123); // 123
cout function3(); // [-127, 0, 255]

byte[] array = [1, 65, 3, -34, -24, 121];
cout array[1>>4]; // [65, 3, -34]
cout array[4>>1]; // [65, 3, -34]
cout array[1<<4]; // [-34, 3, 65]
cout array[4<<1]; // [-34, 3, 65]

unsigned short[] array2 = [16232, 9843, 2121, 5459];
cout rearrange(array2, 2); // [16, 3, 9843, 1, 15459]

byte[] string = "Hello world!";
cout rearrange(string, ' '); // ["Hello", "world!"]

byte[] string2 = ['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'];
cout rearrange(string2, ' '); // ["Hello", "world!"]

byte[] ascii_str = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
cout rearrange(ascii_str, ' '); // ["Hello", "world!"]
cout rearrange(ascii_str, 32); // [[72, 101, 108, 108, 111], [119, 111, 114, 108, 100, 33]]

custom[] custom_arr = [2781, 'g', ["some", variable: "thing"], someFunction: function2, -93478];
cout custom_arr[1]; // 'g'
cout custom_arr[1<<4] + [1, '2', 3]; // [someFunction: function2, ["some", variable: "thing"], 'g', 1, '2', 3]

cout 1 == 0 ? "Impossible!\n" : "True\n";cout 0 == 1 ? "Also impossible ofc\n" : "Yup.\n";

return 0 from __app;