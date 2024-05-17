package ejercicio1;

import java.util.Scanner;

import tp02.ejercicio1.*;

public class TestListaDeEnterosEnlazada {

	public static void main(String[] args) {
		ListaDeEnterosEnlazada L = new ListaDeEnterosEnlazada();
		Scanner s = new Scanner(System.in);
		int num;
		do  {
			System.out.println("Escriba un numero: ");
			num = s.nextInt();
			L.agregarFinal(num);
		} while (num != 0);
		Recursivo r = new Recursivo();
		r.imprimirRecursivoEnlazada(L, 1);
	}

}
