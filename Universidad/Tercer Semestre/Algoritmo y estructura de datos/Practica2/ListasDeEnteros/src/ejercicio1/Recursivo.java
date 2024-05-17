package ejercicio1;

import tp02.ejercicio1.*;

public class Recursivo {

	public void imprimirRecursivoArreglo(ListaDeEnterosConArreglos l, int pos) {
		if (l.elemento(pos) == 0){
			System.out.println("Numero: "+l.elemento(pos));
		} else {
			imprimirRecursivoArreglo(l,pos+1);
			System.out.println("Numero: "+l.elemento(pos));
		}
	}
	
	public void imprimirRecursivoEnlazada(ListaDeEnterosEnlazada l, int pos) {
		if (l.elemento(pos) == 0){
			System.out.println("Numero: "+l.elemento(pos));
		} else {
			imprimirRecursivoEnlazada(l,pos+1);
			System.out.println("Numero: "+l.elemento(pos));
		}
	}
}
