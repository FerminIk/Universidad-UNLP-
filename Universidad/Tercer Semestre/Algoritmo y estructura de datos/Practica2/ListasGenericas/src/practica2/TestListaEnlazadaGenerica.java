package practica2;

import java.util.Scanner;
import tp02.ejercicio2.*;

public class TestListaEnlazadaGenerica {

	public static void main(String[] args) {
		Scanner s= new Scanner(System.in);
		ListaEnlazadaGenerica<Alumno> lista = new ListaEnlazadaGenerica<Alumno>();
		Alumno[] a = new Alumno[3];
		
		for (int i=0;i<3;i++) {
			System.out.println("Coloque nombre, apellido, comision, email y direccion");
			a[i] =  new Alumno(s.nextLine(),s.nextLine(),s.nextLine(),s.nextLine(),i);
		}
		
		lista.agregar(a);
	
		for (int i=1;i<4;i++) {
			System.out.println(lista.elemento(i).tusDatos());
		}
	}

}
