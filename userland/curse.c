#include <conio.h>
#include <stdio.h>
#include <stdlib.h>

#define clear() printf("\033[H\033[J")
#define gotoxy(x,y) printf("\033[%d;%dH", (x), (y))

int main() {
    *filename = "abc.txt";
	FILE *ft;
	ft = fopen(filename, "w+");
	
	int x = 1;
	int y = 1;
	int letter;
	int curse = 1;

	while ( curse == 1 ) {
		scancode = getch();
		switch (scancode) {
			case 97:
				letter = 1;
				printf("a"); 
				break;
			case 98: 
				letter = 2; 
				printf("b"); 
				break;
			case 99:  
				letter = 3; 
				printf("c"); 
				break;
			case 100: 
				letter = 4; 
				printf("d"); 
				break;
			case 101: 
				letter = 5; 
			    printf("e"); 
				break;
			case 102: 
				letter = 6; 
				printf("f"); 
				break;
			case 103: 
				letter = 7; 
				printf("g"); 
				break;
			case 104:
			   	letter = 8;
			   	printf("h");
			   	break;
			case 105:
			   	letter = 9;
			   	printf("i");
			   	break;
			case 106:
			   	letter = 10;
			   	printf("j"); 
				break;
			case 107:
			   	letter = 11;
			   	printf("k");
			   	break;
			case 108:
			   	letter = 12;
			   	printf("l");
			   	break;
			case 109:
			   	letter = 13;
			   	printf("m");
			   	break;
			case 110:
			   	letter = 14; 
				printf("n");
			   	break;
			case 111:
			   	letter = 15;
			   	printf("o");
			   	break;
			case 112:
			   	letter = 16;
			   	printf("p");
			   	break;
			case 113:
			   	letter = 17; 
				printf("q");
			   	break;
			case 114:
			   	letter = 18;
			   	printf("r");
			   	break;
			case 115:
			   	letter = 19;
			   	printf("s");
			   	break;
			case 116:
			   	letter = 20;
			   	printf("t");
			   	break;
			case 117: 
				letter = 21;
			   	printf("u");
			   	break;
			case 118:
			   	letter = 22;
			   	printf("v");
			   	break;
			case 119: 
				letter = 23;
			   	printf("w");
			   	break;
			case 120:
			   	letter = 24;
			   	printf("x"); 
				break;
			case 121:
			   	letter = 25;
			   	printf("y");
			   	break;
			case 122:
			   	letter = 26;
			   	printf("z");
			   	break;

			case 65: 
				letter = 27;
			   	printf("A"); 
				break;
			case 66: 
				letter = 28;
			   	printf("B");
			   	break;
			case 67: 
				letter = 29;
			   	printf("C"); 
				break;
			case 68: 
				letter = 30;
			   	printf("D");
			   	break;
			case 69: 
				letter = 31;
			   	printf("E"); 
				break;
			case 70: 
				letter = 32;
			   	printf("F"); 
				break;
			case 71: 
				letter = 33;
			   	printf("G"); 
				break;
			case 72:
			   	letter = 34;
			   	printf("H");
			   	break;
			case 73:
			   	letter = 35;
			   	printf("I");
			   	break;
			case 74:
			   	letter = 36;
			   	printf("J");
			   	break;
			case 75:
			   	letter = 37;
			   	printf("K");
			   	break;
			case 76:
			    letter = 38;
			   	printf("L");
			   	break;
			case 77:
			   	letter = 39;
			   	printf("M");
			   	break;
			case 78:
			   	letter = 40;
			   	printf("N");
			   	break;
			case 79:
			   	letter = 41;
			   	printf("O");
			   	break;
			case 80:
			   	letter = 42;
			   	printf("P");
			   	break;
			case 81:
			   	letter = 43;
			   	printf("Q");
			   	break;
			case 82:
			   	letter = 44;
			   	printf("R");
			   	break;
			case 83:
			   	letter = 45;
			   	printf("S");
			   	break;
			case 84:
			   	letter = 46;
			   	printf("T");
			   	break;
			case 85:
			   	letter = 47;
			   	printf("U");
			   	break;
			case 86:
			   	letter = 48;
			   	printf("V");
			   	break;
			case 87:
			   	letter = 49;
			   	printf("W");
			   	break;
			case 88:
			   	letter = 50;
			   	printf("X");
			   	break;
			case 89:
			   	letter = 51;
			   	printf("Y");
			   	break;
			case 90:
			   	letter = 52;
			   	printf("Z");
			   	break;

			case 48: 
				letter = 100;
			   	printf("0");
			   	break;
			case 49:
			   	letter = 101;
			   	printf("1");
			   	break;
			case 50:
			   	letter = 102;
			   	printf("2"); 
				break;
			case 51:
			   	letter = 103;
			   	printf("3");
			   	break;
			case 52:
				letter = 104;
			   	printf("4");
			   	break;
			case 53: 
				letter = 105;
			   	printf("5"); 
				break;
			case 54: 
				letter = 106;
			   	printf("6");
			   	break;
			case 55:
				letter = 107; 
				printf("7");
				break;
			case 56:
				letter = 108;
				printf("8"); 
				break;
			case 57: 
				letter = 109;
			   	printf("9");
			   	break;
		}

		if letter = "ENTER" {
			y = y + 1;
			x = 1;
		} else if letter == "BACKSPACE" {
			if x == 1 {
				y = y - 1;
			} else {
				x = x - 1;
				gotoxy(x, y);
				printf(" ");
			}
		} else {
			x = x + 1;
		}

		gotoxy(x, y);		
	}
}
