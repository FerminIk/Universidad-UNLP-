package ejercicio1;

import tp02.ejercicio1.*;
import java.util.Scanner;

public class TestListaDeEnterosConArreglos {

	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);
		ListaDeEnterosConArreglos enteros = new ListaDeEnterosConArreglos();
		int numero;
		do {
			System.out.println("Introduzca un numero: ");
			numero = s.nextInt();
			enteros.agregarFinal(numero);
		} while  (numero != 0);
		Recursivo r = new Recursivo();
		r.imprimirRecursivoArreglo(enteros,1);
	}

}
