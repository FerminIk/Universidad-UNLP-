package tp02.ejercicio04;

import tp02.ejercicio3.*;

public class TestBalanceo {

	public static char getOpuesto(char inicio) {
		switch (inicio) {
			case '(': return ')';
			case '[': return ']';
			case '{': return '}';
		}
		return '?';
	}
	
	public static boolean esApertura(char caracter) {
		switch (caracter) {
			case '(': return true;
			case '[': return true;
			case '{': return true;
		}
		return false;
	}
	
	public static boolean balanceado(String str) {
		PilaGenerica<Character> pila = new PilaGenerica<Character>();
		int aux = str.length();
		for (int i = 0;i<aux;i++) {
			char ch = new str.toCharArray();
		}
		
		return false;
	}
	
}
