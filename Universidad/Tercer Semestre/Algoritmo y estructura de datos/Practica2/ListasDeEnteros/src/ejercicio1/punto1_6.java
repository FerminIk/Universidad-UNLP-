package ejercicio1;

import java.util.Scanner;
import tp02.ejercicio1.ListaDeEnterosEnlazada;
public class punto1_6 {

	public static void main(String[] args) {
		Scanner s = new Scanner(System.in);
		int num = s.nextInt();
		TresX t = new TresX(num);
		ListaDeEnterosEnlazada l = new ListaDeEnterosEnlazada();
		l = t.calcularSucesion(num);
		int i = 0;
		if (!l.esVacia()) {
			do {
				i++;
				System.out.println("Numero: "+ l.elemento(i));
			} while (l.elemento(i) != 1 );
		}
	}

}
